//! This is a test binary for the streaming API.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use clap::Parser;
use fla_common::types::VehicleId;
use fla_server::tokens::ScopeEnum;
use fla_test::{get_client_with_token, get_token_with_scopes};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Parameters {
    vehicle_id: VehicleId,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let params = Parameters::parse();

    let scopes = [ScopeEnum::VehicleCmds].into();
    let token = get_token_with_scopes(&scopes);
    let client = get_client_with_token(token);

    client.wake_up(params.vehicle_id).await.unwrap();
}
