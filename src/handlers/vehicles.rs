//! Vehicle handlers

use std::sync::Arc;

use axum::{extract::Path, Extension, Json};
use serde::Serialize;

use crate::{errors::ResponseError, tokens, TeslaResponse};

/// A vehicle
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct Vehicle {
    /// Vehicle ID for owner-api endpoint.
    pub id: u64,
    /// Vehicle ID for streaming or Auto park API.
    pub vehicle_id: u64,

    /// Vehicle identification number.
    pub vin: String,

    /// Vehicle display name.
    pub display_name: String,

    /// Vehicle option codes.
    option_codes: String,

    /// Vehicle color.
    color: Option<String>,

    /// Vehicle tokens.
    tokens: Vec<String>,

    /// Vehicle state.
    state: String,

    /// Vehicle in service.
    in_service: bool,

    /// Vehicle ID string.
    id_s: String,

    /// Vehicle calendar enabled.
    calendar_enabled: bool,

    /// Vehicle API version.
    api_version: u8,

    /// Vehicle backseat token.
    backseat_token: Option<String>,

    /// Vehicle backseat token updated at.
    backseat_token_updated_at: Option<String>,
}

fn get_vehicles() -> Vec<Vehicle> {
    vec![Vehicle {
        id: 123_456_789,
        vehicle_id: 123_456_789,
        vin: "5YJ3E1EA7JF000000".to_string(),
        display_name: "My Model 3".to_string(),
        option_codes: "AD15,MDL3,PBSB,RENA,BT37,ID3W,RF3G,S3PB,DRLH,APF0,COUS,BC3B,CH07,PC30,FC3P,FG31,GLFR,HL31,HM31,IL31,LLP1,LP01,MR31,FM3B,RS3H,SA3P,STCP,SC04,ST01,SU3C,T3CA,TW00,TM00,UT3P,WR00,AU3P,APH3,AF00,ZCST,MI00,CDM0".to_string(),
        color: Some("Black".to_string()),
        tokens: vec!["abcdef1234567890".to_string()],
        state: "online".to_string(),
        in_service: false,
        id_s: "12345678901234567".to_string(),
        calendar_enabled: true,
        api_version: 6,
        backseat_token: None,
        backseat_token_updated_at: None,
    }, Vehicle {
        id: 123_456_000,
        vehicle_id: 123_456_789,
        vin: "5YJ3E1EA7JF000000".to_string(),
        display_name: "My Model 3".to_string(),
        option_codes: "AD15,MDL3,PBSB,RENA,BT37,ID3W,RF3G,S3PB,DRLH,APF0,COUS,BC3B,CH07,PC30,FC3P,FG31,GLFR,HL31,HM31,IL31,LLP1,LP01,MR31,FM3B,RS3H,SA3P,STCP,SC04,ST01,SU3C,T3CA,TW00,TM00,UT3P,WR00,AU3P,APH3,AF00,ZCST,MI00,CDM0".to_string(),
        color: Some("Black".to_string()),
        tokens: vec!["abcdef1234567890".to_string()],
        state: "online".to_string(),
        in_service: false,
        id_s: "12345678901234567".to_string(),
        calendar_enabled: true,
        api_version: 6,
        backseat_token: None,
        backseat_token_updated_at: None,
    }]
}

/// Get a list of vehicles associated with the authenticated account.
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
pub async fn vehicles_handler(
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
) -> Result<Json<TeslaResponse<Vec<Vehicle>>>, ResponseError> {
    if !config.scopes.contains("vehicle_device_data") {
        return Err(ResponseError::MissingScopes);
    }

    let vehicles = get_vehicles();
    Ok(Json(TeslaResponse::success(vehicles)))
}

/// Get a list of vehicles associated with the authenticated account.
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
pub async fn vehicle_handler(
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<u64>,
) -> Result<Json<TeslaResponse<Vehicle>>, ResponseError> {
    if !config.scopes.contains("vehicle_device_data") {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = get_vehicles()
        .into_iter()
        .find(|v| v.id == id)
        .ok_or(ResponseError::NotFound)?;

    Ok(Json(TeslaResponse::success(vehicle)))
}
