#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use fla_test::get_token_for_all_scopes;
use futures_util::{SinkExt, StreamExt};
use http::StatusCode;
use serde::{Deserialize, Serialize};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Message,
    },
};

use url::Url;

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
enum TeslaOutMessage {
    #[serde(rename = "data:subscribe_oauth")]
    DataSubscribeOauth {
        token: String,
        value: String,
        tag: u64,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "msg_type")]
enum TeslaInMessage {
    #[serde(rename = "control:hello")]
    ControlHello { connection_timeout: u64 },

    #[serde(rename = "data:update")]
    DataUpdate { tag: u64, value: String },

    #[serde(rename = "data:error")]
    DataError {
        tag: String,
        error_type: ErrorType,
        value: String,
    },
}

#[tokio::test]
async fn test_streaming() {
    let token = get_token_for_all_scopes();

    let url = "ws://localhost:4080/streaming/";

    let (mut socket, response) = connect_async(Url::parse(url).unwrap())
        .await
        .expect("Can't connect");
    assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);

    let msg = TeslaOutMessage::DataSubscribeOauth {
        token: token.access_token,
        value: "speed,odometer,soc,elevation,est_heading,est_lat,est_lng,power,shift_state,range,est_range,heading".to_string(),
        tag: 123_456_000,
    };
    let msg = serde_json::to_string(&msg).unwrap();
    socket.send(Message::Text(msg)).await.unwrap();

    let mut iteration = 0;
    while let Some(msg) = socket.next().await {
        match msg {
            Ok(Message::Text(msg)) => {
                let msg: TeslaInMessage = serde_json::from_str(&msg).unwrap();
                match msg {
                    TeslaInMessage::ControlHello {
                        connection_timeout: _,
                    } => {
                        println!("Received: {msg:?}");
                    }
                    TeslaInMessage::DataUpdate { tag, value } => {
                        println!("Received: {tag} {value}");
                    }
                    TeslaInMessage::DataError {
                        tag,
                        error_type,
                        value,
                    } => {
                        println!("Received: {tag} {error_type:?} {value}");
                    }
                }
            }
            Ok(msg) => {
                println!("Received: {msg:?}");
            }
            Err(e) => {
                println!("Error: {e:?}");
            }
        }

        if iteration > 1 {
            break;
        }
        iteration += 1;
    }

    socket
        .close(Some(CloseFrame {
            code: CloseCode::Normal,
            reason: "I hate you".into(),
        }))
        .await
        .unwrap();
}
