use std::fmt::Formatter;

use crate::simulator;
use fla_common::types::{
    ChargeState, ClimateState, DriveState, GranularAccess, GuiSettings, ShiftState, Timestamp,
    VehicleConfig, VehicleDefinition, VehicleId, VehicleState,
};
use serde::Serialize;

/// A vehicle
pub struct Vehicle {
    /// The vehicle data
    pub data: VehicleDefinition,

    /// The command sender
    pub command: simulator::CommandSender,

    /// The stream receiver
    pub stream: simulator::StreamReceiver,
}

impl Vehicle {
    /// Create a new vehicle
    #[must_use]
    pub fn new(data: VehicleDefinition) -> Self {
        let (command, stream) = simulator::server::start(data.clone());
        Self {
            data,
            command,
            stream,
        }
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
#[derive(Default, Debug, Clone, Serialize)]
pub struct VehicleDataState {
    pub id: VehicleId,
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
    pub charge_state: ChargeState,
    pub climate_state: ClimateState,
    pub drive_state: DriveState,
    pub gui_settings: GuiSettings,
    pub vehicle_config: VehicleConfig,
    pub vehicle_state: VehicleState,
}

#[derive(Clone, Debug)]
pub struct StreamingData {
    /// The vehicle id.
    pub id: VehicleId,

    /// Unix timestamp in milliseconds.
    pub time: Timestamp,

    /// Speed in km per hour.
    pub speed: Option<u32>,

    /// Odometer reading in km.
    pub odometer: f64,

    /// State of charge as a percentage.
    pub soc: u8,

    /// Elevation in meters.
    pub elevation: u32,

    /// Estimated heading in degrees.
    pub est_heading: u16,

    /// Estimated latitude in decimal degrees.
    pub est_lat: f64,

    /// Estimated longitude in decimal degrees.
    pub est_lng: f64,

    /// Power usage in watts.
    pub power: Option<i32>,

    /// Shift state of the vehicle.
    pub shift_state: Option<ShiftState>,

    /// Estimated range in km.
    pub range: u32,

    /// Estimated range based on energy usage in km.
    pub est_range: u32,

    /// Heading in degrees.
    pub heading: u16,
}
