//! This is a test binary for the streaming API.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use clap::Parser;
use fla_common::{streaming::StreamingFields, types::VehicleGuid};
use fla_server::tokens::ScopeEnum;
use fla_test::{get_client_with_token, get_token_with_scopes};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Parameters {
    vehicle_id: VehicleGuid,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let params = Parameters::parse();

    let scopes = [ScopeEnum::VehicleDeviceData].into();
    let token = get_token_with_scopes(&scopes);
    let client = get_client_with_token(token);

    let fields = vec![
        StreamingFields::Speed,
        StreamingFields::Odometer,
        StreamingFields::Soc,
        StreamingFields::Elevation,
        StreamingFields::EstHeading,
        StreamingFields::EstLat,
        StreamingFields::EstLng,
        StreamingFields::Power,
        StreamingFields::ShiftState,
        StreamingFields::Range,
        StreamingFields::EstRange,
        StreamingFields::Heading,
    ];
    let mut streaming = client.streaming(params.vehicle_id, fields).unwrap();

    while let Some(msg) = streaming.recv().await {
        println!("Woof Received: {msg:?}");
    }
}
