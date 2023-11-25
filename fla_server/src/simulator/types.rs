use fla_common::{
    simulator::SimulationStateEnum,
    streaming::StreamingData,
    types::{
        ChargeState, ClimateState, DriveState, GranularAccess, GuiSettings, Timestamp,
        VehicleConfig, VehicleData, VehicleGuid, VehicleId, VehicleState, VehicleStateEnum,
    },
};
use tokio::time::Instant;

#[derive(Debug, Clone)]
pub struct SimulationDriveState {
    pub time: Instant,
    pub latitude: f64,
    pub longitude: f64,
    pub heading: u16,
    pub speed: f32,
}

impl From<&VehicleDataState> for SimulationDriveState {
    fn from(data: &VehicleDataState) -> Self {
        Self {
            time: Instant::now(),
            latitude: data.drive_state.latitude.unwrap_or(0.0),
            longitude: data.drive_state.longitude.unwrap_or(0.0),
            heading: data.drive_state.heading,
            speed: 60.0,
        }
    }
}

#[derive(Debug, Clone)]
pub struct SimulationChargeState {
    pub time: Instant,
    pub battery_level: u8,
    // pub battery_range: f32,
}

impl From<&VehicleDataState> for SimulationChargeState {
    fn from(data: &VehicleDataState) -> Self {
        Self {
            time: Instant::now(),
            battery_level: data.charge_state.battery_level,
            // battery_range: data.charge_state.battery_range,
        }
    }
}

#[derive(Debug, Clone)]
pub enum SimulationState {
    /// The vehicle is driving
    Driving {
        state: SimulationDriveState,
        update_time: Instant,
    },

    /// The vehicle is charging
    Charging {
        state: SimulationChargeState,
        update_time: Instant,
    },

    /// The vehicle is idle
    Idle { sleep_time: Instant },

    /// The vehicle is idle but should not go to sleep
    IdleNoSleep,

    /// The vehicle is sleeping
    Sleeping { wake_up_time: Option<Instant> },
}

impl SimulationState {
    pub fn is_asleep(&self) -> bool {
        matches!(self, Self::Sleeping { .. })
    }

    pub fn is_driving(&self) -> bool {
        matches!(self, Self::Driving { .. })
    }

    pub fn drive(self, data: &VehicleDataState, now: Instant) -> Self {
        let state = if let Self::Driving { state, .. } = self {
            state
        } else {
            SimulationDriveState::from(data)
        };
        Self::Driving {
            state,
            update_time: now + std::time::Duration::from_secs(1),
        }
    }

    pub fn charge(self, data: &VehicleDataState, now: Instant) -> Self {
        let state = if let Self::Charging { state, .. } = self {
            state
        } else {
            SimulationChargeState::from(data)
        };
        Self::Charging {
            state,
            update_time: now + std::time::Duration::from_secs(10),
        }
    }

    pub fn idle(now: Instant) -> Self {
        Self::Idle {
            sleep_time: now + std::time::Duration::from_secs(60),
        }
    }

    pub fn sleeping() -> Self {
        Self::Sleeping { wake_up_time: None }
    }

    pub fn wake_up(self, now: Instant) -> Self {
        if let Self::Sleeping {
            wake_up_time: Some(_),
        } = self
        {
            // Wake up time already set, do nothing
            return self;
        }
        // Schedule wake up time
        Self::Sleeping {
            wake_up_time: Some(now + std::time::Duration::from_secs(60)),
        }
    }

    pub fn update_sleep_time(self, now: Instant) -> Self {
        if let Self::Idle { .. } = self {
            Self::Idle {
                sleep_time: now + std::time::Duration::from_secs(60),
            }
        } else {
            self
        }
    }
}

impl From<&SimulationState> for SimulationStateEnum {
    fn from(state: &SimulationState) -> Self {
        match state {
            SimulationState::Driving { .. } => Self::Driving,
            SimulationState::Charging { .. } => Self::Charging,
            SimulationState::Idle { .. } => Self::Idle,
            SimulationState::IdleNoSleep => Self::IdleNoSleep,
            SimulationState::Sleeping { .. } => Self::Sleeping,
        }
    }
}

/// Current state of all Vehicle Data
#[allow(missing_docs)]
#[derive(Debug)]
pub struct VehicleDataState {
    // These fields are from VehicleData.
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

    // These fields are from VehicleData but not optional.
    pub charge_state: ChargeState,
    pub climate_state: ClimateState,
    pub drive_state: DriveState,
    pub gui_settings: GuiSettings,
    pub vehicle_config: VehicleConfig,
    pub vehicle_state: VehicleState,

    // Extra data not in VehicleData.
    pub elevation: u32,
}

impl From<&VehicleDataState> for StreamingData {
    fn from(data: &VehicleDataState) -> Self {
        Self {
            id: data.vehicle_id,
            time: data.drive_state.timestamp,
            speed: data.drive_state.speed,
            odometer: Some(data.vehicle_state.odometer),
            soc: Some(data.charge_state.battery_level),
            elevation: Some(data.elevation),
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

impl From<&VehicleDataState> for VehicleData {
    fn from(data: &VehicleDataState) -> Self {
        Self {
            id: data.id,
            user_id: data.user_id,
            vehicle_id: data.vehicle_id,
            vin: data.vin.clone(),
            color: data.color.clone(),
            access_type: data.access_type.clone(),
            granular_access: data.granular_access.clone(),
            tokens: data.tokens.clone(),
            state: data.state.clone(),
            in_service: data.in_service,
            id_s: data.id_s.clone(),
            calendar_enabled: data.calendar_enabled,
            api_version: data.api_version,
            backseat_token: data.backseat_token.clone(),
            backseat_token_updated_at: data.backseat_token_updated_at,
            charge_state: Some(data.charge_state.clone()),
            climate_state: Some(data.climate_state.clone()),
            drive_state: Some(data.drive_state.clone()),
            gui_settings: Some(data.gui_settings.clone()),
            vehicle_config: Some(data.vehicle_config.clone()),
            vehicle_state: Some(data.vehicle_state.clone()),
        }
    }
}
