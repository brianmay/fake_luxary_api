//! Vehicle handlers

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;

use crate::{
    errors::ResponseError,
    tokens,
    types::{self, VehicleDefinition},
    TeslaResponse,
};

/// Get a list of vehicles associated with the authenticated account.
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
pub async fn vehicles_handler(
    State(vehicles): State<Arc<Vec<types::Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
) -> Result<Json<TeslaResponse<Vec<types::VehicleDefinition>>>, ResponseError> {
    if !config
        .scopes
        .contains(&tokens::ScopeEnum::VehicleDeviceData)
    {
        return Err(ResponseError::MissingScopes);
    }

    let vehicles: Vec<VehicleDefinition> = vehicles.iter().map(|v| v.data.clone()).collect();
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
    State(vehicles): State<Arc<Vec<types::Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<u64>,
) -> Result<Json<TeslaResponse<types::VehicleDefinition>>, ResponseError> {
    if !config
        .scopes
        .contains(&tokens::ScopeEnum::VehicleDeviceData)
    {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = vehicles
        .iter()
        .find(|v| v.data.id == id)
        .ok_or(ResponseError::NotFound)?
        .data
        .clone();

    Ok(Json(TeslaResponse::success(vehicle)))
}

/// Get live vehicle data
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
#[allow(clippy::unused_async)]
pub async fn vehicle_data_handler(
    State(vehicles): State<Arc<Vec<types::Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<u64>,
) -> Result<Json<TeslaResponse<types::VehicleData>>, ResponseError> {
    if !config
        .scopes
        .contains(&tokens::ScopeEnum::VehicleDeviceData)
    {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = vehicles
        .iter()
        .find(|v| v.data.id == id)
        .ok_or(ResponseError::NotFound)?;

    let data = vehicle.command.get_vehicle_data().await?;

    Ok(Json(TeslaResponse::success(data)))
}
