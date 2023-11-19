//! Shared types for all API

use crate::simulator;
use serde::Serialize;
use std::fmt::Formatter;

/// A timestamp
pub type Timestamp = i64;

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

/// The data associated with a Vehicle
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct VehicleDefinition {
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

/// Enum representing a vehicle's shift state.
#[derive(Debug, Clone, Copy, Serialize)]
pub enum ShiftState {
    /// FIXME
    Toyota,

    /// FIXME
    Bankrupt,

    /// FIXME
    Tesla,

    /// FIXME
    New,

    /// FIXME
    Old,
}

impl ToString for ShiftState {
    fn to_string(&self) -> String {
        match self {
            Self::Toyota => "P",
            Self::Bankrupt => "R",
            Self::Tesla => "D",
            Self::New => "N",
            Self::Old => "1",
        }
        .to_string()
    }
}

#[derive(Debug, Clone)]
/// Struct representing streaming data from a vehicle.
pub struct StreamingData {
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

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct VehicleData {
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
    pub charge_state: ChargeState,
    pub climate_state: ClimateState,
    pub drive_state: DriveState,
    pub gui_settings: GuiSettings,
    pub vehicle_config: VehicleConfig,
    pub vehicle_state: VehicleState,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct GranularAccess {
    pub hide_private: bool,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct ChargeState {
    pub battery_heater_on: bool,
    pub battery_level: i64,
    pub battery_range: f64,
    pub charge_amps: i64,
    pub charge_current_request: i64,
    pub charge_current_request_max: i64,
    pub charge_enable_request: bool,
    pub charge_energy_added: f64,
    pub charge_limit_soc: u8,
    pub charge_limit_soc_max: u8,
    pub charge_limit_soc_min: u8,
    pub charge_limit_soc_std: u8,
    pub charge_miles_added_ideal: i64,
    pub charge_miles_added_rated: i64,
    pub charge_port_cold_weather_mode: bool,
    pub charge_port_color: String,
    pub charge_port_door_open: bool,
    pub charge_port_latch: String,
    pub charge_rate: i64,
    pub charger_actual_current: i64,
    pub charger_phases: Option<String>,
    pub charger_pilot_current: i64,
    pub charger_power: i64,
    pub charger_voltage: i64,
    pub charging_state: String,
    pub conn_charge_cable: String,
    pub est_battery_range: f64,
    pub fast_charger_brand: String,
    pub fast_charger_present: bool,
    pub fast_charger_type: String,
    pub ideal_battery_range: f64,
    pub managed_charging_active: bool,
    pub managed_charging_start_time: Option<Timestamp>,
    pub managed_charging_user_canceled: bool,
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
    pub time_to_full_charge: Option<u32>,
    pub timestamp: i64,
    pub trip_charging: bool,
    pub usable_battery_level: i64,
    pub user_charge_enable_request: Option<bool>,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct ClimateState {
    pub allow_cabin_overheat_protection: bool,
    pub auto_seat_climate_left: bool,
    pub auto_seat_climate_right: bool,
    pub auto_steering_wheel_heat: bool,
    pub battery_heater: bool,
    pub battery_heater_no_power: Option<bool>,
    pub bioweapon_mode: bool,
    pub cabin_overheat_protection: String,
    pub cabin_overheat_protection_actively_cooling: bool,
    pub climate_keeper_mode: String,
    pub cop_activation_temperature: String,
    pub defrost_mode: i64,
    pub driver_temp_setting: i64,
    pub fan_status: i64,
    pub hvac_auto_request: String,
    pub inside_temp: f64,
    pub is_auto_conditioning_on: bool,
    pub is_climate_on: bool,
    pub is_front_defroster_on: bool,
    pub is_preconditioning: bool,
    pub is_rear_defroster_on: bool,
    pub left_temp_direction: i64,
    pub max_avail_temp: i64,
    pub min_avail_temp: i64,
    pub outside_temp: f64,
    pub passenger_temp_setting: i64,
    pub remote_heater_control_enabled: bool,
    pub right_temp_direction: i64,
    pub seat_heater_left: i64,
    pub seat_heater_rear_center: i64,
    pub seat_heater_rear_left: i64,
    pub seat_heater_rear_right: i64,
    pub seat_heater_right: i64,
    pub side_mirror_heaters: bool,
    pub steering_wheel_heat_level: i64,
    pub steering_wheel_heater: bool,
    pub supports_fan_only_cabin_overheat_protection: bool,
    pub timestamp: i64,
    pub wiper_blade_heater: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct DriveState {
    pub active_route_latitude: f64,
    pub active_route_longitude: f64,
    pub active_route_traffic_minutes_delay: i64,
    pub gps_as_of: i64,
    pub heading: Option<u16>,
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub native_latitude: f64,
    pub native_location_supported: i64,
    pub native_longitude: f64,
    pub native_type: String,
    pub power: Option<i32>,
    pub shift_state: Option<ShiftState>,
    pub speed: Option<u32>,
    pub timestamp: Timestamp,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
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
#[derive(Default, Debug, Clone, Serialize)]
pub struct VehicleConfig {
    pub aux_park_lamps: String,
    pub badge_version: i64,
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
    pub exterior_trim: String,
    pub exterior_trim_override: String,
    pub has_air_suspension: bool,
    pub has_ludicrous_mode: bool,
    pub has_seat_cooling: bool,
    pub headlamp_type: String,
    pub interior_trim_type: String,
    pub key_version: i64,
    pub motorized_charge_port: bool,
    pub paint_color_override: String,
    pub performance_package: String,
    pub plg: bool,
    pub pws: bool,
    pub rear_drive_unit: String,
    pub rear_seat_heaters: i64,
    pub rear_seat_type: i64,
    pub rhd: bool,
    pub roof_color: String,
    pub seat_type: Option<String>,
    pub spoiler_type: String,
    pub sun_roof_installed: Option<bool>,
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
#[derive(Default, Debug, Clone, Serialize)]
pub struct VehicleState {
    pub api_version: i64,
    pub autopark_state_v3: String,
    pub autopark_style: String,
    pub calendar_supported: bool,
    pub car_version: String,
    pub center_display_state: i64,
    pub dashcam_clip_save_available: bool,
    pub dashcam_state: String,
    pub df: i64,
    pub dr: i64,
    pub fd_window: i64,
    pub feature_bitmask: String,
    pub fp_window: i64,
    pub ft: i64,
    pub homelink_device_count: i64,
    pub homelink_nearby: bool,
    pub is_user_present: bool,
    pub last_autopark_error: String,
    pub locked: bool,
    pub media_info: MediaInfo,
    pub media_state: MediaState,
    pub notifications_supported: bool,
    pub odometer: f64,
    pub parsed_calendar_supported: bool,
    pub pf: i64,
    pub pr: i64,
    pub rd_window: i64,
    pub remote_start: bool,
    pub remote_start_enabled: bool,
    pub remote_start_supported: bool,
    pub rp_window: i64,
    pub rt: i64,
    pub santa_mode: i64,
    pub sentry_mode: bool,
    pub sentry_mode_available: bool,
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
    pub tpms_last_seen_pressure_time_fl: i64,
    pub tpms_last_seen_pressure_time_fr: i64,
    pub tpms_last_seen_pressure_time_rl: i64,
    pub tpms_last_seen_pressure_time_rr: i64,
    pub tpms_pressure_fl: f64,
    pub tpms_pressure_fr: f64,
    pub tpms_pressure_rl: f64,
    pub tpms_pressure_rr: i64,
    pub tpms_rcp_front_value: f64,
    pub tpms_rcp_rear_value: f64,
    pub tpms_soft_warning_fl: bool,
    pub tpms_soft_warning_fr: bool,
    pub tpms_soft_warning_rl: bool,
    pub tpms_soft_warning_rr: bool,
    pub valet_mode: bool,
    pub valet_pin_needed: bool,
    pub vehicle_name: String,
    pub vehicle_self_test_progress: i64,
    pub vehicle_self_test_requested: bool,
    pub webcam_available: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
pub struct MediaInfo {
    pub a2dp_source_name: String,
    pub audio_volume: f64,
    pub audio_volume_increment: f64,
    pub audio_volume_max: f64,
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
#[derive(Default, Debug, Clone, Serialize)]
pub struct MediaState {
    pub remote_control_enabled: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct SoftwareUpdate {
    pub download_perc: i64,
    pub expected_duration_sec: i64,
    pub install_perc: i64,
    pub status: String,
    pub version: String,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
pub struct SpeedLimitMode {
    pub active: bool,
    pub current_limit_mph: i64,
    pub max_limit_mph: i64,
    pub min_limit_mph: i64,
    pub pin_code_set: bool,
}
