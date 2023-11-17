//! Vehicle handlers

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use std::sync::Arc;

use crate::{errors::ResponseError, tokens, types, TeslaResponse};

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
) -> Result<Json<TeslaResponse<Vec<types::Vehicle>>>, ResponseError> {
    if !config
        .scopes
        .contains(&tokens::ScopeEnum::VehicleDeviceData)
    {
        return Err(ResponseError::MissingScopes);
    }

    let vehicles = (*vehicles).clone();
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
) -> Result<Json<TeslaResponse<types::Vehicle>>, ResponseError> {
    if !config
        .scopes
        .contains(&tokens::ScopeEnum::VehicleDeviceData)
    {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = vehicles
        .iter()
        .find(|v| v.id == id)
        .ok_or(ResponseError::NotFound)?
        .clone();

    Ok(Json(TeslaResponse::success(vehicle)))
}
