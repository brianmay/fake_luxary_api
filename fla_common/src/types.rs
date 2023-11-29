//! Shared types for all API

use std::{convert::Infallible, num::ParseIntError, str::FromStr};

use serde::{Deserialize, Serialize, Serializer};
use serde_enum_str::{Deserialize_enum_str, Serialize_enum_str};

use crate::simulator::SimulationStateEnum;

/// A timestamp
pub type Timestamp = i64;

/// A vehicle ID
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq)]
pub struct VehicleId(u64);

impl FromStr for VehicleId {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl ToString for VehicleId {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl VehicleId {
    /// Create a new VehicleId
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self(id)
    }

    // /// Get the ID
    // #[must_use]
    // pub fn to_u64(&self) -> u64 {
    //     self.0
    // }
}

/// A extended vehicle ID
#[derive(Debug, Serialize, Deserialize, Clone, Copy, Eq, PartialEq, Hash)]
pub struct VehicleGuid(u64);

impl FromStr for VehicleGuid {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Self(s.parse()?))
    }
}

impl ToString for VehicleGuid {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

impl VehicleGuid {
    /// Create a new VehicleGuid
    #[must_use]
    pub fn new(id: u64) -> Self {
        Self(id)
    }
}

/// Enum representing a vehicle's shift state.
#[derive(Deserialize_enum_str, Serialize_enum_str, Clone, Eq, PartialEq, Debug)]
#[serde(rename_all = "snake_case")]
pub enum VehicleStateEnum {
    // Offline state
    Offline,

    // Online state
    Online,

    /// Unknown shift state
    #[serde(other)]
    Unknown(String),
}

impl From<SimulationStateEnum> for VehicleStateEnum {
    fn from(state: SimulationStateEnum) -> Self {
        match state {
            SimulationStateEnum::Driving => Self::Online,
            SimulationStateEnum::Charging => Self::Online,
            SimulationStateEnum::Idle => Self::Online,
            SimulationStateEnum::IdleNoSleep => Self::Online,
            SimulationStateEnum::Sleeping => Self::Offline,
        }
    }
}

/// The data associated with a Vehicle
#[allow(dead_code)]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct VehicleDefinition {
    /// Vehicle ID for owner-api endpoint.
    pub id: VehicleId,

    /// Vehicle ID for streaming or Auto park API.
    pub vehicle_id: VehicleGuid,

    /// Vehicle identification number.
    pub vin: String,

    /// Vehicle display name.
    pub display_name: String,

    /// Vehicle option codes.
    pub option_codes: Option<String>,

    /// Vehicle color.
    pub color: Option<String>,

    /// Vehicle tokens.
    pub tokens: Vec<String>,

    /// Vehicle state.
    pub state: VehicleStateEnum,

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

/// Enum representing a vehicle's shift state.
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ShiftState {
    // Park state
    Park,

    // Drive state
    Drive,

    // Reverse state
    Reverse,

    /// Unknown shift state
    Unknown(String),
}

impl Serialize for ShiftState {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        serializer.serialize_str(&self.to_string())
    }
}

impl<'de> Deserialize<'de> for ShiftState {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        Ok(Self::from_str(&s).unwrap_or(Self::Unknown(s)))
    }
}

impl FromStr for ShiftState {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "P" => Ok(Self::Park),
            "D" => Ok(Self::Drive),
            "R" => Ok(Self::Reverse),
            _ => Ok(Self::Unknown(s.to_string())),
        }
    }
}

/// Required for streaming.
impl ToString for ShiftState {
    fn to_string(&self) -> String {
        match self {
            Self::Park => "P",
            Self::Drive => "D",
            Self::Reverse => "R",
            Self::Unknown(s) => s,
        }
        .to_string()
    }
}

/// Is the car currently charging?
#[derive(Deserialize_enum_str, Serialize_enum_str, Clone, Eq, PartialEq, Debug)]
pub enum ChargingStateEnum {
    /// Charging is starting
    Starting,

    /// Charging is complete
    Complete,

    /// Charging is in progress
    Charging,

    /// Charging is not in progress and we are disconnected
    Disconnected,

    /// Charging is not in progress
    Stopped,

