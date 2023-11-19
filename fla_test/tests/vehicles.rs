#![allow(clippy::unwrap_used)]

use fla_test::{get_token_for_all_scopes, URL};
use reqwest::Client;
use restest::assert_body_matches;
use serde::Deserialize;
use serde_json::Value;

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
pub struct Vehicle {
    /// Vehicle ID for owner-api endpoint.
    pub id: u64,
    /// Vehicle ID for streaming or Auto park API.
    pub vehicle_id: u64,

    /// Vehicle identification number.
    pub vin: String,

    /// Vehicle display name.
    pub display_name: String,
    option_codes: String,
    color: Option<String>,
    tokens: Vec<String>,
    state: String,
    in_service: bool,
    id_s: String,
    calendar_enabled: bool,
    api_version: u8,
    backseat_token: Option<String>,
    backseat_token_updated_at: Option<String>,
}

#[derive(Debug, Deserialize)]
struct VehiclesResponse {
    response: Vec<Vehicle>,
}

#[derive(Debug, Deserialize)]
struct VehicleResponse {
    response: Vehicle,
}

#[allow(dead_code)]
#[derive(Debug, Deserialize)]
struct VehicleDataResponse {
    response: Value,
}

#[tokio::test]
async fn test_vehicles() {
    let token = get_token_for_all_scopes();

    let url = format!("{URL}api/1/vehicles");
    let vehicles = Client::new()
        .get(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token.access_token)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<VehiclesResponse>()
        .await
        .unwrap();

    assert_body_matches!(
        vehicles,
        VehiclesResponse {
            response: [
                Vehicle {
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
                Vehicle {
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
        },
    );
}

#[tokio::test]
async fn test_vehicle_1() {
    let token = get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let url = format!("{URL}api/1/vehicles/{}", 123_456_789);
    let vehicle = Client::new()
        .get(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token.access_token)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<VehicleResponse>()
        .await
        .unwrap();

    assert_body_matches!(
        vehicle,
        VehicleResponse {
            response: Vehicle {
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
            }
        },
    );
}

#[tokio::test]
async fn test_vehicle_2() {
    let token = get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let url = format!("{URL}api/1/vehicles/{}", 123_456_000);
    let vehicle = Client::new()
        .get(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token.access_token)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<VehicleResponse>()
        .await
        .unwrap();

    assert_body_matches!(
        vehicle,
        VehicleResponse {
            response: Vehicle {
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
        },
    );
}

#[tokio::test]
async fn test_wakeup() {
    let token = get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let url = format!("{URL}api/1/vehicles/{}/wake_up", 123_456_000);
    let vehicle = Client::new()
        .post(url)
        .header("Content-Type", "application/json")
        .bearer_auth(token.access_token)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<VehicleResponse>()
        .await
        .unwrap();

    assert_body_matches!(
        vehicle,
        VehicleResponse {
            response: Vehicle {
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
        },
    );
}

#[tokio::test]
async fn test_vehicle_data() {
    let token = get_token_for_all_scopes();

    let endpoints = "charge_state,climate_state,closures_state,drive_state,gui_settings,location_data,vehicle_config,vehicle_state,vehicle_data_combo";
    let query = [("endpoints", endpoints)];

    // Test code that use `CONTEXT` for a specific route
    let url = format!("{URL}api/1/vehicles/{}/vehicle_data", 123_456_000);
    let vehicle = Client::new()
        .get(url)
        .query(&query)
        .header("Content-Type", "application/json")
        .bearer_auth(token.access_token)
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<VehicleDataResponse>()
        .await
        .unwrap();

    assert_body_matches!(vehicle, VehicleDataResponse { response: _ },);
    vehicle
        .response
        .as_object()
        .unwrap()
        .get("charge_state")
        .unwrap()
        .as_object()
        .unwrap();

    vehicle
        .response
        .as_object()
        .unwrap()
        .get("climate_state")
        .unwrap()
        .as_object()
        .unwrap();

    vehicle
        .response
        .as_object()
        .unwrap()
        .get("drive_state")
        .unwrap()
        .as_object()
        .unwrap();

    vehicle
        .response
        .as_object()
        .unwrap()
        .get("gui_settings")
        .unwrap()
        .as_object()
        .unwrap();

    vehicle
        .response
        .as_object()
        .unwrap()
        .get("vehicle_config")
        .unwrap()
        .as_object()
        .unwrap();

    vehicle
        .response
        .as_object()
        .unwrap()
        .get("vehicle_state")
        .unwrap()
        .as_object()
        .unwrap();
}
