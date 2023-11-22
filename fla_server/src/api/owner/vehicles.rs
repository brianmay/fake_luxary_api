//! Vehicle handlers

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use std::{collections::HashSet, str::FromStr, sync::Arc};
use tap::Pipe;
use tracing::error;

use crate::{errors::ResponseError, tokens, types::Vehicle};
use fla_common::{
    responses::{TeslaResponse, VehicleDataResponse, VehicleResponse, VehiclesResponse},
    types::{DriveState, VehicleData, VehicleDataEndpoint, VehicleDataQuery, VehicleDefinition},
};

/// Get a list of vehicles associated with the authenticated account.
///
/// # Errors
///
/// Returns a 403 Forbidden if the token does not have the required scopes.
#[allow(clippy::module_name_repetitions)]
#[allow(clippy::unused_async)]
pub async fn vehicles_handler(
    State(vehicles): State<Arc<Vec<Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
) -> Result<Json<VehiclesResponse>, ResponseError> {
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
    State(vehicles): State<Arc<Vec<Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<u64>,
) -> Result<Json<VehicleResponse>, ResponseError> {
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
    State(vehicles): State<Arc<Vec<Vehicle>>>,
    Extension(config): Extension<Arc<tokens::AccessClaims>>,
    Path(id): Path<u64>,
    query: Query<VehicleDataQuery>,
) -> Result<Json<VehicleDataResponse>, ResponseError> {
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

    let endpoints = query
        .endpoints
        .as_ref()
        .map(|e| {
            e.split(';')
                .map(VehicleDataEndpoint::from_str)
                .collect::<Result<HashSet<_>, _>>()
                .map_err(|err| {
                    error!("endpoints not valid: {err:?}");
                    err
                })
        })
        .transpose()
        .map_err(|_| ResponseError::InvalidCommand)?
        .unwrap_or_default();

    let charge_state = data
        .charge_state
        .pipe(Some)
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::ChargeState));

    let climate_state = data
        .climate_state
        .pipe(Some)
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::ClimateState));

    let drive_state = if endpoints.contains(&VehicleDataEndpoint::DriveState) {
        let location = endpoints.contains(&VehicleDataEndpoint::LocationData);

        DriveState {
            latitude: data.drive_state.latitude.filter(|_| location),
            longitude: data.drive_state.longitude.filter(|_| location),
            ..data.drive_state
        }
        .pipe(Some)
    } else {
        None
    };

    let gui_settings = data
        .gui_settings
        .pipe(Some)
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::GuiSettings));

    let vehicle_config = data
        .vehicle_config
        .pipe(Some)
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::VehicleConfig));

    let vehicle_state = data
        .vehicle_state
        .pipe(Some)
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::VehicleState));

    let response = VehicleData {
        id: data.id,
        user_id: data.user_id,
        vehicle_id: data.vehicle_id,
        vin: data.vin,
        color: data.color,
        access_type: data.access_type,
        granular_access: data.granular_access,
        tokens: data.tokens,
        state: data.state,
        in_service: data.in_service,
        id_s: data.id_s.clone(),
        calendar_enabled: vehicle.data.calendar_enabled,
        api_version: data.api_version,
        backseat_token: data.backseat_token,
        backseat_token_updated_at: data.backseat_token_updated_at,
        charge_state,
        climate_state,
        drive_state,
        gui_settings,
        vehicle_config,
        vehicle_state,
    };

    Ok(Json(TeslaResponse::success(response)))
}
