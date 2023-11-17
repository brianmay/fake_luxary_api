//! Forward commands to the vehicle

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Extension, Json,
};

use crate::{
    errors::ResponseError,
    tokens,
    types::{self, VehicleData},
    TeslaResponse,
};

/// Wake up the vehicle
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
/// Returns a 404 Not Found if the vehicle does not exist.
#[allow(clippy::unused_async)]
pub async fn wake_up_handler(
    State(vehicles): State<Arc<Vec<types::Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<u64>,
) -> Result<Json<TeslaResponse<VehicleData>>, ResponseError> {
    if !config.scopes.contains(&tokens::ScopeEnum::VehicleCmds) {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = vehicles
        .iter()
        .find(|v| v.data.id == id)
        .ok_or(ResponseError::NotFound)?;

    vehicle.command.wake_up().await?;

    Ok(Json(TeslaResponse::success(vehicle.data.clone())))
}
