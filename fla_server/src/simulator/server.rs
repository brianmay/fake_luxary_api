//! Simulator server

use std::{cmp::min, sync::Arc};

use chrono::{DateTime, Utc};
use fla_common::{
    streaming::{DataError, StreamingData},
    types::{
        ChargeState, ChargingStateEnum, ClimateState, DriveState, GranularAccess, GuiSettings,
        MediaInfo, MediaState, SoftwareUpdate, SpeedLimitMode, VehicleConfig, VehicleDefinition,
        VehicleState,
    },
};
use flat_projection::FlatProjection;
use tap::Pipe;
use tokio::{
    select,
    sync::{broadcast, mpsc},
    time::{sleep_until, Instant},
};
use tracing::debug;

use crate::errors::ResponseError;

use super::{
    types::{SimulationChargeState, SimulationDriveState, SimulationState, VehicleDataState},
    Command, CommandSender,
};

#[allow(clippy::too_many_lines)]
fn get_vehicle_data(vehicle: &VehicleDefinition, now: DateTime<Utc>) -> VehicleDataState {
    let timestamp = now.timestamp();

    VehicleDataState {
        id: vehicle.id,
        user_id: 800_001,
        vehicle_id: vehicle.vehicle_id,
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
        id_s: vehicle.id_s.clone(),
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
            charge_limit_soc: 0,
            charge_limit_soc_max: 100,
            charge_limit_soc_min: 50,
            charge_limit_soc_std: 90,
            charge_miles_added_ideal: 202.0,
            charge_miles_added_rated: 202.0,
            charge_port_cold_weather_mode: Some(false),
            charge_port_color: "<invalid>".to_string(),
            charge_port_door_open: false,
            charge_port_latch: "Engaged".to_string(),
            charge_rate: None,
            charger_actual_current: 0,
            charger_phases: None,
            charger_pilot_current: 48,
            charger_power: 0,
            charger_voltage: 2,
            charging_state: ChargingStateEnum::Disconnected,
            conn_charge_cable: "<invalid>".to_string(),
            est_battery_range: 143.88,
            fast_charger_brand: "<invalid>".to_string(),
            fast_charger_present: false,
            fast_charger_type: "<invalid>".to_string(),
            ideal_battery_range: 133.99,
            managed_charging_active: Some(false),
            managed_charging_start_time: None,
            managed_charging_user_canceled: Some(false),
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
            auto_seat_climate_left: Some(false),
            auto_seat_climate_right: Some(false),
            auto_steering_wheel_heat: Some(false),
            battery_heater: false,
            battery_heater_no_power: None,
            bioweapon_mode: false,
            cabin_overheat_protection: "On".to_string(),
            cabin_overheat_protection_actively_cooling: Some(true),
            climate_keeper_mode: "off".to_string(),
            cop_activation_temperature: "High".to_string(),
            defrost_mode: 0,
            driver_temp_setting: 21.0,
            fan_status: 0,
            hvac_auto_request: "On".to_string(),
            inside_temp: 38.4,
            is_auto_conditioning_on: true,
            is_climate_on: false,
            is_front_defroster_on: false,
            is_preconditioning: false,
            is_rear_defroster_on: false,
            left_temp_direction: -293,
            max_avail_temp: 28.0,
            min_avail_temp: 15.0,
            outside_temp: 36.5,
            passenger_temp_setting: 21.0,
            remote_heater_control_enabled: false,
            right_temp_direction: -276,
            seat_heater_left: 0,
            seat_heater_rear_center: 0,
            seat_heater_rear_left: 0,
            seat_heater_rear_right: 0,
            seat_heater_right: 0,
            side_mirror_heaters: false,
            steering_wheel_heat_level: Some(0),
            steering_wheel_heater: false,
            supports_fan_only_cabin_overheat_protection: true,
            timestamp,
            wiper_blade_heater: false,
        },
        drive_state: DriveState {
            active_route_latitude: 37.776_549_4,
            active_route_longitude: -122.419_541_8,
            active_route_traffic_minutes_delay: 0.0,
            gps_as_of: 1_692_137_422,
            heading: 0,
            latitude: Some(0.0),
            longitude: Some(0.0),
            native_latitude: None,
            native_location_supported: 1,
            native_longitude: None,
            native_type: "wgs".to_string(),
            power: Some(0),
            shift_state: None,
            speed: Some(0),
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
            aux_park_lamps: Some("NaPremium".to_string()),
            badge_version: None,
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
            exterior_trim: Some("Black".to_string()),
            exterior_trim_override: String::new(),
            has_air_suspension: false,
            has_ludicrous_mode: false,
            has_seat_cooling: false,
            headlamp_type: "Premium".to_string(),
            interior_trim_type: "Black2".to_string(),
            key_version: Some(2),
            motorized_charge_port: true,
            paint_color_override: "19,20,22,0.8,0.04".to_string(),
            performance_package: Some("Base".to_string()),
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
            autopark_state_v3: Some("ready".to_string()),
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
            homelink_device_count: Some(3),
            homelink_nearby: Some(false),
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
            odometer: 0.0,
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
            sentry_mode: Some(false),
            sentry_mode_available: Some(true),
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
                current_limit_mph: 85.0,
                max_limit_mph: 120.0,
                min_limit_mph: 50.0,
                pin_code_set: false,
            },
            summon_standby_mode_enabled: false,
            timestamp,
            tpms_hard_warning_fl: false,
            tpms_hard_warning_fr: false,
            tpms_hard_warning_rl: false,
            tpms_hard_warning_rr: false,
            tpms_last_seen_pressure_time_fl: Some(timestamp),
            tpms_last_seen_pressure_time_fr: Some(timestamp),
            tpms_last_seen_pressure_time_rl: Some(timestamp),
            tpms_last_seen_pressure_time_rr: Some(timestamp),
            tpms_pressure_fl: 3.1,
            tpms_pressure_fr: 3.1,
            tpms_pressure_rl: 3.15,
            tpms_pressure_rr: 3.0,
            tpms_rcp_front_value: 2.9,
            tpms_rcp_rear_value: 2.9,
            tpms_soft_warning_fl: false,
            tpms_soft_warning_fr: false,
            tpms_soft_warning_rl: false,
            tpms_soft_warning_rr: false,
            valet_mode: false,
            valet_pin_needed: true,
            vehicle_name: Some("grADOFIN".to_string()),
            vehicle_self_test_progress: Some(0),
            vehicle_self_test_requested: Some(false),
            webcam_available: true,
        },

        elevation: 0,
        ss: SimulationState::idle(Instant::now()),
    }
}

