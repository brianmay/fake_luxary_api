//! Simulator server

use std::sync::Arc;

use chrono::{DateTime, Utc};
use fla_common::types::{
    ChargeState, ClimateState, DriveState, GranularAccess, GuiSettings, MediaInfo, MediaState,
    ShiftState, SoftwareUpdate, SpeedLimitMode, VehicleConfig, VehicleDefinition, VehicleState,
};
use tokio::{
    select,
    sync::{broadcast, mpsc},
    time::sleep_until,
};
use tracing::debug;

use crate::types::{StreamingData, VehicleData};

use super::{Command, CommandSender, StreamReceiver};

#[allow(clippy::too_many_lines)]
fn get_vehicle_data(data: &StreamingData, now: DateTime<Utc>) -> VehicleData {
    let timestamp = now.timestamp();

    VehicleData {
        id: 100_021,
        user_id: 800_001,
        vehicle_id: 99_999,
        vin: "TEST00000000VIN01".to_string(),
        color: None,
        access_type: "OWNER".to_string(),
        granular_access: GranularAccess {
            hide_private: false,
        },
        tokens: vec![
            "4f993c5b9e2b937b".to_string(),
            "7a3153b1bbb48a96".to_string(),
        ],
        state: None,
        in_service: false,
        id_s: "100021".to_string(),
        calendar_enabled: true,
        api_version: 54,
        backseat_token: None,
        backseat_token_updated_at: None,
        charge_state: ChargeState {
            battery_heater_on: false,
            battery_level: 42,
            battery_range: 133.99,
            charge_amps: 48,
            charge_current_request: 48,
            charge_current_request_max: 48,
            charge_enable_request: true,
            charge_energy_added: 48.45,
            charge_limit_soc: data.soc,
            charge_limit_soc_max: 100,
            charge_limit_soc_min: 50,
            charge_limit_soc_std: 90,
            charge_miles_added_ideal: 202,
            charge_miles_added_rated: 202,
            charge_port_cold_weather_mode: false,
            charge_port_color: "<invalid>".to_string(),
            charge_port_door_open: false,
            charge_port_latch: "Engaged".to_string(),
            charge_rate: 0,
            charger_actual_current: 0,
            charger_phases: None,
            charger_pilot_current: 48,
            charger_power: 0,
            charger_voltage: 2,
            charging_state: "Disconnected".to_string(),
            conn_charge_cable: "<invalid>".to_string(),
            est_battery_range: 143.88,
            fast_charger_brand: "<invalid>".to_string(),
            fast_charger_present: false,
            fast_charger_type: "<invalid>".to_string(),
            ideal_battery_range: 133.99,
            managed_charging_active: false,
            managed_charging_start_time: None,
            managed_charging_user_canceled: false,
            max_range_charge_counter: 0,
            minutes_to_full_charge: 0,
            not_enough_power_to_heat: None,
            off_peak_charging_enabled: false,
            off_peak_charging_times: "all_week".to_string(),
            off_peak_hours_end_time: 360,
            preconditioning_enabled: false,
            preconditioning_times: "all_week".to_string(),
            scheduled_charging_mode: "Off".to_string(),
            scheduled_charging_pending: false,
            scheduled_charging_start_time: None,
            scheduled_departure_time: 1_634_914_800,
            scheduled_departure_time_minutes: 480,
            supercharger_session_trip_planner: false,
            time_to_full_charge: None,
            timestamp,
            trip_charging: false,
            usable_battery_level: 42,
            user_charge_enable_request: None,
        },
        climate_state: ClimateState {
            allow_cabin_overheat_protection: true,
            auto_seat_climate_left: false,
            auto_seat_climate_right: false,
            auto_steering_wheel_heat: false,
            battery_heater: false,
            battery_heater_no_power: None,
            bioweapon_mode: false,
            cabin_overheat_protection: "On".to_string(),
            cabin_overheat_protection_actively_cooling: true,
            climate_keeper_mode: "off".to_string(),
            cop_activation_temperature: "High".to_string(),
            defrost_mode: 0,
            driver_temp_setting: 21,
            fan_status: 0,
            hvac_auto_request: "On".to_string(),
            inside_temp: 38.4,
            is_auto_conditioning_on: true,
            is_climate_on: false,
            is_front_defroster_on: false,
            is_preconditioning: false,
            is_rear_defroster_on: false,
            left_temp_direction: -293,
            max_avail_temp: 28,
            min_avail_temp: 15,
            outside_temp: 36.5,
            passenger_temp_setting: 21,
            remote_heater_control_enabled: false,
            right_temp_direction: -276,
            seat_heater_left: 0,
            seat_heater_rear_center: 0,
            seat_heater_rear_left: 0,
            seat_heater_rear_right: 0,
            seat_heater_right: 0,
            side_mirror_heaters: false,
            steering_wheel_heat_level: 0,
            steering_wheel_heater: false,
            supports_fan_only_cabin_overheat_protection: true,
            timestamp,
            wiper_blade_heater: false,
        },
        drive_state: DriveState {
            active_route_latitude: 37.776_549_4,
            active_route_longitude: -122.419_541_8,
            active_route_traffic_minutes_delay: 0,
            gps_as_of: 1_692_137_422,
            heading: Some(data.est_heading),
            latitude: Some(data.est_lat),
            longitude: Some(data.est_lng),
            native_latitude: data.est_lat,
            native_location_supported: 1,
            native_longitude: data.est_lng,
            native_type: "wgs".to_string(),
            power: data.power,
            shift_state: data.shift_state,
            speed: data.speed,
            timestamp,
        },
        gui_settings: GuiSettings {
            gui_24_hour_time: false,
            gui_charge_rate_units: "mi/hr".to_string(),
            gui_distance_units: "mi/hr".to_string(),
            gui_range_display: "Rated".to_string(),
            gui_temperature_units: "F".to_string(),
            gui_tirepressure_units: "Psi".to_string(),
            show_range_units: false,
            timestamp,
        },
        vehicle_config: VehicleConfig {
            aux_park_lamps: "NaPremium".to_string(),
            badge_version: 0,
            can_accept_navigation_requests: true,
            can_actuate_trunks: true,
            car_special_type: "base".to_string(),
            car_type: "modely".to_string(),
            charge_port_type: "US".to_string(),
            cop_user_set_temp_supported: true,
            dashcam_clip_save_supported: true,
            default_charge_to_max: false,
            driver_assist: "TeslaAP3".to_string(),
            ece_restrictions: false,
            efficiency_package: "MY2021".to_string(),
            eu_vehicle: false,
            exterior_color: "MidnightSilver".to_string(),
            exterior_trim: "Black".to_string(),
            exterior_trim_override: String::new(),
            has_air_suspension: false,
            has_ludicrous_mode: false,
            has_seat_cooling: false,
            headlamp_type: "Premium".to_string(),
            interior_trim_type: "Black2".to_string(),
            key_version: 2,
            motorized_charge_port: true,
            paint_color_override: "19,20,22,0.8,0.04".to_string(),
            performance_package: "Base".to_string(),
            plg: true,
            pws: true,
            rear_drive_unit: "PM216MOSFET".to_string(),
            rear_seat_heaters: 1,
            rear_seat_type: 0,
            rhd: false,
            roof_color: "RoofColorGlass".to_string(),
            seat_type: None,
            spoiler_type: "None".to_string(),
            sun_roof_installed: None,
            supports_qr_pairing: false,
            third_row_seats: "None".to_string(),
            timestamp,
            trim_badging: "74d".to_string(),
            use_range_badging: true,
            utc_offset: -25200,
            webcam_selfie_supported: true,
            webcam_supported: true,
            wheel_type: "Apollo19".to_string(),
        },
        vehicle_state: VehicleState {
            api_version: 54,
            autopark_state_v3: "ready".to_string(),
            autopark_style: "dead_man".to_string(),
            calendar_supported: true,
            car_version: "2023.7.20 7910d26d5c64".to_string(),
            center_display_state: 0,
            dashcam_clip_save_available: false,
            dashcam_state: "Unavailable".to_string(),
            df: 0,
            dr: 0,
            fd_window: 0,
            feature_bitmask: "15dffbff,0".to_string(),
            fp_window: 0,
            ft: 0,
            homelink_device_count: 3,
            homelink_nearby: false,
            is_user_present: false,
            last_autopark_error: "no_error".to_string(),
            locked: true,
            media_info: MediaInfo {
                a2dp_source_name: "Pixel 6".to_string(),
                audio_volume: 2.6667,
                audio_volume_increment: 0.333_333,
                audio_volume_max: 10.333_333,
                media_playback_status: "Playing".to_string(),
                now_playing_album: "KQED".to_string(),
                now_playing_artist: "PBS Newshour on KQED FM".to_string(),
                now_playing_duration: 0,
                now_playing_elapsed: 0,
                now_playing_source: "13".to_string(),
                now_playing_station: "88.5 FM KQED".to_string(),
                now_playing_title: "PBS Newshour".to_string(),
            },
            media_state: MediaState {
                remote_control_enabled: true,
            },
            notifications_supported: true,
            odometer: data.odometer,
            parsed_calendar_supported: true,
            pf: 0,
            pr: 0,
            rd_window: 0,
            remote_start: false,
            remote_start_enabled: true,
            remote_start_supported: true,
            rp_window: 0,
            rt: 0,
            santa_mode: 0,
            sentry_mode: false,
            sentry_mode_available: true,
            service_mode: false,
            service_mode_plus: false,
            smart_summon_available: true,
            software_update: SoftwareUpdate {
                download_perc: 0,
                expected_duration_sec: 2700,
                install_perc: 1,
                status: String::new(),
                version: " ".to_string(),
            },
            speed_limit_mode: SpeedLimitMode {
                active: false,
                current_limit_mph: 85,
                max_limit_mph: 120,
                min_limit_mph: 50,
                pin_code_set: false,
            },
            summon_standby_mode_enabled: false,
            timestamp,
            tpms_hard_warning_fl: false,
            tpms_hard_warning_fr: false,
            tpms_hard_warning_rl: false,
            tpms_hard_warning_rr: false,
            tpms_last_seen_pressure_time_fl: 1_692_136_878,
            tpms_last_seen_pressure_time_fr: 1_692_136_878,
            tpms_last_seen_pressure_time_rl: 1_692_136_878,
            tpms_last_seen_pressure_time_rr: 1_692_136_878,
            tpms_pressure_fl: 3.1,
            tpms_pressure_fr: 3.1,
            tpms_pressure_rl: 3.15,
            tpms_pressure_rr: 3,
            tpms_rcp_front_value: 2.9,
            tpms_rcp_rear_value: 2.9,
            tpms_soft_warning_fl: false,
            tpms_soft_warning_fr: false,
            tpms_soft_warning_rl: false,
            tpms_soft_warning_rr: false,
            valet_mode: false,
            valet_pin_needed: true,
            vehicle_name: "grADOFIN".to_string(),
            vehicle_self_test_progress: 0,
            vehicle_self_test_requested: false,
            webcam_available: true,
        },
    }
}

