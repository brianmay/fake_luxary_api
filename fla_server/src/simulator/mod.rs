//! Simulate a car
pub mod data;
pub mod server;
mod types;

use std::sync::Arc;

use fla_common::{
    simulator::SimulationStateEnum,
    streaming::{DataError, StreamingData},
    types::{VehicleData, VehicleGuid},
};
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::errors;

type WakeUpResponse = Result<(), errors::ResponseError>;
type VehicleDataResponse = Result<VehicleData, errors::ResponseError>;
type SimulateResponse = Result<(), errors::ResponseError>;
type SubscribeResponse = Result<broadcast::Receiver<Arc<StreamingData>>, DataError>;

enum Command {
    WakeUp(oneshot::Sender<WakeUpResponse>),
    GetVehicleData(oneshot::Sender<VehicleDataResponse>),
    Subscribe(oneshot::Sender<SubscribeResponse>),
    Simulate(SimulationStateEnum, oneshot::Sender<SimulateResponse>),
    WatchState(oneshot::Sender<broadcast::Receiver<SimulationStateEnum>>),
}

/// A handle to the simulator
#[derive(Clone)]
pub struct CommandSender(mpsc::Sender<Command>, VehicleGuid);

const TIMEOUT: std::time::Duration = std::time::Duration::from_secs(10);

impl CommandSender {
    /// Wake up the car
    ///
    /// # Errors
    ///
    /// If the simulator is dead, an error will be returned.
    /// If the request times out, an error will be returned.
    pub async fn wake_up(&self) -> WakeUpResponse {
        let (tx, rx) = oneshot::channel();
        self.0
            .send(Command::WakeUp(tx))
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?;

        tokio::time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
    }

    /// Get the vehicle data
    ///
    /// # Errors
    ///
    /// If the simulator is dead, an error will be returned.
    /// If the request times out, an error will be returned.
    pub async fn get_vehicle_data(&self) -> VehicleDataResponse {
        let (tx, rx) = oneshot::channel();
        self.0
            .send(Command::GetVehicleData(tx))
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?;

        tokio::time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
    }

    /// Subscribe to vehicle data
    ///
    /// # Errors
    ///
    /// If the simulator is dead, an error will be returned.
    /// If the request times out, an error will be returned.
    pub async fn subscribe(&self) -> SubscribeResponse {
        let (tx, rx) = oneshot::channel();
        self.0
            .send(Command::Subscribe(tx))
            .await
            .map_err(|_| DataError::disconnected(self.1))?;

        tokio::time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| DataError::disconnected(self.1))?
            .map_err(|_| DataError::disconnected(self.1))?
    }

    /// Simulate a state
    ///
    /// # Errors
    ///
    /// If the simulator is dead, an error will be returned.
    /// If the request times out, an error will be returned.
    pub async fn simulate(&self, state: SimulationStateEnum) -> SimulateResponse {
        let (tx, rx) = oneshot::channel();
        self.0
            .send(Command::Simulate(state, tx))
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?;

        tokio::time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
    }

    /// Watch the state of the vehicle
    ///
    /// Intended for internal use only.
    ///
    /// # Errors
    ///
    /// If the simulator is dead, an error will be returned.
    /// If the request times out, an error will be returned.
    pub async fn watch_state(
        &self,
    ) -> Result<broadcast::Receiver<SimulationStateEnum>, errors::ResponseError> {
        let (tx, rx) = oneshot::channel();
        self.0
            .send(Command::WatchState(tx))
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?;

        tokio::time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)?
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)
    }
}
