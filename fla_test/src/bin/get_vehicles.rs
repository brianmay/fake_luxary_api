//! This is a test binary for the streaming API.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use fla_server::tokens::ScopeEnum;
use fla_test::{get_client_with_token, get_token_with_scopes};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let scopes = [ScopeEnum::VehicleDeviceData].into();
    let token = get_token_with_scopes(&scopes);
    let client = get_client_with_token(token);

    let vehicles = client.get_vehicles().await.unwrap();
    println!("{vehicles:#?}");
}
