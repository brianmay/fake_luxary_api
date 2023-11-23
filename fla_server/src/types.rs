use std::fmt::Formatter;

use crate::simulator;
use fla_common::{
    streaming::StreamingData,
    types::{
        ChargeState, ClimateState, DriveState, GranularAccess, GuiSettings, Timestamp,
        VehicleConfig, VehicleDefinition, VehicleGuid, VehicleId, VehicleState,
    },
};
use serde::Serialize;

/// A vehicle
pub struct Vehicle {
    /// The vehicle data
    pub data: VehicleDefinition,

    /// The command sender
    pub command: simulator::CommandSender,
}

impl Vehicle {
    /// Create a new vehicle
    #[must_use]
    pub fn new(data: VehicleDefinition) -> Self {
        let command = simulator::server::start(data.clone());
        Self { data, command }
    }
}
impl std::fmt::Debug for Vehicle {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Vehicle")
            .field("data", &self.data)
            // .field("command", &self.command)
            // .field("stream", &self.stream)
            .finish_non_exhaustive()
    }
}

/// Current state of all Vehicle Data
#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize)]
pub struct VehicleDataState {
    pub id: VehicleId,
    pub user_id: i64,
    pub vehicle_id: VehicleGuid,
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
    pub charge_state: ChargeState,
    pub climate_state: ClimateState,
    pub drive_state: DriveState,
    pub gui_settings: GuiSettings,
    pub vehicle_config: VehicleConfig,
    pub vehicle_state: VehicleState,
}

impl From<&VehicleDataState> for StreamingData {
    fn from(data: &VehicleDataState) -> Self {
        Self {
            id: data.vehicle_id,
            time: data.drive_state.timestamp,
            speed: data.drive_state.speed,
            odometer: Some(data.vehicle_state.odometer),
            soc: Some(data.charge_state.battery_level),
            // FIXME
            elevation: Some(0),
            est_heading: Some(data.drive_state.heading),
            est_lat: data.drive_state.latitude,
            est_lng: data.drive_state.longitude,
            power: data.drive_state.power,
            shift_state: data.drive_state.shift_state.clone(),
            range: Some(data.charge_state.battery_range),
            est_range: Some(data.charge_state.est_battery_range),
            heading: Some(data.drive_state.heading),
        }
    }
}