    /// Charger cable is connected but not getting power
    NoPower,

    /// Unknown charging state
    #[serde(other)]
    Unknown(String),
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct GranularAccess {
    pub hide_private: bool,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChargeState {
    pub battery_heater_on: bool,
    pub battery_level: u8,
    pub battery_range: f32,
    pub charge_amps: i64,
    pub charge_current_request: i64,
    pub charge_current_request_max: i64,
    pub charge_enable_request: bool,
    pub charge_energy_added: f32,
    pub charge_limit_soc: u8,
    pub charge_limit_soc_max: u8,
    pub charge_limit_soc_min: u8,
    pub charge_limit_soc_std: u8,
    pub charge_miles_added_ideal: f32,
    pub charge_miles_added_rated: f32,
    pub charge_port_cold_weather_mode: Option<bool>,
    pub charge_port_color: String,
    pub charge_port_door_open: bool,
    pub charge_port_latch: String,
    pub charge_rate: Option<f32>,
    pub charger_actual_current: i64,
    pub charger_phases: Option<u8>,
    pub charger_pilot_current: i64,
    pub charger_power: i64,
    pub charger_voltage: i64,
    pub charging_state: ChargingStateEnum,
    pub conn_charge_cable: String,
    pub est_battery_range: f32,
    pub fast_charger_brand: String,
    pub fast_charger_present: bool,
    pub fast_charger_type: String,
    pub ideal_battery_range: f32,
    pub managed_charging_active: Option<bool>,
    pub managed_charging_start_time: Option<Timestamp>,
    pub managed_charging_user_canceled: Option<bool>,
    pub max_range_charge_counter: i64,
    pub minutes_to_full_charge: i64,
    pub not_enough_power_to_heat: Option<bool>,
    pub off_peak_charging_enabled: bool,
    pub off_peak_charging_times: String,
    pub off_peak_hours_end_time: i64,
    pub preconditioning_enabled: bool,
    pub preconditioning_times: String,
    pub scheduled_charging_mode: String,
    pub scheduled_charging_pending: bool,
    pub scheduled_charging_start_time: Option<Timestamp>,
    pub scheduled_departure_time: Timestamp,
    pub scheduled_departure_time_minutes: i64,
    pub supercharger_session_trip_planner: bool,
    pub time_to_full_charge: Option<f64>,
    pub timestamp: i64,
    pub trip_charging: bool,
    pub usable_battery_level: i64,
    pub user_charge_enable_request: Option<bool>,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct ClimateState {
    pub allow_cabin_overheat_protection: bool,
    pub auto_seat_climate_left: Option<bool>,
    pub auto_seat_climate_right: Option<bool>,
    pub auto_steering_wheel_heat: Option<bool>,
    pub battery_heater: bool,
    pub battery_heater_no_power: Option<bool>,
    pub bioweapon_mode: bool,
    pub cabin_overheat_protection: String,
    pub cabin_overheat_protection_actively_cooling: Option<bool>,
    pub climate_keeper_mode: String,
    pub cop_activation_temperature: String,
    pub defrost_mode: i64,
    pub driver_temp_setting: f32,
    pub fan_status: i64,
    pub hvac_auto_request: String,
    pub inside_temp: f32,
    pub is_auto_conditioning_on: bool,
    pub is_climate_on: bool,
    pub is_front_defroster_on: bool,
    pub is_preconditioning: bool,
    pub is_rear_defroster_on: bool,
    pub left_temp_direction: i64,
    pub max_avail_temp: f32,
    pub min_avail_temp: f32,
    pub outside_temp: f32,
    pub passenger_temp_setting: f32,
    pub remote_heater_control_enabled: bool,
    pub right_temp_direction: i64,
    pub seat_heater_left: i64,
    pub seat_heater_rear_center: i64,
    pub seat_heater_rear_left: i64,
    pub seat_heater_rear_right: i64,
    pub seat_heater_right: i64,
    pub side_mirror_heaters: bool,
    pub steering_wheel_heat_level: Option<i64>,
    pub steering_wheel_heater: bool,
    pub supports_fan_only_cabin_overheat_protection: bool,
    pub timestamp: i64,
    pub wiper_blade_heater: bool,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DriveState {
    pub active_route_latitude: f64,
    pub active_route_longitude: f64,
    pub active_route_traffic_minutes_delay: f32,
    pub gps_as_of: Timestamp,
    pub heading: u16,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub native_latitude: Option<f64>,
    pub native_location_supported: i64,
    pub native_longitude: Option<f64>,
    pub native_type: String,
    pub power: Option<i32>,
    pub shift_state: Option<ShiftState>,
    pub speed: Option<f32>,
    pub timestamp: Timestamp,
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GuiSettings {
    pub gui_24_hour_time: bool,
    pub gui_charge_rate_units: String,
    pub gui_distance_units: String,
    pub gui_range_display: String,
    pub gui_temperature_units: String,
    pub gui_tirepressure_units: String,
    pub show_range_units: bool,
    pub timestamp: i64,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleConfig {
    pub aux_park_lamps: Option<String>,
    pub badge_version: Option<i64>,
    pub can_accept_navigation_requests: bool,
    pub can_actuate_trunks: bool,
    pub car_special_type: String,
    pub car_type: String,
    pub charge_port_type: String,
    pub cop_user_set_temp_supported: bool,
    pub dashcam_clip_save_supported: bool,
    pub default_charge_to_max: bool,
    pub driver_assist: String,
    pub ece_restrictions: bool,
    pub efficiency_package: String,
    pub eu_vehicle: bool,
    pub exterior_color: String,
    pub exterior_trim: Option<String>,
    pub exterior_trim_override: String,
    pub has_air_suspension: bool,
    pub has_ludicrous_mode: bool,
    pub has_seat_cooling: bool,
    pub headlamp_type: String,
    pub interior_trim_type: String,
    pub key_version: Option<u8>,
    pub motorized_charge_port: bool,
    pub paint_color_override: String,
    pub performance_package: Option<String>,
    pub plg: bool,
    pub pws: bool,
    pub rear_drive_unit: String,
    pub rear_seat_heaters: i64,
    pub rear_seat_type: i64,
    pub rhd: bool,
    pub roof_color: String,
    pub seat_type: Option<i8>,
    pub spoiler_type: String,
    pub sun_roof_installed: Option<u8>,
    pub supports_qr_pairing: bool,
    pub third_row_seats: String,
    pub timestamp: i64,
    pub trim_badging: String,
    pub use_range_badging: bool,
    pub utc_offset: i64,
    pub webcam_selfie_supported: bool,
    pub webcam_supported: bool,
    pub wheel_type: String,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct VehicleState {
    pub api_version: i64,
    pub autopark_state_v3: Option<String>,
    pub autopark_style: String,
    pub calendar_supported: bool,
    pub car_version: String,
    pub center_display_state: i64,
    pub dashcam_clip_save_available: bool,
    pub dashcam_state: String,
    pub df: u8,
    pub dr: u8,
    pub fd_window: i64,
    pub feature_bitmask: String,
    pub fp_window: i64,
    pub ft: u8,
    pub homelink_device_count: Option<u8>,
    pub homelink_nearby: Option<bool>,
    pub is_user_present: bool,
    pub last_autopark_error: String,
    pub locked: bool,
    pub media_info: MediaInfo,
    pub media_state: MediaState,
    pub notifications_supported: bool,
    pub odometer: f32,
    pub parsed_calendar_supported: bool,
    pub pf: u8,
    pub pr: u8,
    pub rd_window: i64,
    pub remote_start: bool,
    pub remote_start_enabled: bool,
    pub remote_start_supported: bool,
    pub rp_window: i64,
    pub rt: u8,
    pub santa_mode: i64,
    pub sentry_mode: Option<bool>,
    pub sentry_mode_available: Option<bool>,
    pub service_mode: bool,
    pub service_mode_plus: bool,
    pub smart_summon_available: bool,
    pub software_update: SoftwareUpdate,
    pub speed_limit_mode: SpeedLimitMode,
    pub summon_standby_mode_enabled: bool,
    pub timestamp: i64,
    pub tpms_hard_warning_fl: bool,
    pub tpms_hard_warning_fr: bool,
    pub tpms_hard_warning_rl: bool,
    pub tpms_hard_warning_rr: bool,
    pub tpms_last_seen_pressure_time_fl: Option<Timestamp>,
    pub tpms_last_seen_pressure_time_fr: Option<Timestamp>,
    pub tpms_last_seen_pressure_time_rl: Option<Timestamp>,
    pub tpms_last_seen_pressure_time_rr: Option<Timestamp>,
    pub tpms_pressure_fl: f32,
    pub tpms_pressure_fr: f32,
    pub tpms_pressure_rl: f32,
    pub tpms_pressure_rr: f32,
    pub tpms_rcp_front_value: f32,
    pub tpms_rcp_rear_value: f32,
    pub tpms_soft_warning_fl: bool,
    pub tpms_soft_warning_fr: bool,
    pub tpms_soft_warning_rl: bool,
    pub tpms_soft_warning_rr: bool,
    pub valet_mode: bool,
    pub valet_pin_needed: bool,
    pub vehicle_name: Option<String>,
    pub vehicle_self_test_progress: Option<i64>,
    pub vehicle_self_test_requested: Option<bool>,
    pub webcam_available: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
pub struct MediaInfo {
    pub a2dp_source_name: String,
    pub audio_volume: f32,
    pub audio_volume_increment: f32,
    pub audio_volume_max: f32,
    pub media_playback_status: String,
    pub now_playing_album: String,
    pub now_playing_artist: String,
    pub now_playing_duration: i64,
    pub now_playing_elapsed: i64,
    pub now_playing_source: String,
    pub now_playing_station: String,
    pub now_playing_title: String,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct MediaState {
    pub remote_control_enabled: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SoftwareUpdate {
    pub download_perc: i64,
    pub expected_duration_sec: i64,
    pub install_perc: i64,
    pub status: String,
    pub version: String,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub struct SpeedLimitMode {
    pub active: bool,
    pub current_limit_mph: f32,
    pub max_limit_mph: f32,
    pub min_limit_mph: f32,
    pub pin_code_set: bool,
}

/// The possible endpoints for vehicle data
#[derive(Eq, PartialEq, Hash)]
pub enum VehicleDataEndpoint {
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

impl FromStr for VehicleDataEndpoint {
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

impl ToString for VehicleDataEndpoint {
    fn to_string(&self) -> String {
        match self {
            Self::ChargeState => "charge_state",
            Self::ClimateState => "climate_state",
            Self::ClosuresState => "closures_state",
            Self::DriveState => "drive_state",
            Self::GuiSettings => "gui_settings",
            Self::LocationData => "location_data",
            Self::VehicleConfig => "vehicle_config",
            Self::VehicleState => "vehicle_state",
            Self::VehicleDataCombo => "vehicle_data_combo",
        }
        .to_string()
    }
}

#[allow(missing_docs)]
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VehicleData {
    pub id: VehicleId,
    pub user_id: i64,
    pub vehicle_id: VehicleGuid,
    pub vin: String,
    pub color: Option<String>,
    pub access_type: String,
    pub granular_access: GranularAccess,
    pub tokens: Vec<String>,
    pub state: VehicleStateEnum,
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
#[derive(Serialize, Deserialize, Debug)]
pub struct VehicleDataQuery {
    /// List of endpoints to retrieve
    pub endpoints: Option<String>,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_serialize_shift_state() {
        let serialized = serde_json::to_string(&ShiftState::Drive).unwrap();
        assert_eq!(serialized, "\"D\"");
    }

    #[test]
    fn test_serialize_shift_state_unknown() {
        let shift_state = ShiftState::Unknown("XX".to_string());
        let serialized = serde_json::to_string(&shift_state).unwrap();
        assert_eq!(serialized, "\"XX\"");
    }

    #[test]
    fn test_deserialize_shift_state() {
        let deserialized: ShiftState = serde_json::from_str("\"D\"").unwrap();
        assert_eq!(deserialized, ShiftState::Drive);
    }

    #[test]
    fn test_deserialize_shift_state_unknown() {
        let deserialized: ShiftState = serde_json::from_str("\"XX\"").unwrap();
        assert_eq!(deserialized, ShiftState::Unknown("XX".to_string()));
    }
}
