//! This is a test binary for the streaming API.

#![allow(clippy::unwrap_used)]
#![allow(clippy::expect_used)]

use fla_common::types::VehicleStateEnum;
use fla_server::tokens::ScopeEnum;
use fla_test::{get_client_with_token, get_token_with_scopes};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let endpoints = [
        fla_common::types::VehicleDataEndpoint::ChargeState,
        fla_common::types::VehicleDataEndpoint::ClimateState,
        fla_common::types::VehicleDataEndpoint::DriveState,
        fla_common::types::VehicleDataEndpoint::LocationData,
        fla_common::types::VehicleDataEndpoint::GuiSettings,
        fla_common::types::VehicleDataEndpoint::VehicleConfig,
        fla_common::types::VehicleDataEndpoint::VehicleState,
        fla_common::types::VehicleDataEndpoint::VehicleDataCombo,
    ]
    .into();

    let scopes = [ScopeEnum::VehicleDeviceData].into();
    let token = get_token_with_scopes(&scopes);
    let client = get_client_with_token(token);

    let vehicles = client.get_vehicles().await.unwrap();

    for vehicle in vehicles.get_response().unwrap() {
        println!("vehicle: {vehicle:#?}");

        if vehicle.state != VehicleStateEnum::Online {
            println!("vehicle is not online");
            println!();
            continue;
        }

        let vehicle_id = vehicle.id;
        let vehicle_data = client
            .get_vehicle_data(vehicle_id, &endpoints)
            .await
            .unwrap();
        let vehicle_data = vehicle_data.get_response().unwrap();
        println!("vehicle_data: {vehicle_data:#?}");
        println!();
    }
}
