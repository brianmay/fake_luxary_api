#![allow(clippy::unwrap_used)]

use fla_common::types::{VehicleData, VehicleDefinition, VehicleGuid, VehicleId};
use fla_test::get_client;
use restest::assert_body_matches;

#[tokio::test]
async fn test_vehicles() {
    let client = get_client();
    let vehicles = client.get_vehicles().await.unwrap().get_response().unwrap();

    assert_eq!(vehicles[0].id, VehicleId::new(123_456_789));
    assert_eq!(vehicles[0].vehicle_id, VehicleGuid::new(999_456_789));

    assert_eq!(vehicles[1].id, VehicleId::new(123_456_000));
    assert_eq!(vehicles[1].vehicle_id, VehicleGuid::new(999_456_000));

    assert_body_matches!(
        vehicles,
        [
            VehicleDefinition {
                id: _,
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
                id: _,
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
        ]
    );
}

#[tokio::test]
async fn test_vehicle_1() {
    let client = get_client();
    let vehicle = client
        .get_vehicle(123_456_789)
        .await
        .unwrap()
        .get_response()
        .unwrap();

    assert_eq!(vehicle.id, VehicleId::new(123_456_789));
    assert_eq!(vehicle.vehicle_id, VehicleGuid::new(999_456_789));

    assert_body_matches!(
        vehicle,
        VehicleDefinition {
            id: _,
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
    );
}

#[tokio::test]
async fn test_vehicle_2() {
    let client = get_client();
    let vehicle = client
        .get_vehicle(123_456_000)
        .await
        .unwrap()
        .get_response()
        .unwrap();

    assert_eq!(vehicle.id, VehicleId::new(123_456_000));
    assert_eq!(vehicle.vehicle_id, VehicleGuid::new(999_456_000));

    assert_body_matches!(
        vehicle,
        VehicleDefinition {
            id: _,
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
    );
}

#[tokio::test]
async fn test_wakeup() {
    let client = get_client();
    let vehicle = client
        .get_vehicle(123_456_000)
        .await
        .unwrap()
        .get_response()
        .unwrap();

    assert_eq!(vehicle.id, VehicleId::new(123_456_000));
    assert_eq!(vehicle.vehicle_id, VehicleGuid::new(999_456_000));

    assert_body_matches!(
        vehicle,
        VehicleDefinition {
            id: _,
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
        .get_vehicle_data(VehicleId::new(123_456_000), &endpoints)
        .await
        .unwrap()
        .get_response()
        .unwrap();

    assert_eq!(vehicle.id, VehicleId::new(123_456_000));
    assert_eq!(vehicle.vehicle_id, VehicleGuid::new(999_456_000));

    assert_body_matches!(
        vehicle,
        VehicleData {
            id: _,
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
    );

    vehicle.charge_state.unwrap();
    vehicle.climate_state.unwrap();
    let ds = vehicle.drive_state.unwrap();
    ds.latitude.unwrap();
    ds.longitude.unwrap();
    vehicle.gui_settings.unwrap();
    vehicle.vehicle_config.unwrap();
    vehicle.vehicle_state.unwrap();
}
