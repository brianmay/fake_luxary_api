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
use fla_common::streaming::{
    ErrorType, FromServerStreamingMessage, StreamingData, StreamingFields, ToServerStreamingMessage,
};
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
            StreamingFields::ShiftState => push_data(&mut result, data.shift_state),
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

/// Actual websocket state machine (one will be spawned per connection)
#[allow(clippy::too_many_lines)]
async fn handle_socket(
    mut socket: WebSocket,
    config: Arc<tokens::Config>,
    vehicles: Arc<Vec<Vehicle>>,
) {
    // receive single message from a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    let msg = match socket.recv().await {
        Some(Ok(Message::Text(text))) => text,
        Some(Ok(msg)) => {
            error!("Unexpected message: {msg:?}");
            send_error(
                &mut socket,
                "0".to_string(),
                ErrorType::ClientError,
                "Unexpected message".to_string(),
            )
            .await;
            _ = socket.close().await;
            return;
        }
        Some(Err(_)) | None => {
            error!("client abruptly disconnected");
            _ = socket.close().await;
            return;
        }
    };

    let Ok(msg) = serde_json::from_str::<ToServerStreamingMessage>(&msg) else {
        error!("Could not parse message!");
        send_error(
            &mut socket,
            "0".to_string(),
            ErrorType::ClientError,
            "Could not parse message".to_string(),
        )
        .await;
        _ = socket.close().await;
        return;
    };

    let (token, value, tag) = match msg {
        ToServerStreamingMessage::DataSubscribeOauth { token, value, tag } => (token, value, tag),
    };

    // tag is the vehicle_id

    let fields = deserialize_field_names(&value);

    let Ok(claims) = validate_access_token(&token, &config) else {
        error!("Invalid token!");
        send_error(
            &mut socket,
            "0".to_string(),
            ErrorType::ClientError,
            "Invalid token".to_string(),
        )
        .await;
        _ = socket.close().await;
        return;
    };

    if !claims
        .scopes
        .contains(&tokens::ScopeEnum::VehicleDeviceData)
    {
        error!("Invalid scope!");
        send_error(
            &mut socket,
            "0".to_string(),
            ErrorType::ClientError,
            "Invalid scope".to_string(),
        )
        .await;
        _ = socket.close().await;
        return;
    }

    // send a message to a client (we can either receive or send with socket).
    // this will likely be the Pong for our Ping or a hello message from client.
    // waiting for message from a client will block this task, but will not block other client's
    // connections.
    let hello = FromServerStreamingMessage::ControlHello {
        connection_timeout: 30000,
    };
    if send_message(&mut socket, hello).await.is_err() {
        _ = socket.close().await;
        return;
    }

    let maybe_vehicle = vehicles.iter().find(|v| v.data.id == tag);

    let Some(vehicle) = maybe_vehicle else {
        error!("Invalid vehicle id!");
        send_error(
            &mut socket,
            "0".to_string(),
            ErrorType::ClientError,
            "Invalid vehicle id".to_string(),
        )
        .await;
        _ = socket.close().await;
        return;
    };

    let mut rx = vehicle.stream.subscribe();

    loop {
        select! {
            data = rx.recv() => {
                let data = match data {
                    Ok(data) => data,
                    Err(_err) => {
                        debug!("Vehicle disconnected");
                        send_error(
                            &mut socket,
                            tag.to_string(),
                            ErrorType::VehicleDisconnected,
                            "Vehicle disconnected".to_string(),
                        )
                        .await;
                        _ = socket.close().await;
                        return;
                    }
                };
                let value = serialize_fields(&fields, &data);
                let msg = FromServerStreamingMessage::DataUpdate { tag, value };

                debug!("Sending: {msg:?}");
                if send_message(&mut socket, msg).await.is_err() {
                    _ = socket.close().await;
                    return;
                }
            }
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
}

async fn send_message(
    socket: &mut WebSocket,
    message: FromServerStreamingMessage,
) -> Result<(), ()> {
    let Ok(text) = serde_json::to_string(&message) else {
        error!("Could not serialize message!");
        return Err(());
    };

    if socket.send(Message::Text(text)).await.is_ok() {
        Ok(())
    } else {
        error!("Could not send a message!");
        Err(())
    }
}

async fn send_error(socket: &mut WebSocket, tag: String, error_type: ErrorType, value: String) {
    let msg = FromServerStreamingMessage::DataError {
        tag,
        error_type,
        value,
    };
    _ = send_message(socket, msg).await;
}
