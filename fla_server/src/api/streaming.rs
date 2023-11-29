//! Streaming handler
use std::{collections::HashMap, sync::Arc};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use fla_common::{
    streaming::{
        DataError, ErrorType, FromServerStreamingMessage, StreamingData, StreamingFields,
        ToServerStreamingMessage,
    },
    types::VehicleGuid,
};
use futures::{stream::FuturesUnordered, StreamExt};
use thiserror::Error;
use tokio::{select, sync::broadcast};
use tracing::{debug, error};

use crate::{
    tokens::{self, validate_access_token},
    types::Vehicle,
    Config,
};

/// Retrieve router for Tesla streaming API
///
pub fn router(config: &Config) -> Router {
    Router::new()
        .route("/streaming/", get(ws_handler))
        .with_state(config.clone())
}

fn deserialize_field_names(str: &str) -> Vec<StreamingFields> {
    str.split(',')
        .filter_map(|x| match x.parse() {
            Ok(field) => Some(field),
            Err(_) => None,
        })
        .collect()
}

fn serialize_fields(fields: &[StreamingFields], data: &StreamingData) -> String {
    let mut result = Vec::new();
    result.push(data.time.to_string());

    for field in fields {
        match field {
            StreamingFields::Speed => push_data(&mut result, data.speed),
            StreamingFields::Odometer => push_data(&mut result, data.odometer),
            StreamingFields::Soc => push_data(&mut result, data.soc),
            StreamingFields::Elevation => push_data(&mut result, data.elevation),
            StreamingFields::EstHeading => push_data(&mut result, data.est_heading),
            StreamingFields::EstLat => push_data(&mut result, data.est_lat),
            StreamingFields::EstLng => push_data(&mut result, data.est_lng),
            StreamingFields::Power => push_data(&mut result, data.power),
            StreamingFields::ShiftState => push_data(&mut result, data.shift_state.clone()),
            StreamingFields::Range => push_data(&mut result, data.range),
            StreamingFields::EstRange => push_data(&mut result, data.est_range),
            StreamingFields::Heading => push_data(&mut result, data.heading),
        }
    }
    result.join(",")
}

