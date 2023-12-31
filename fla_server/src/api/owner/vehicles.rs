//! Vehicle handlers

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use futures::future::join_all;
use std::{collections::HashSet, str::FromStr, sync::Arc};
use tap::Pipe;
use tracing::error;

use crate::{errors::ResponseError, tokens, types::Vehicle};
use fla_common::{
    responses::{TeslaResponse, VehicleDataResponse, VehicleResponse, VehiclesResponse},
    types::{
        DriveState, VehicleData, VehicleDataEndpoint, VehicleDataQuery, VehicleDefinition,
        VehicleId,
    },
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

    let vehicles: Vec<VehicleDefinition> = vehicles
        .iter()
        .map(|v| async { v.data.read().await.clone() })
        .pipe(join_all)
        .await;

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
    Path(id): Path<VehicleId>,
) -> Result<Json<VehicleResponse>, ResponseError> {
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
        .data
        .read()
        .await
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
    Path(id): Path<VehicleId>,
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
        .find(|v| v.id == id)
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
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::ChargeState));

    let climate_state = data
        .climate_state
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::ClimateState));

    let drive_state = if endpoints.contains(&VehicleDataEndpoint::DriveState) {
        if let Some(ds) = data.drive_state {
            let location = endpoints.contains(&VehicleDataEndpoint::LocationData);

            DriveState {
                latitude: ds.latitude.filter(|_| location),
                longitude: ds.longitude.filter(|_| location),
                ..ds
            }
            .pipe(Some)
        } else {
            None
        }
    } else {
        None
    };

    let gui_settings = data
        .gui_settings
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::GuiSettings));

    let vehicle_config = data
        .vehicle_config
        .filter(|_| endpoints.contains(&VehicleDataEndpoint::VehicleConfig));

    let vehicle_state = data
        .vehicle_state
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
        calendar_enabled: data.calendar_enabled,
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
