//! Vehicle handlers

use axum::{
    extract::{Path, Query, State},
    Extension, Json,
};
use serde::Serialize;
use std::{collections::HashSet, str::FromStr, sync::Arc};
use tap::Pipe;
use tracing::error;

use crate::{
    errors::ResponseError,
    tokens,
    types::{
        self, ChargeState, ClimateState, DriveState, GranularAccess, GuiSettings, Timestamp,
        VehicleConfig, VehicleDefinition, VehicleState,
    },
    TeslaResponse,
};

#[derive(Eq, PartialEq, Hash)]
enum Endpoint {
    ChargeState,
    ClimateState,
    ClosuresState,
    DriveState,
    GuiSettings,
    LocationData,
    VehicleConfig,
    VehicleState,
    VehicleDataCombo,
}

impl FromStr for Endpoint {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "charge_state" => Ok(Self::ChargeState),
            "climate_state" => Ok(Self::ClimateState),
            "closures_state" => Ok(Self::ClosuresState),
            "drive_state" => Ok(Self::DriveState),
            "gui_settings" => Ok(Self::GuiSettings),
            "location_data" => Ok(Self::LocationData),
            "vehicle_config" => Ok(Self::VehicleConfig),
            "vehicle_state" => Ok(Self::VehicleState),
            "vehicle_data_combo" => Ok(Self::VehicleDataCombo),
            _ => Err(format!("Invalid endpoint: {s}")),
        }
    }
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct VehicleDataResponse {
    pub id: i64,
    pub user_id: i64,
    pub vehicle_id: i64,
    pub vin: String,
    pub color: Option<String>,
    pub access_type: String,
    pub granular_access: GranularAccess,
    pub tokens: Vec<String>,
    pub state: Option<String>,
    pub in_service: bool,
    pub id_s: String,
    pub calendar_enabled: bool,
    pub api_version: i64,
    pub backseat_token: Option<String>,
    pub backseat_token_updated_at: Option<Timestamp>,
    pub charge_state: Option<ChargeState>,
    pub climate_state: Option<ClimateState>,
    pub drive_state: Option<DriveState>,
    pub gui_settings: Option<GuiSettings>,
    pub vehicle_config: Option<VehicleConfig>,
    pub vehicle_state: Option<VehicleState>,
}

/// Query parameters for vehicle data
#[derive(serde::Deserialize, Debug)]
pub struct VehicleDataQuery {
    /// List of endpoints to retrieve
    endpoints: Option<String>,
}

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
    query: Query<VehicleDataQuery>,
) -> Result<Json<TeslaResponse<VehicleDataResponse>>, ResponseError> {
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
            e.split(',')
                .map(Endpoint::from_str)
                .collect::<Result<HashSet<_>, _>>()
                .map_err(|err| {
                    error!("xxxx not valid: {err:?}");
                    err
                })
        })
        .map_or(Ok(None), |e| e.map(Some))
        .map_err(|_| ResponseError::InvalidCommand)?
        .unwrap_or_default();

    let charge_state = data
        .charge_state
        .pipe(Some)
        .filter(|_| endpoints.contains(&Endpoint::ChargeState));

    let climate_state = data
        .climate_state
        .pipe(Some)
        .filter(|_| endpoints.contains(&Endpoint::ClimateState));

    let drive_state = if endpoints.contains(&Endpoint::DriveState) {
        let location = endpoints.contains(&Endpoint::LocationData);

        DriveState {
            heading: data.drive_state.heading.filter(|_| location),
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
        .filter(|_| endpoints.contains(&Endpoint::GuiSettings));

    let vehicle_config = data
        .vehicle_config
        .pipe(Some)
        .filter(|_| endpoints.contains(&Endpoint::VehicleConfig));

    let vehicle_state = data
        .vehicle_state
        .pipe(Some)
        .filter(|_| endpoints.contains(&Endpoint::VehicleState));

    let response = VehicleDataResponse {
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
