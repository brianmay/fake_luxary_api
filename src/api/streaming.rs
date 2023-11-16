//! Streaming handler
use std::{sync::Arc, time::Duration};

use axum::{
    extract::{
        ws::{Message, WebSocket, WebSocketUpgrade},
        State,
    },
    response::IntoResponse,
    routing::get,
    Router,
};
use serde::{Deserialize, Serialize};
use tokio::time::sleep;
use tracing::{debug, error};

use crate::{
    tokens::{self, validate_access_token},
    Config,
};

/// Retrieve router for Tesla streaming API
///
pub fn router(config: &Config) -> Router {
    Router::new()
        .route("/streaming/", get(ws_handler))
        .with_state(config.clone())
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
enum Fields {
    Speed,
    Odometer,
    Soc,
    Elevation,
    EstHeading,
    EstLat,
    EstLng,
    Power,
    ShiftState,
    Range,
    EstRange,
    Heading,
}

struct StreamingData {
    time: u64,
    speed: u32,
    odometer: u64,
    soc: u8,
    elevation: u32,
    est_heading: u16,
    est_lat: f32,
    est_lng: f32,
    power: String,
    shift_state: String,
    range: u32,
    est_range: u32,
    heading: u16,
}

fn deserialize_field_names(str: &str) -> Vec<Fields> {
    let mut result = Vec::new();
    for field in str.split(',') {
        match field {
            "speed" => result.push(Fields::Speed),
            "odometer" => result.push(Fields::Odometer),
            "soc" => result.push(Fields::Soc),
            "elevation" => result.push(Fields::Elevation),
            "est_heading" => result.push(Fields::EstHeading),
            "est_lat" => result.push(Fields::EstLat),
            "est_lng" => result.push(Fields::EstLng),
            "power" => result.push(Fields::Power),
            "shift_state" => result.push(Fields::ShiftState),
            "range" => result.push(Fields::Range),
            "est_range" => result.push(Fields::EstRange),
            "heading" => result.push(Fields::Heading),
            _ => {}
        }
    }
    result
}

fn serialize_fields(fields: &[Fields], data: &StreamingData) -> String {
    let mut result = Vec::new();
    result.push(data.time.to_string());
    for field in fields {
        match field {
            Fields::Speed => result.push(data.speed.to_string()),
            Fields::Odometer => result.push(data.odometer.to_string()),
            Fields::Soc => result.push(data.soc.to_string()),
            Fields::Elevation => result.push(data.elevation.to_string()),
            Fields::EstHeading => result.push(data.est_heading.to_string()),
            Fields::EstLat => result.push(data.est_lat.to_string()),
            Fields::EstLng => result.push(data.est_lng.to_string()),
            Fields::Power => result.push(data.power.to_string()),
            Fields::ShiftState => result.push(data.shift_state.to_string()),
            Fields::Range => result.push(data.range.to_string()),
            Fields::EstRange => result.push(data.est_range.to_string()),
            Fields::Heading => result.push(data.heading.to_string()),
        }
    }
    result.join(",")
}

#[derive(Deserialize, Serialize, Debug)]
enum ErrorType {
    #[serde(rename = "vehicle_disconnected")]
    VehicleDisconnected,

    #[serde(rename = "vehicle_error")]
    VehicleError,

    #[serde(rename = "client_error")]
    ClientError,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "msg_type")]
enum TeslaInMessage {
    #[serde(rename = "data:subscribe_oauth")]
    DataSubscribeOauth {
        token: String,
        value: String,
        tag: String,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "msg_type")]
enum TeslaOutMessage {
    #[serde(rename = "control:hello")]
    ControlHello { connection_timeout: u64 },

    #[serde(rename = "data:update")]
    DataUpdate { tag: String, value: String },

    #[serde(rename = "data:error")]
    DataError {
        tag: String,
        error_type: ErrorType,
        value: String,
    },
}

/// The handler for the HTTP request (this gets called when the HTTP GET lands at the start
/// of websocket negotiation). After this completes, the actual switching from HTTP to
/// websocket protocol will occur.
/// This is the last point where we can extract TCP/IP metadata such as IP address of the client
/// as well as things from HTTP headers such as user-agent of the browser etc.
#[allow(clippy::unused_async)]
pub async fn ws_handler(
    State(config): State<Arc<tokens::Config>>,
    ws: WebSocketUpgrade,
) -> impl IntoResponse {
    // finalize the upgrade process by returning upgrade callback.
    // we can customize the callback by sending additional info such as address.
    ws.on_upgrade(|socket| handle_socket(socket, config))
}

/// Actual websocket statemachine (one will be spawned per connection)
#[allow(clippy::too_many_lines)]
async fn handle_socket(mut socket: WebSocket, config: Arc<tokens::Config>) {
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

    let Ok(msg) = serde_json::from_str::<TeslaInMessage>(&msg) else {
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
        TeslaInMessage::DataSubscribeOauth { token, value, tag } => (token, value, tag),
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
    let hello = TeslaOutMessage::ControlHello {
        connection_timeout: 30000,
    };
    if send_message(&mut socket, hello).await.is_err() {
        _ = socket.close().await;
        return;
    }

    loop {
        let data = StreamingData {
            time: 0,
            speed: 0,
            odometer: 0,
            soc: 0,
            elevation: 0,
            est_heading: 0,
            est_lat: 0.0,
            est_lng: 0.0,
            power: String::new(),
            shift_state: String::new(),
            range: 0,
            est_range: 0,
            heading: 0,
        };

        let value = serialize_fields(&fields, &data);
        let msg = TeslaOutMessage::DataUpdate {
            tag: tag.clone(),
            value,
        };

        debug!("Sending: {msg:?}");
        if send_message(&mut socket, msg).await.is_err() {
            _ = socket.close().await;
            return;
        }

        sleep(Duration::from_secs(5)).await;
    }
}

async fn send_message(socket: &mut WebSocket, message: TeslaOutMessage) -> Result<(), ()> {
    let Ok(text) = serde_json::to_string(&message) else {
        error!("Could not serialize message!");
        return Err(());
    };

    if socket.send(Message::Text(text)).await.is_ok() {
        Ok(())
    } else {
        error!("Could not send a message!");
        // no Error here since the only thing we can do is to close the connection.
        // If we can not send messages, there is no way to salvage the statemachine anyway.
        Err(())
    }
}

async fn send_error(socket: &mut WebSocket, tag: String, error_type: ErrorType, value: String) {
    let msg = TeslaOutMessage::DataError {
        tag,
        error_type,
        value,
    };
    _ = send_message(socket, msg).await;
}
