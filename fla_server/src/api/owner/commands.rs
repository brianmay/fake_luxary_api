//! Forward commands to the vehicle

use std::sync::Arc;

use axum::{
    extract::{Path, State},
    Extension, Json,
};
use fla_common::{
    responses::{TeslaResponse, VehicleResponse},
    simulator::SimulationStateEnum,
    types::VehicleId,
};

use crate::{errors::ResponseError, tokens, types::Vehicle};

/// Wake up the vehicle
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
/// Returns a 404 Not Found if the vehicle does not exist.
#[allow(clippy::unused_async)]
pub async fn wake_up_handler(
    State(vehicles): State<Arc<Vec<Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<VehicleId>,
) -> Result<Json<VehicleResponse>, ResponseError> {
    if !config.scopes.contains(&tokens::ScopeEnum::VehicleCmds) {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = vehicles
        .iter()
        .find(|v| v.id == id)
        .ok_or(ResponseError::NotFound)?;

    vehicle.command.wake_up().await?;

    let response = vehicle.data.read().await.clone();
    Ok(Json(TeslaResponse::success(response)))
}

/// Send a simulate command to the vehicle
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
/// Returns a 404 Not Found if the vehicle does not exist.
#[allow(clippy::unused_async)]
pub async fn simulate_handler(
    State(vehicles): State<Arc<Vec<Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<VehicleId>,
    Json(state): Json<SimulationStateEnum>,
) -> Result<(), ResponseError> {
    if !config.scopes.contains(&tokens::ScopeEnum::VehicleCmds) {
        return Err(ResponseError::MissingScopes);
    }

    let vehicle = vehicles
        .iter()
        .find(|v| v.id == id)
        .ok_or(ResponseError::NotFound)?;

    vehicle.command.simulate(state).await?;

    Ok(())
}
