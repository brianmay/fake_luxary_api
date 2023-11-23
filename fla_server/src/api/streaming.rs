//! Streaming handler
use std::sync::Arc;

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
use thiserror::Error;
use tokio::select;
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
            _ = socket.close().await;
        }

        Err(SocketError::NotReportableError(err)) => {
            error!("Not reportable error: {err}");
            _ = socket.close().await;
        }

        Ok(()) => {
            _ = socket.close().await;
        }
    }
}

async fn handle_socket_internal(
    socket: &mut WebSocket,
    config: Arc<tokens::Config>,
    vehicles: Arc<Vec<Vehicle>>,
) -> Result<(), SocketError> {
    // Receive the subscription message.
    let msg = match socket.recv().await {
        Some(Ok(Message::Text(text))) => text,
        Some(Ok(Message::Binary(binary))) => match String::from_utf8(binary) {
            Ok(text) => text,
            Err(err) => {
                let error = format!("Could not parse message: {err}");
                return Err(SocketError::NotReportableError(error));
            }
        },
        Some(Ok(msg)) => {
            let error = format!("Unexpected message: {msg:?}");
            return Err(SocketError::NotReportableError(error));
        }
        Some(Err(_)) | None => {
            let error = "Connection closed waiting for subscription".to_string();
            return Err(SocketError::NotReportableError(error));
        }
    };

    // Parse the subscription message.
    let msg = serde_json::from_str::<ToServerStreamingMessage>(&msg).map_err(|err| {
        error!("Could not parse subscription message: {err}");
        let error = "Could not parse subscription message".to_string();
        SocketError::NotReportableError(error)
    })?;

    // Extract the values from the subscription message.
    let (token, value, tag) = match msg {
        ToServerStreamingMessage::DataSubscribeOauth { token, value, tag } => (token, value, tag),
    };

    // Deserialize the incoming data
    let fields = deserialize_field_names(&value);

    // Validate the token
    let claims = validate_access_token(&token, &config).map_err(|err| {
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

    // Say hello to the client. Pretend to be polite. The client will never guess the truth.
    let hello = FromServerStreamingMessage::ControlHello {
        connection_timeout: 30000,
    };
    send_message(socket, hello).await.map_err(|err| {
        error!("Could not send hello: {err:?}");
        let error = DataError::new(&tag, ErrorType::ClientError, "Could not send hello");
        SocketError::ReportableError(error)
    })?;

    // The vehicle_id is the tag
    let vehicle_id: VehicleGuid = tag.clone().parse().map_err(|err| {
        error!("Vehicle id is not an integer: {err}");
        let error = DataError::new(&tag, ErrorType::ClientError, "Invalid vehicle id");
        SocketError::ReportableError(error)
    })?;
    let maybe_vehicle = vehicles.iter().find(|v| v.data.vehicle_id == vehicle_id);
    let Some(vehicle) = maybe_vehicle else {
        error!("Vehicle id not found: {vehicle_id:?}");
        let error = DataError::new(&tag, ErrorType::ClientError, "Invalid vehicle id");
        return Err(SocketError::ReportableError(error));
    };
    let mut rx = vehicle.command.subscribe().await?;

    // Wait for data, either from simulator or from client.
    loop {
        select! {
            // We got Data from the simulator.
            data = rx.recv() => {
                let data = match data {
                    Ok(data) => data,
                    Err(_err) => {
                        let error = DataError::disconnected();
                        return Err(SocketError::ReportableError(error));
                    }
                };
                let value = serialize_fields(&fields, &data);
                let msg = FromServerStreamingMessage::DataUpdate { tag: vehicle_id.to_string(), value };

                debug!("Sending: {msg:?}");
                send_message(socket, msg).await.map_err(|err| {
                    let error = format!("Could not send message: {err:?}");
                    // If we could not send the message, the client is probably gone.
                    // We should probably make funeral arrangements.
                    // But no point trying to tell the client about it.
                    SocketError::NotReportableError(error)
                })?
            }
            // We got a message from the client.
            // We don't expect any messages from the client.
            // The client still thinks we are friends.
            msg = socket.recv() => {
                match msg {
                    Some(Ok(msg)) => {
                        debug!("Unexpected Received: {msg:?}");
                    },

                    Some(Err(err)) => {
                        debug!("Error receiving message: {err}");
                    }
                    None =>  {
                        debug!("Simulator disconnected");
                        break;
                    }
                }
            }
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
