use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Copy, Clone)]
pub enum SimulationStateEnum {
    /// The vehicle is driving
    Driving,

    /// The vehicle is charging
    Charging,

    /// The vehicle is idle
    Idle,

    /// The vehicle is idle but should not go to sleep
    IdleNoSleep,

    /// The vehicle is sleeping
    Sleeping,
}

impl FromStr for SimulationStateEnum {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "driving" => Ok(Self::Driving),
            "charging" => Ok(Self::Charging),
            "idle" => Ok(Self::Idle),
            "idle_no_sleep" => Ok(Self::IdleNoSleep),
            "sleeping" => Ok(Self::Sleeping),
            _ => Err(format!("Unknown simulation state: {}", s)),
        }
    }
}
