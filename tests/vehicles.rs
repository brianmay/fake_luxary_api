use http::StatusCode;
use restest::{assert_body_matches, path, Context, Request};
use serde::Deserialize;
use serde_json::Value;

mod common;

const CONTEXT: Context = Context::new().with_port(4080);

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
    let token = common::get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let request = Request::get(path!["api", 1, "vehicles"])
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", format!("Bearer {}", token.access_token))
        .with_body(());

    let vehicles: VehiclesResponse = CONTEXT
        .run(request)
        .await
        .expect_status(StatusCode::OK)
        .await;

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
    let token = common::get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let request = Request::get(path!["api", 1, "vehicles", 123_456_789])
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", format!("Bearer {}", token.access_token))
        .with_body(());

    let vehicle: VehicleResponse = CONTEXT
        .run(request)
        .await
        .expect_status(StatusCode::OK)
        .await;

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
    let token = common::get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let request = Request::get(path!["api", 1, "vehicles", 123_456_000])
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", format!("Bearer {}", token.access_token))
        .with_body(());

    let vehicle: VehicleResponse = CONTEXT
        .run(request)
        .await
        .expect_status(StatusCode::OK)
        .await;

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
    let token = common::get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let request = Request::post(path!["api", 1, "vehicles", 123_456_000, "wake_up"])
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", format!("Bearer {}", token.access_token))
        .with_body(());

    let vehicle: VehicleResponse = CONTEXT
        .run(request)
        .await
        .expect_status(StatusCode::OK)
        .await;

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
    let token = common::get_token_for_all_scopes();

    // Test code that use `CONTEXT` for a specific route
    let request = Request::get(path!["api", 1, "vehicles", 123_456_000, "vehicle_data"])
        .with_header("Content-Type", "application/json")
        .with_header("Authorization", format!("Bearer {}", token.access_token))
        .with_body(());

    let vehicle: VehicleDataResponse = CONTEXT
        .run(request)
        .await
        .expect_status(StatusCode::OK)
        .await;

    assert_body_matches!(vehicle, VehicleDataResponse { response: _ },);
}
