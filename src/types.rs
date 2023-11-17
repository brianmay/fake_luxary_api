//! Shared types for all API

use std::fmt::Formatter;

use serde::Serialize;

use crate::simulator;

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

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleData {
    pub id: i64,
    #[serde(rename = "user_id")]
    pub user_id: i64,
    #[serde(rename = "vehicle_id")]
    pub vehicle_id: i64,
    pub vin: String,
    pub color: Option<String>,
    #[serde(rename = "access_type")]
    pub access_type: String,
    #[serde(rename = "granular_access")]
    pub granular_access: GranularAccess,
    pub tokens: Vec<String>,
    pub state: Option<String>,
    #[serde(rename = "in_service")]
    pub in_service: bool,
    #[serde(rename = "id_s")]
    pub id_s: String,
    #[serde(rename = "calendar_enabled")]
    pub calendar_enabled: bool,
    #[serde(rename = "api_version")]
    pub api_version: i64,
    #[serde(rename = "backseat_token")]
    pub backseat_token: Option<String>,
    #[serde(rename = "backseat_token_updated_at")]
    pub backseat_token_updated_at: Option<u64>,
    #[serde(rename = "charge_state")]
    pub charge_state: ChargeState,
    #[serde(rename = "climate_state")]
    pub climate_state: ClimateState,
    #[serde(rename = "drive_state")]
    pub drive_state: DriveState,
    #[serde(rename = "gui_settings")]
    pub gui_settings: GuiSettings,
    #[serde(rename = "vehicle_config")]
    pub vehicle_config: VehicleConfig,
    #[serde(rename = "vehicle_state")]
    pub vehicle_state: VehicleState,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GranularAccess {
    #[serde(rename = "hide_private")]
    pub hide_private: bool,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ChargeState {
    #[serde(rename = "battery_heater_on")]
    pub battery_heater_on: bool,
    #[serde(rename = "battery_level")]
    pub battery_level: i64,
    #[serde(rename = "battery_range")]
    pub battery_range: f64,
    #[serde(rename = "charge_amps")]
    pub charge_amps: i64,
    #[serde(rename = "charge_current_request")]
    pub charge_current_request: i64,
    #[serde(rename = "charge_current_request_max")]
    pub charge_current_request_max: i64,
    #[serde(rename = "charge_enable_request")]
    pub charge_enable_request: bool,
    #[serde(rename = "charge_energy_added")]
    pub charge_energy_added: f64,
    #[serde(rename = "charge_limit_soc")]
    pub charge_limit_soc: i64,
    #[serde(rename = "charge_limit_soc_max")]
    pub charge_limit_soc_max: i64,
    #[serde(rename = "charge_limit_soc_min")]
    pub charge_limit_soc_min: i64,
    #[serde(rename = "charge_limit_soc_std")]
    pub charge_limit_soc_std: i64,
    #[serde(rename = "charge_miles_added_ideal")]
    pub charge_miles_added_ideal: i64,
    #[serde(rename = "charge_miles_added_rated")]
    pub charge_miles_added_rated: i64,
    #[serde(rename = "charge_port_cold_weather_mode")]
    pub charge_port_cold_weather_mode: bool,
    #[serde(rename = "charge_port_color")]
    pub charge_port_color: String,
    #[serde(rename = "charge_port_door_open")]
    pub charge_port_door_open: bool,
    #[serde(rename = "charge_port_latch")]
    pub charge_port_latch: String,
    #[serde(rename = "charge_rate")]
    pub charge_rate: i64,
    #[serde(rename = "charger_actual_current")]
    pub charger_actual_current: i64,
    #[serde(rename = "charger_phases")]
    pub charger_phases: Option<String>,
    #[serde(rename = "charger_pilot_current")]
    pub charger_pilot_current: i64,
    #[serde(rename = "charger_power")]
    pub charger_power: i64,
    #[serde(rename = "charger_voltage")]
    pub charger_voltage: i64,
    #[serde(rename = "charging_state")]
    pub charging_state: String,
    #[serde(rename = "conn_charge_cable")]
    pub conn_charge_cable: String,
    #[serde(rename = "est_battery_range")]
    pub est_battery_range: f64,
    #[serde(rename = "fast_charger_brand")]
    pub fast_charger_brand: String,
    #[serde(rename = "fast_charger_present")]
    pub fast_charger_present: bool,
    #[serde(rename = "fast_charger_type")]
    pub fast_charger_type: String,
    #[serde(rename = "ideal_battery_range")]
    pub ideal_battery_range: f64,
    #[serde(rename = "managed_charging_active")]
    pub managed_charging_active: bool,
    #[serde(rename = "managed_charging_start_time")]
    pub managed_charging_start_time: Option<u64>,
    #[serde(rename = "managed_charging_user_canceled")]
    pub managed_charging_user_canceled: bool,
    #[serde(rename = "max_range_charge_counter")]
    pub max_range_charge_counter: i64,
    #[serde(rename = "minutes_to_full_charge")]
    pub minutes_to_full_charge: i64,
    #[serde(rename = "not_enough_power_to_heat")]
    pub not_enough_power_to_heat: Option<bool>,
    #[serde(rename = "off_peak_charging_enabled")]
    pub off_peak_charging_enabled: bool,
    #[serde(rename = "off_peak_charging_times")]
    pub off_peak_charging_times: String,
    #[serde(rename = "off_peak_hours_end_time")]
    pub off_peak_hours_end_time: i64,
    #[serde(rename = "preconditioning_enabled")]
    pub preconditioning_enabled: bool,
    #[serde(rename = "preconditioning_times")]
    pub preconditioning_times: String,
    #[serde(rename = "scheduled_charging_mode")]
    pub scheduled_charging_mode: String,
    #[serde(rename = "scheduled_charging_pending")]
    pub scheduled_charging_pending: bool,
    #[serde(rename = "scheduled_charging_start_time")]
    pub scheduled_charging_start_time: Option<u64>,
    #[serde(rename = "scheduled_departure_time")]
    pub scheduled_departure_time: u64,
    #[serde(rename = "scheduled_departure_time_minutes")]
    pub scheduled_departure_time_minutes: i64,
    #[serde(rename = "supercharger_session_trip_planner")]
    pub supercharger_session_trip_planner: bool,
    #[serde(rename = "time_to_full_charge")]
    pub time_to_full_charge: i64,
    pub timestamp: i64,
    #[serde(rename = "trip_charging")]
    pub trip_charging: bool,
    #[serde(rename = "usable_battery_level")]
    pub usable_battery_level: i64,
    #[serde(rename = "user_charge_enable_request")]
    pub user_charge_enable_request: Option<bool>,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ClimateState {
    #[serde(rename = "allow_cabin_overheat_protection")]
    pub allow_cabin_overheat_protection: bool,
    #[serde(rename = "auto_seat_climate_left")]
    pub auto_seat_climate_left: bool,
    #[serde(rename = "auto_seat_climate_right")]
    pub auto_seat_climate_right: bool,
    #[serde(rename = "auto_steering_wheel_heat")]
    pub auto_steering_wheel_heat: bool,
    #[serde(rename = "battery_heater")]
    pub battery_heater: bool,
    #[serde(rename = "battery_heater_no_power")]
    pub battery_heater_no_power: Option<bool>,
    #[serde(rename = "bioweapon_mode")]
    pub bioweapon_mode: bool,
    #[serde(rename = "cabin_overheat_protection")]
    pub cabin_overheat_protection: String,
    #[serde(rename = "cabin_overheat_protection_actively_cooling")]
    pub cabin_overheat_protection_actively_cooling: bool,
    #[serde(rename = "climate_keeper_mode")]
    pub climate_keeper_mode: String,
    #[serde(rename = "cop_activation_temperature")]
    pub cop_activation_temperature: String,
    #[serde(rename = "defrost_mode")]
    pub defrost_mode: i64,
    #[serde(rename = "driver_temp_setting")]
    pub driver_temp_setting: i64,
    #[serde(rename = "fan_status")]
    pub fan_status: i64,
    #[serde(rename = "hvac_auto_request")]
    pub hvac_auto_request: String,
    #[serde(rename = "inside_temp")]
    pub inside_temp: f64,
    #[serde(rename = "is_auto_conditioning_on")]
    pub is_auto_conditioning_on: bool,
    #[serde(rename = "is_climate_on")]
    pub is_climate_on: bool,
    #[serde(rename = "is_front_defroster_on")]
    pub is_front_defroster_on: bool,
    #[serde(rename = "is_preconditioning")]
    pub is_preconditioning: bool,
    #[serde(rename = "is_rear_defroster_on")]
    pub is_rear_defroster_on: bool,
    #[serde(rename = "left_temp_direction")]
    pub left_temp_direction: i64,
    #[serde(rename = "max_avail_temp")]
    pub max_avail_temp: i64,
    #[serde(rename = "min_avail_temp")]
    pub min_avail_temp: i64,
    #[serde(rename = "outside_temp")]
    pub outside_temp: f64,
    #[serde(rename = "passenger_temp_setting")]
    pub passenger_temp_setting: i64,
    #[serde(rename = "remote_heater_control_enabled")]
    pub remote_heater_control_enabled: bool,
    #[serde(rename = "right_temp_direction")]
    pub right_temp_direction: i64,
    #[serde(rename = "seat_heater_left")]
    pub seat_heater_left: i64,
    #[serde(rename = "seat_heater_rear_center")]
    pub seat_heater_rear_center: i64,
    #[serde(rename = "seat_heater_rear_left")]
    pub seat_heater_rear_left: i64,
    #[serde(rename = "seat_heater_rear_right")]
    pub seat_heater_rear_right: i64,
    #[serde(rename = "seat_heater_right")]
    pub seat_heater_right: i64,
    #[serde(rename = "side_mirror_heaters")]
    pub side_mirror_heaters: bool,
    #[serde(rename = "steering_wheel_heat_level")]
    pub steering_wheel_heat_level: i64,
    #[serde(rename = "steering_wheel_heater")]
    pub steering_wheel_heater: bool,
    #[serde(rename = "supports_fan_only_cabin_overheat_protection")]
    pub supports_fan_only_cabin_overheat_protection: bool,
    pub timestamp: i64,
    #[serde(rename = "wiper_blade_heater")]
    pub wiper_blade_heater: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DriveState {
    #[serde(rename = "active_route_latitude")]
    pub active_route_latitude: f64,
    #[serde(rename = "active_route_longitude")]
    pub active_route_longitude: f64,
    #[serde(rename = "active_route_traffic_minutes_delay")]
    pub active_route_traffic_minutes_delay: i64,
    #[serde(rename = "gps_as_of")]
    pub gps_as_of: i64,
    pub heading: i64,
    pub latitude: f64,
    pub longitude: f64,
    #[serde(rename = "native_latitude")]
    pub native_latitude: f64,
    #[serde(rename = "native_location_supported")]
    pub native_location_supported: i64,
    #[serde(rename = "native_longitude")]
    pub native_longitude: f64,
    #[serde(rename = "native_type")]
    pub native_type: String,
    pub power: i64,
    #[serde(rename = "shift_state")]
    pub shift_state: Option<String>,
    pub speed: Option<u32>,
    pub timestamp: u64,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GuiSettings {
    #[serde(rename = "gui_24_hour_time")]
    pub gui_24_hour_time: bool,
    #[serde(rename = "gui_charge_rate_units")]
    pub gui_charge_rate_units: String,
    #[serde(rename = "gui_distance_units")]
    pub gui_distance_units: String,
    #[serde(rename = "gui_range_display")]
    pub gui_range_display: String,
    #[serde(rename = "gui_temperature_units")]
    pub gui_temperature_units: String,
    #[serde(rename = "gui_tirepressure_units")]
    pub gui_tirepressure_units: String,
    #[serde(rename = "show_range_units")]
    pub show_range_units: bool,
    pub timestamp: i64,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleConfig {
    #[serde(rename = "aux_park_lamps")]
    pub aux_park_lamps: String,
    #[serde(rename = "badge_version")]
    pub badge_version: i64,
    #[serde(rename = "can_accept_navigation_requests")]
    pub can_accept_navigation_requests: bool,
    #[serde(rename = "can_actuate_trunks")]
    pub can_actuate_trunks: bool,
    #[serde(rename = "car_special_type")]
    pub car_special_type: String,
    #[serde(rename = "car_type")]
    pub car_type: String,
    #[serde(rename = "charge_port_type")]
    pub charge_port_type: String,
    #[serde(rename = "cop_user_set_temp_supported")]
    pub cop_user_set_temp_supported: bool,
    #[serde(rename = "dashcam_clip_save_supported")]
    pub dashcam_clip_save_supported: bool,
    #[serde(rename = "default_charge_to_max")]
    pub default_charge_to_max: bool,
    #[serde(rename = "driver_assist")]
    pub driver_assist: String,
    #[serde(rename = "ece_restrictions")]
    pub ece_restrictions: bool,
    #[serde(rename = "efficiency_package")]
    pub efficiency_package: String,
    #[serde(rename = "eu_vehicle")]
    pub eu_vehicle: bool,
    #[serde(rename = "exterior_color")]
    pub exterior_color: String,
    #[serde(rename = "exterior_trim")]
    pub exterior_trim: String,
    #[serde(rename = "exterior_trim_override")]
    pub exterior_trim_override: String,
    #[serde(rename = "has_air_suspension")]
    pub has_air_suspension: bool,
    #[serde(rename = "has_ludicrous_mode")]
    pub has_ludicrous_mode: bool,
    #[serde(rename = "has_seat_cooling")]
    pub has_seat_cooling: bool,
    #[serde(rename = "headlamp_type")]
    pub headlamp_type: String,
    #[serde(rename = "interior_trim_type")]
    pub interior_trim_type: String,
    #[serde(rename = "key_version")]
    pub key_version: i64,
    #[serde(rename = "motorized_charge_port")]
    pub motorized_charge_port: bool,
    #[serde(rename = "paint_color_override")]
    pub paint_color_override: String,
    #[serde(rename = "performance_package")]
    pub performance_package: String,
    pub plg: bool,
    pub pws: bool,
    #[serde(rename = "rear_drive_unit")]
    pub rear_drive_unit: String,
    #[serde(rename = "rear_seat_heaters")]
    pub rear_seat_heaters: i64,
    #[serde(rename = "rear_seat_type")]
    pub rear_seat_type: i64,
    pub rhd: bool,
    #[serde(rename = "roof_color")]
    pub roof_color: String,
    #[serde(rename = "seat_type")]
    pub seat_type: Option<String>,
    #[serde(rename = "spoiler_type")]
    pub spoiler_type: String,
    #[serde(rename = "sun_roof_installed")]
    pub sun_roof_installed: Option<bool>,
    #[serde(rename = "supports_qr_pairing")]
    pub supports_qr_pairing: bool,
    #[serde(rename = "third_row_seats")]
    pub third_row_seats: String,
    pub timestamp: i64,
    #[serde(rename = "trim_badging")]
    pub trim_badging: String,
    #[serde(rename = "use_range_badging")]
    pub use_range_badging: bool,
    #[serde(rename = "utc_offset")]
    pub utc_offset: i64,
    #[serde(rename = "webcam_selfie_supported")]
    pub webcam_selfie_supported: bool,
    #[serde(rename = "webcam_supported")]
    pub webcam_supported: bool,
    #[serde(rename = "wheel_type")]
    pub wheel_type: String,
}

#[allow(missing_docs)]
#[allow(clippy::struct_excessive_bools)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VehicleState {
    #[serde(rename = "api_version")]
    pub api_version: i64,
    #[serde(rename = "autopark_state_v3")]
    pub autopark_state_v3: String,
    #[serde(rename = "autopark_style")]
    pub autopark_style: String,
    #[serde(rename = "calendar_supported")]
    pub calendar_supported: bool,
    #[serde(rename = "car_version")]
    pub car_version: String,
    #[serde(rename = "center_display_state")]
    pub center_display_state: i64,
    #[serde(rename = "dashcam_clip_save_available")]
    pub dashcam_clip_save_available: bool,
    #[serde(rename = "dashcam_state")]
    pub dashcam_state: String,
    pub df: i64,
    pub dr: i64,
    #[serde(rename = "fd_window")]
    pub fd_window: i64,
    #[serde(rename = "feature_bitmask")]
    pub feature_bitmask: String,
    #[serde(rename = "fp_window")]
    pub fp_window: i64,
    pub ft: i64,
    #[serde(rename = "homelink_device_count")]
    pub homelink_device_count: i64,
    #[serde(rename = "homelink_nearby")]
    pub homelink_nearby: bool,
    #[serde(rename = "is_user_present")]
    pub is_user_present: bool,
    #[serde(rename = "last_autopark_error")]
    pub last_autopark_error: String,
    pub locked: bool,
    #[serde(rename = "media_info")]
    pub media_info: MediaInfo,
    #[serde(rename = "media_state")]
    pub media_state: MediaState,
    #[serde(rename = "notifications_supported")]
    pub notifications_supported: bool,
    pub odometer: f64,
    #[serde(rename = "parsed_calendar_supported")]
    pub parsed_calendar_supported: bool,
    pub pf: i64,
    pub pr: i64,
    #[serde(rename = "rd_window")]
    pub rd_window: i64,
    #[serde(rename = "remote_start")]
    pub remote_start: bool,
    #[serde(rename = "remote_start_enabled")]
    pub remote_start_enabled: bool,
    #[serde(rename = "remote_start_supported")]
    pub remote_start_supported: bool,
    #[serde(rename = "rp_window")]
    pub rp_window: i64,
    pub rt: i64,
    #[serde(rename = "santa_mode")]
    pub santa_mode: i64,
    #[serde(rename = "sentry_mode")]
    pub sentry_mode: bool,
    #[serde(rename = "sentry_mode_available")]
    pub sentry_mode_available: bool,
    #[serde(rename = "service_mode")]
    pub service_mode: bool,
    #[serde(rename = "service_mode_plus")]
    pub service_mode_plus: bool,
    #[serde(rename = "smart_summon_available")]
    pub smart_summon_available: bool,
    #[serde(rename = "software_update")]
    pub software_update: SoftwareUpdate,
    #[serde(rename = "speed_limit_mode")]
    pub speed_limit_mode: SpeedLimitMode,
    #[serde(rename = "summon_standby_mode_enabled")]
    pub summon_standby_mode_enabled: bool,
    pub timestamp: i64,
    #[serde(rename = "tpms_hard_warning_fl")]
    pub tpms_hard_warning_fl: bool,
    #[serde(rename = "tpms_hard_warning_fr")]
    pub tpms_hard_warning_fr: bool,
    #[serde(rename = "tpms_hard_warning_rl")]
    pub tpms_hard_warning_rl: bool,
    #[serde(rename = "tpms_hard_warning_rr")]
    pub tpms_hard_warning_rr: bool,
    #[serde(rename = "tpms_last_seen_pressure_time_fl")]
    pub tpms_last_seen_pressure_time_fl: i64,
    #[serde(rename = "tpms_last_seen_pressure_time_fr")]
    pub tpms_last_seen_pressure_time_fr: i64,
    #[serde(rename = "tpms_last_seen_pressure_time_rl")]
    pub tpms_last_seen_pressure_time_rl: i64,
    #[serde(rename = "tpms_last_seen_pressure_time_rr")]
    pub tpms_last_seen_pressure_time_rr: i64,
    #[serde(rename = "tpms_pressure_fl")]
    pub tpms_pressure_fl: f64,
    #[serde(rename = "tpms_pressure_fr")]
    pub tpms_pressure_fr: f64,
    #[serde(rename = "tpms_pressure_rl")]
    pub tpms_pressure_rl: f64,
    #[serde(rename = "tpms_pressure_rr")]
    pub tpms_pressure_rr: i64,
    #[serde(rename = "tpms_rcp_front_value")]
    pub tpms_rcp_front_value: f64,
    #[serde(rename = "tpms_rcp_rear_value")]
    pub tpms_rcp_rear_value: f64,
    #[serde(rename = "tpms_soft_warning_fl")]
    pub tpms_soft_warning_fl: bool,
    #[serde(rename = "tpms_soft_warning_fr")]
    pub tpms_soft_warning_fr: bool,
    #[serde(rename = "tpms_soft_warning_rl")]
    pub tpms_soft_warning_rl: bool,
    #[serde(rename = "tpms_soft_warning_rr")]
    pub tpms_soft_warning_rr: bool,
    #[serde(rename = "valet_mode")]
    pub valet_mode: bool,
    #[serde(rename = "valet_pin_needed")]
    pub valet_pin_needed: bool,
    #[serde(rename = "vehicle_name")]
    pub vehicle_name: String,
    #[serde(rename = "vehicle_self_test_progress")]
    pub vehicle_self_test_progress: i64,
    #[serde(rename = "vehicle_self_test_requested")]
    pub vehicle_self_test_requested: bool,
    #[serde(rename = "webcam_available")]
    pub webcam_available: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, PartialEq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaInfo {
    #[serde(rename = "a2dp_source_name")]
    pub a2dp_source_name: String,
    #[serde(rename = "audio_volume")]
    pub audio_volume: f64,
    #[serde(rename = "audio_volume_increment")]
    pub audio_volume_increment: f64,
    #[serde(rename = "audio_volume_max")]
    pub audio_volume_max: f64,
    #[serde(rename = "media_playback_status")]
    pub media_playback_status: String,
    #[serde(rename = "now_playing_album")]
    pub now_playing_album: String,
    #[serde(rename = "now_playing_artist")]
    pub now_playing_artist: String,
    #[serde(rename = "now_playing_duration")]
    pub now_playing_duration: i64,
    #[serde(rename = "now_playing_elapsed")]
    pub now_playing_elapsed: i64,
    #[serde(rename = "now_playing_source")]
    pub now_playing_source: String,
    #[serde(rename = "now_playing_station")]
    pub now_playing_station: String,
    #[serde(rename = "now_playing_title")]
    pub now_playing_title: String,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MediaState {
    #[serde(rename = "remote_control_enabled")]
    pub remote_control_enabled: bool,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SoftwareUpdate {
    #[serde(rename = "download_perc")]
    pub download_perc: i64,
    #[serde(rename = "expected_duration_sec")]
    pub expected_duration_sec: i64,
    #[serde(rename = "install_perc")]
    pub install_perc: i64,
    pub status: String,
    pub version: String,
}

#[allow(missing_docs)]
#[derive(Default, Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SpeedLimitMode {
    pub active: bool,
    #[serde(rename = "current_limit_mph")]
    pub current_limit_mph: i64,
    #[serde(rename = "max_limit_mph")]
    pub max_limit_mph: i64,
    #[serde(rename = "min_limit_mph")]
    pub min_limit_mph: i64,
    #[serde(rename = "pin_code_set")]
    pub pin_code_set: bool,
}
