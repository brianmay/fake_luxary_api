use std::str::FromStr;

use serde::{Deserialize, Serialize};

use crate::types::{ShiftState, Timestamp, VehicleId};

#[derive(Deserialize, Serialize, Debug)]
pub enum ErrorType {
    #[serde(rename = "vehicle_disconnected")]
    VehicleDisconnected,

    #[serde(rename = "vehicle_error")]
    VehicleError,

    #[serde(rename = "client_error")]
    ClientError,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "msg_type")]
pub enum ToServerStreamingMessage {
    #[serde(rename = "data:subscribe_oauth")]
    DataSubscribeOauth {
        token: String,
        value: String,
        tag: u64,
    },
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(tag = "msg_type")]
pub enum FromServerStreamingMessage {
    #[serde(rename = "control:hello")]
    ControlHello { connection_timeout: u64 },

    #[serde(rename = "data:update")]
    DataUpdate { tag: u64, value: String },

    #[serde(rename = "data:error")]
    DataError {
        tag: String,
        error_type: ErrorType,
        value: String,
    },
}

#[derive(Copy, Clone, Debug)]
//#[serde(rename_all = "snake_case")]
pub enum StreamingFields {
    Speed,
    Odometer,
    Soc,
    Elevation,
    EstHeading,
    EstLat,
    EstLng,
    Power,
    ShiftState,
    Range,
    EstRange,
    Heading,
}

impl FromStr for StreamingFields {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "speed" => Ok(Self::Speed),
            "odometer" => Ok(Self::Odometer),
            "soc" => Ok(Self::Soc),
            "elevation" => Ok(Self::Elevation),
            "est_heading" => Ok(Self::EstHeading),
            "est_lat" => Ok(Self::EstLat),
            "est_lng" => Ok(Self::EstLng),
            "power" => Ok(Self::Power),
            "shift_state" => Ok(Self::ShiftState),
            "range" => Ok(Self::Range),
            "est_range" => Ok(Self::EstRange),
            "heading" => Ok(Self::Heading),
            _ => Err(()),
        }
    }
}

impl ToString for StreamingFields {
    fn to_string(&self) -> String {
        match self {
            StreamingFields::Speed => "speed".to_string(),
            StreamingFields::Odometer => "odometer".to_string(),
            StreamingFields::Soc => "soc".to_string(),
            StreamingFields::Elevation => "elevation".to_string(),
            StreamingFields::EstHeading => "est_heading".to_string(),
            StreamingFields::EstLat => "est_lat".to_string(),
            StreamingFields::EstLng => "est_lng".to_string(),
            StreamingFields::Power => "power".to_string(),
            StreamingFields::ShiftState => "shift_state".to_string(),
            StreamingFields::Range => "range".to_string(),
            StreamingFields::EstRange => "est_range".to_string(),
            StreamingFields::Heading => "heading".to_string(),
        }
    }
}

#[derive(Debug, Clone)]
/// Struct representing streaming data from a vehicle.
pub struct StreamingData {
    /// The vehicle id.
    pub id: VehicleId,

    /// Unix timestamp in milliseconds.
    pub time: Timestamp,

    /// Speed in km per hour.
    pub speed: Option<u32>,

    /// Odometer reading in km.
    pub odometer: Option<f64>,

    /// State of charge as a percentage.
    pub soc: Option<u8>,

    /// Elevation in meters.
    pub elevation: Option<u32>,

    /// Estimated heading in degrees.
    pub est_heading: Option<u16>,

    /// Estimated latitude in decimal degrees.
    pub est_lat: Option<f64>,

    /// Estimated longitude in decimal degrees.
    pub est_lng: Option<f64>,

    /// Power usage in watts.
    pub power: Option<i32>,

    /// Shift state of the vehicle.
    pub shift_state: Option<ShiftState>,

    /// Estimated range in km.
    pub range: Option<f32>,

    /// Estimated range based on energy usage in km.
    pub est_range: Option<f32>,

    /// Heading in degrees.
    pub heading: Option<u16>,
}

impl StreamingData {
    pub fn new(id: VehicleId, time: Timestamp) -> Self {
        Self {
            id,
            time,
            speed: None,
            odometer: None,
            soc: None,
            elevation: None,
            est_heading: None,
            est_lat: None,
            est_lng: None,
            power: None,
            shift_state: None,
            range: None,
            est_range: None,
            heading: None,
        }
    }
}