/// Start the simulator
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn start(_vehicle: VehicleDefinition) -> (CommandSender, StreamReceiver) {
    let (c_tx, mut c_rx) = mpsc::channel(1);
    let (s_tx, _s_rx) = broadcast::channel(1);

    let s_tx_clone = s_tx.clone();
    tokio::spawn(async move {
        // Simulated real time values.
        let data = StreamingData {
            time: 0,
            speed: None,
            odometer: 0.0,
            soc: 0,
            elevation: 0,
            est_heading: 0,
            est_lat: 0.0,
            est_lng: 0.0,
            power: Some(100),
            shift_state: Some(ShiftState::Tesla),
            range: 0,
            est_range: 0,
            heading: 0,
        };

        let mut next_instant = tokio::time::Instant::now() + std::time::Duration::from_secs(1);
        loop {
            select! {
                () = sleep_until(next_instant) => {
                    // It is not an error if we are sending and nobody is listening.
                    _ = s_tx_clone.send(Arc::new(data.clone()));
                    next_instant = tokio::time::Instant::now() + std::time::Duration::from_secs(1);
                }
                cmd = c_rx.recv() => {
                    let now = Utc::now();
                    match cmd {
                        Some(Command::WakeUp(tx)) => {
                            let rc = Ok(());
                            let _ = tx.send(rc);
                        }
                        Some(Command::GetVehicleData(tx)) => {
                            let data = get_vehicle_data(&data, now);
                            let _ = tx.send(data);
                        }
                        None => {
                            debug!("Command channel closed, exiting simulator");
                            break;
                        }
                    }
                }
            }
        }
    });

    (CommandSender(c_tx), StreamReceiver(s_tx))
}
