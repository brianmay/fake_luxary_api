#![allow(clippy::unwrap_used)]

use fla_common::{
    responses::TeslaResponse,
    types::{VehicleData, VehicleDefinition},
};
use fla_test::get_client;
use restest::assert_body_matches;

#[tokio::test]
async fn test_vehicles() {
    let client = get_client();
    let vehicles = client.get_vehicles().await.unwrap();

    assert_body_matches!(
        vehicles,
        TeslaResponse {
            response: [
                VehicleDefinition {
                    id: 123_456_789,
                    vehicle_id: _,
                    vin: _,
                    display_name: _,
                    option_codes: _,
                    color: _,
                    tokens: _,
                    state: _,
                    in_service: _,
                    id_s: _,
                    calendar_enabled: _,
                    api_version: _,
                    backseat_token: _,
                    backseat_token_updated_at: _,
                },
                VehicleDefinition {
                    id: 123_456_000,
                    vehicle_id: _,
                    vin: _,
                    display_name: _,
                    option_codes: _,
                    color: _,
                    tokens: _,
                    state: _,
                    in_service: _,
                    id_s: _,
                    calendar_enabled: _,
                    api_version: _,
                    backseat_token: _,
                    backseat_token_updated_at: _,
                }
            ],
            error: "",
            error_description: "",
            messages: _
        },
    );
}

#[tokio::test]
async fn test_vehicle_1() {
    let client = get_client();
    let vehicle = client.get_vehicle(123_456_789).await.unwrap();

    assert_body_matches!(
        vehicle,
        TeslaResponse {
            response: VehicleDefinition {
                id: 123_456_789,
                vehicle_id: _,
                vin: _,
                display_name: _,
                option_codes: _,
                color: _,
                tokens: _,
                state: _,
                in_service: _,
                id_s: _,
                calendar_enabled: _,
                api_version: _,
                backseat_token: _,
                backseat_token_updated_at: _,
            },
            error: "",
            error_description: "",
            messages: _
        },
    );
}

#[tokio::test]
async fn test_vehicle_2() {
    let client = get_client();
    let vehicle = client.get_vehicle(123_456_000).await.unwrap();

    assert_body_matches!(
        vehicle,
        TeslaResponse {
            response: VehicleDefinition {
                id: 123_456_000,
                vehicle_id: _,
                vin: _,
                display_name: _,
                option_codes: _,
                color: _,
                tokens: _,
                state: _,
                in_service: _,
                id_s: _,
                calendar_enabled: _,
                api_version: _,
                backseat_token: _,
                backseat_token_updated_at: _,
            },
            error: "",
            error_description: "",
            messages: _
        },
    );
}

#[tokio::test]
async fn test_wakeup() {
    let client = get_client();
    let vehicle = client.get_vehicle(123_456_000).await.unwrap();

    assert_body_matches!(
        vehicle,
        TeslaResponse {
            response: VehicleDefinition {
                id: 123_456_000,
                vehicle_id: _,
                vin: _,
                display_name: _,
                option_codes: _,
                color: _,
                tokens: _,
                state: _,
                in_service: _,
                id_s: _,
                calendar_enabled: _,
                api_version: _,
                backseat_token: _,
                backseat_token_updated_at: _,
            },
            error: "",
            error_description: "",
            messages: _
        },
    );
}

#[tokio::test]
async fn test_vehicle_data() {
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

    let client = get_client();
    let vehicle = client
        .get_vehicle_data(123_456_000, endpoints)
        .await
        .unwrap();

    println!("{:#?}", vehicle.response);

    assert_body_matches!(
        vehicle,
        TeslaResponse {
            response: VehicleData {
                id: 123_456_000,
                user_id: _,
                vehicle_id: _,
                vin: _,
                color: _,
                access_type: _,
                granular_access: _,
                tokens: _,
                state: _,
                in_service: _,
                id_s: _,
                calendar_enabled: _,
                api_version: _,
                backseat_token: _,
                backseat_token_updated_at: _,
                charge_state: _,
                climate_state: _,
                drive_state: _,
                gui_settings: _,
                vehicle_config: _,
                vehicle_state: _,
            },
            error: "",
            error_description: "",
            messages: _
        },
    );
    vehicle.response.charge_state.unwrap();
    vehicle.response.climate_state.unwrap();
    let ds = vehicle.response.drive_state.unwrap();
    ds.latitude.unwrap();
    ds.longitude.unwrap();
    vehicle.response.gui_settings.unwrap();
    vehicle.response.vehicle_config.unwrap();
    vehicle.response.vehicle_state.unwrap();
}