fn push_data<T: ToString>(result: &mut Vec<String>, data: Option<T>) {
    result.push(data.map(|x| T::to_string(&x)).unwrap_or_default())
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
#[allow(clippy::unused_async)]
pub async fn ws_handler(
    State(config): State<Arc<tokens::Config>>,
    State(vehicles): State<Arc<Vec<Vehicle>>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(|socket| handle_socket(socket, config, vehicles))
}

#[derive(Error, Debug)]
enum SocketError {
    #[error("{0}")]
    ReportableError(#[from] DataError),

    #[error("{0}")]
    NotReportableError(String),
}

/// Actual websocket state machine (one will be spawned per connection)
async fn handle_socket(
    mut socket: WebSocket,
    config: Arc<tokens::Config>,
    vehicles: Arc<Vec<Vehicle>>,
) {
    match handle_socket_internal(&mut socket, config, vehicles).await {
        Err(SocketError::ReportableError(err)) => {
            error!("Reportable error: {err}");
            send_error(&mut socket, err).await;
            // _ = socket.close().await;
        }

        Err(SocketError::NotReportableError(err)) => {
            error!("Not reportable error: {err}");
            // _ = socket.close().await;
        }

        Ok(()) => {
            _ = socket.close().await;
        }
    }
}

struct Subscription {
    vehicle_id: VehicleGuid,
    fields: Arc<Vec<StreamingFields>>,
    rx: broadcast::Receiver<Arc<StreamingData>>,
}

async fn handle_socket_internal(
    socket: &mut WebSocket,
    config: Arc<tokens::Config>,
    vehicles: Arc<Vec<Vehicle>>,
) -> Result<(), SocketError> {
    // Say hello to the client. Pretend to be polite. The client will never guess the truth.
    let hello = FromServerStreamingMessage::ControlHello {
        connection_timeout: 30000,
    };
    send_message(socket, hello).await.map_err(|err| {
        error!("Could not send hello: {err:?}");
        SocketError::NotReportableError("Could not send hello".to_string())
    })?;

    let mut subscriptions: HashMap<VehicleGuid, Subscription> = HashMap::new();

    // Wait for data, either from simulator or from client.
    loop {
        let delete_subscription;
        let add_subscription;

        {
            let mut futures = {
                let futures = FuturesUnordered::new();
                for (id, s) in subscriptions.iter_mut() {
                    futures.push(async { (*id, s.fields.clone(), s.rx.recv().await) });
                }
                futures
            };

            (delete_subscription, add_subscription) = select! {
                // We got Data from the simulator.
                Some((vehicle_id, fields, data)) = futures.next() => {
                    match data {
                        Ok(data) => {
                            let value = serialize_fields(&fields, &data);
                            let msg = FromServerStreamingMessage::data_update(vehicle_id, value );

                            debug!("Sending: {msg:?}");
                            send_message(socket, msg).await.map_err(|err| {
                                let error = format!("Could not send message: {err:?}");
                                // If we could not send the message, the client is probably gone.
                                // We should probably make funeral arrangements.
                                // But no point trying to tell the client about it.
                                SocketError::NotReportableError(error)
                            })?;

                            (None, None)
                        }
                        Err(_err) => {
                            let error = DataError::disconnected(vehicle_id);
                            send_error(socket, error).await;
                            (Some(vehicle_id), None)
                        }
                    }
                }

                // We got a message from the client.
                // We don't expect any messages from the client.
                // The client still thinks we are friends.
                msg = socket.recv() => {
                    let text = match msg {
                        Some(Ok(Message::Close(_))) => {
                            debug!("Client disconnected");
                            break;
                        }
                        Some(Ok(Message::Text(text))) => Some(text),
                            Some(Ok(Message::Binary(binary))) => match String::from_utf8(binary) {
                                Ok(text) => Some(text),
                                Err(err) => {
                                    error!("Could not parse message: {err}");
                                    let error = format!("Could not parse message: {err}");
                                    return Err(SocketError::NotReportableError(error));
                                }
                            },

                        Some(Ok(Message::Ping(_))) => {
                            debug!("Ping");
                            None
                        }

                        Some(Ok(Message::Pong(_))) => {
                            debug!("Pong");
                            None
                        }

                        Some(Err(err)) => {
                            debug!("Error receiving message: {err}");
                            let error = format!("Error receiving message: {err}");
                            return Err(SocketError::NotReportableError(error));
                        }
                        None =>  {
                            debug!("Simulator disconnected");
                            break;
                        }
                    };

                    if let Some(text) = text {
                        debug!("Received: {text}");
                        process_client_message(text, &config, &vehicles).await?
                    } else  { (None, None)
                    }
                }
            }
        }

        if let Some(vehicle_id) = delete_subscription {
            subscriptions.remove(&vehicle_id);
        }

        if let Some(subscription) = add_subscription {
            subscriptions.insert(subscription.vehicle_id, subscription);
        }
    }

    Ok(())
}

async fn send_message(
    socket: &mut WebSocket,
    message: FromServerStreamingMessage,
) -> Result<(), ()> {
    let Ok(text) = serde_json::to_string(&message) else {
        error!("Could not serialize message!");
        return Err(());
    };

    let binary = String::as_bytes(&text).to_vec();
    if socket.send(Message::Binary(binary)).await.is_ok() {
        Ok(())
    } else {
        error!("Could not send a message!");
        Err(())
    }
}

async fn send_error(socket: &mut WebSocket, error: DataError) {
    let msg = FromServerStreamingMessage::DataError(error);
    _ = send_message(socket, msg).await;
}

async fn process_client_message(
    text: String,
    config: &tokens::Config,
    vehicles: &[Vehicle],
) -> Result<(Option<VehicleGuid>, Option<Subscription>), SocketError> {
    // // Parse the subscription message.
    let message = serde_json::from_str::<ToServerStreamingMessage>(&text).map_err(|err| {
        error!("Could not parse subscription message: {err}");
        let error = "Could not parse subscription message".to_string();
        SocketError::NotReportableError(error)
    })?;

    match message {
        ToServerStreamingMessage::DataSubscribeOauth { token, value, tag } => {
            let claims = validate_access_token(&token, config).map_err(|err| {
                error!("Invalid token: {err}");
                let error = DataError::new(&tag, ErrorType::ClientError, "Invalid token");
                SocketError::ReportableError(error)
            })?;

            // Validate the claims
            if !claims
                .scopes
                .contains(&tokens::ScopeEnum::VehicleDeviceData)
            {
                let error = DataError::new(&tag, ErrorType::ClientError, "Invalid scope");
                return Err(SocketError::ReportableError(error));
            }

            // The vehicle_id is the tag
            let vehicle_id: VehicleGuid = tag.clone().parse().map_err(|err| {
                error!("Vehicle id is not an integer: {err}");
                let error = DataError::new(&tag, ErrorType::ClientError, "Invalid vehicle id");
                SocketError::ReportableError(error)
            })?;

            // Find the vehicle
            let maybe_vehicle = vehicles.iter().find(|v| v.vehicle_id == vehicle_id);
            let vehicle = match maybe_vehicle {
                Some(vehicle) => vehicle,
                None => {
                    error!("Vehicle id not found: {vehicle_id:?}");
                    let error = DataError::new(&tag, ErrorType::ClientError, "Invalid vehicle id");
                    return Err(SocketError::ReportableError(error));
                }
            };

            // Deserialize the incoming data
            let fields = Arc::new(deserialize_field_names(&value));

            // Subscribe to the vehicle
            let rx = vehicle.command.subscribe().await?;

            let add = Subscription {
                vehicle_id,
                fields,
                rx,
            };
            Ok((None, Some(add)))
        }
        ToServerStreamingMessage::DataUnsubscribe { tag } => {
            let vehicle_id: VehicleGuid = tag.clone().parse().map_err(|err| {
                error!("Vehicle id is not an integer: {err}");
                let error = DataError::new(&tag, ErrorType::ClientError, "Invalid vehicle id");
                SocketError::ReportableError(error)
            })?;

            Ok((Some(vehicle_id), None))
        }
    }
}