/// Start the simulator
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn start(vehicle: VehicleDefinition) -> CommandSender {
    let (c_tx, mut c_rx) = mpsc::channel(1);
    let mut maybe_s_tx: Option<broadcast::Sender<Arc<StreamingData>>> = None;

    tokio::spawn(async move {
        // Simulated real time values.

        let mut data = get_vehicle_data(&vehicle, Utc::now());

        loop {
            let new_ss = select! {
                Some(state) = maybe_update_drive(&data) => {
                    debug!("Car {:?} is driving", data.id);
                    let (drive_state, elevation, ss) = get_updated_drive_state(&data, state);
                    data.drive_state = drive_state;
                    data.elevation = elevation;
                    let streaming_data: StreamingData = (&data).into();

                    if let Some(s_tx) = &maybe_s_tx {
                        // It is not an error if we are sending and nobody is listening.
                        _ = s_tx.send(Arc::new(streaming_data.clone()));
                    }

                    // If the car is stopped, stop sending data.
                    if data.drive_state.speed.unwrap_or(0) == 0 {
                        maybe_s_tx = None;
                    }

                    ss
                }
                Some(state) = maybe_update_charge(&data) => {
                    debug!("Car {:?} is charging", data.id);
                    let (charge_state, ss) = get_updated_charge_state(&data, state);
                    data.charge_state = charge_state;
                    ss
                }
                Some(()) = maybe_sleep(&data) => {
                    debug!("Car {:?} is going to sleep", data.id);
                    SimulationState::sleeping()
                }
                Some(()) = maybe_wake_up(&data) => {
                    debug!("Car {:?} is waking up", data.id);
                    SimulationState::idle(Instant::now())
                }
                cmd = c_rx.recv() => {
                    match cmd {
                        Some(Command::WakeUp(tx)) => {
                            if data.ss.is_asleep() {
                                _= Err(ResponseError::DeviceNotAvailable).pipe(|x| tx.send(x));
                                data.ss.wake_up(Instant::now())
                            } else {
                                _ = Ok(()).pipe(|x| tx.send(x));
                                data.ss
                            }
                        }
                        Some(Command::GetVehicleData(tx)) => {
                            if data.ss.is_asleep() {
                                _= Err(ResponseError::DeviceNotAvailable).pipe(|x| tx.send(x));
                                data.ss
                            } else {
                                let response = (&data).into();
                                _ = Ok(response).pipe(|x| tx.send(x));
                                data.ss
                            }
                        }
                        Some(Command::Subscribe(tx)) => {
                            if data.ss.is_asleep() {
                                _ = Err(DataError::disconnected()).pipe(|x| tx.send(x));
                            } else if let Some(s_tx) = &maybe_s_tx {
                                _ = s_tx.subscribe().pipe(Ok).pipe(|x| tx.send(x));
                            } else {
                                let (s_tx, s_rx) = broadcast::channel(1);
                                _ = s_rx.pipe(Ok).pipe(|x| tx.send(x));
                                maybe_s_tx = Some(s_tx);

                            }
                            data.ss
                        }
                        None => {
                            debug!("Command channel closed, exiting simulator");
                            break;
                        }
                    }
                }
            };

            // If the car is asleep, stop streaming
            if new_ss.is_asleep() {
                maybe_s_tx = None;
            }

            data.ss = new_ss;
        }
    });

    CommandSender(c_tx)
}

