//! Shared types for all API

use std::fmt::Formatter;

use serde::Serialize;

use crate::simulator;

/// A vehicle
pub struct Vehicle {
    /// The vehicle data
    pub data: VehicleData,

    /// The command sender
    pub command: simulator::CommandSender,

    /// The stream receiver
    pub stream: simulator::StreamReceiver,
}

impl Vehicle {
    /// Create a new vehicle
    #[must_use]
    pub fn new(data: VehicleData) -> Self {
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

/// The data associated with a Vehicle
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct VehicleData {
    /// Vehicle ID for owner-api endpoint.
    pub id: u64,
    /// Vehicle ID for streaming or Auto park API.
    pub vehicle_id: u64,

    /// Vehicle identification number.
    pub vin: String,

    /// Vehicle display name.
    pub display_name: String,

    /// Vehicle option codes.
    pub option_codes: String,

    /// Vehicle color.
    pub color: Option<String>,

    /// Vehicle tokens.
    pub tokens: Vec<String>,

    /// Vehicle state.
    pub state: String,

    /// Vehicle in service.
    pub in_service: bool,

    /// Vehicle ID string.
    pub id_s: String,

    /// Vehicle calendar enabled.
    pub calendar_enabled: bool,

    /// Vehicle API version.
    pub api_version: u8,

    /// Vehicle backseat token.
    pub backseat_token: Option<String>,

    /// Vehicle backseat token updated at.
    pub backseat_token_updated_at: Option<String>,
}

/// Struct representing streaming data from a vehicle.
pub struct StreamingData {
    /// Unix timestamp in milliseconds.
    pub time: u64,

    /// Speed in km per hour.
    pub speed: u32,

    /// Odometer reading in km.
    pub odometer: u64,

    /// State of charge as a percentage.
    pub soc: u8,

    /// Elevation in meters.
    pub elevation: u32,

    /// Estimated heading in degrees.
    pub est_heading: u16,

    /// Estimated latitude in decimal degrees.
    pub est_lat: f32,

    /// Estimated longitude in decimal degrees.
    pub est_lng: f32,

    /// Power usage in watts.
    pub power: String,

    /// Shift state of the vehicle.
    pub shift_state: String,

    /// Estimated range in km.
    pub range: u32,

    /// Estimated range based on energy usage in km.
    pub est_range: u32,

    /// Heading in degrees.
    pub heading: u16,
}