async fn maybe_update_drive(vehicle: &VehicleDataState) -> Option<&SimulationDriveState> {
    if let SimulationState::Driving { update_time, state } = &vehicle.ss {
        sleep_until(*update_time).await;
        Some(state)
    } else {
        None
    }
}

async fn maybe_update_charge(vehicle: &VehicleDataState) -> Option<&SimulationChargeState> {
    if let SimulationState::Charging { update_time, state } = &vehicle.ss {
        sleep_until(*update_time).await;
        Some(state)
    } else {
        None
    }
}

async fn maybe_sleep(vehicle: &VehicleDataState) -> Option<()> {
    if let SimulationState::Idle { sleep_time } = &vehicle.ss {
        sleep_until(*sleep_time).await;
        Some(())
    } else {
        None
    }
}

async fn maybe_wake_up(vehicle: &VehicleDataState) -> Option<()> {
    if let SimulationState::Sleeping {
        wake_up_time: Some(wake_up_time),
    } = &vehicle.ss
    {
        sleep_until(*wake_up_time).await;
        Some(())
    } else {
        None
    }
}

fn get_updated_drive_state(
    data: &VehicleDataState,
    state: &SimulationDriveState,
) -> (DriveState, u32, SimulationState) {
    let now = Utc::now();
    let duration = state.time.duration_since(Instant::now());

    let proj = FlatProjection::new(state.longitude, state.latitude);
    let mut point = proj.project(state.longitude, state.latitude);
    point.x += duration.as_secs_f64() * 10.0;
    point.y += duration.as_secs_f64() * 10.0;
    let (latitude, longitude) = proj.unproject(&point);

    let drive_state = DriveState {
        active_route_latitude: latitude,
        active_route_longitude: longitude,
        active_route_traffic_minutes_delay: 0.0,
        gps_as_of: now.timestamp(),
        heading: 0,
        latitude: Some(latitude),
        longitude: Some(longitude),
        native_latitude: None,
        native_location_supported: 1,
        native_longitude: None,
        native_type: "wgs".to_string(),
        power: Some(0),
        shift_state: None,
        speed: Some(0),
        timestamp: now.timestamp(),
    };

    let elevation = 0;

    (
        drive_state,
        elevation,
        data.ss.clone().drive(data, Instant::now()),
    )
}

fn get_updated_charge_state(
    data: &VehicleDataState,
    state: &SimulationChargeState,
) -> (ChargeState, SimulationState) {
    let now = Utc::now();
    let duration = state.time.duration_since(Instant::now());

    let battery_level_u64 = u64::from(state.battery_level) + duration.as_secs() / 60 * 10;
    let battery_level = u8::try_from(battery_level_u64).unwrap_or(255);

    let finished_charging = battery_level >= 100;
    let battery_level = min(battery_level, 100);

    let charge_state = ChargeState {
        battery_heater_on: false,
        battery_level,
        battery_range: 133.99,
        charge_amps: 48,
        charge_current_request: 48,
        charge_current_request_max: 48,
        charge_enable_request: true,
        charge_energy_added: 48.45,
        charge_limit_soc: 0,
        charge_limit_soc_max: 100,
        charge_limit_soc_min: 50,
        charge_limit_soc_std: 90,
        charge_miles_added_ideal: 202.0,
        charge_miles_added_rated: 202.0,
        charge_port_cold_weather_mode: Some(false),
        charge_port_color: "<invalid>".to_string(),
        charge_port_door_open: false,
        charge_port_latch: "Engaged".to_string(),
        charge_rate: None,
        charger_actual_current: 0,
        charger_phases: None,
        charger_pilot_current: 48,
        charger_power: 0,
        charger_voltage: 2,
        charging_state: ChargingStateEnum::Charging,
        conn_charge_cable: "<invalid>".to_string(),
        est_battery_range: 143.88,
        fast_charger_brand: "<invalid>".to_string(),
        fast_charger_present: false,
        fast_charger_type: "<invalid>".to_string(),
        ideal_battery_range: 133.99,
        managed_charging_active: Some(false),
        managed_charging_start_time: None,
        managed_charging_user_canceled: Some(false),
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
        timestamp: now.timestamp(),
        trip_charging: false,
        usable_battery_level: 42,
        user_charge_enable_request: None,
    };

    if finished_charging {
        (charge_state, SimulationState::idle(Instant::now()))
    } else {
        (charge_state, data.ss.clone().charge(data, Instant::now()))
    }
}
