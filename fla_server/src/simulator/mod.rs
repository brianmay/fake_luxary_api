//! Simulate a car
pub mod data;

use std::sync::Arc;

use fla_common::streaming::{DataError, StreamingData};
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::{errors, types::VehicleDataState};
pub mod server;

type WakeUpResponse = Result<(), errors::ResponseError>;
type VehicleDataResponse = Result<VehicleDataState, errors::ResponseError>;
type SubscribeResponse = Result<broadcast::Receiver<Arc<StreamingData>>, DataError>;

enum Command {
    WakeUp(oneshot::Sender<WakeUpResponse>),
    GetVehicleData(oneshot::Sender<VehicleDataResponse>),
    Subscribe(oneshot::Sender<SubscribeResponse>),
}

/// A handle to the simulator
pub struct CommandSender(mpsc::Sender<Command>);

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
            .map_err(|_| DataError::disconnected())?;

        tokio::time::timeout(TIMEOUT, rx)
            .await
            .map_err(|_| DataError::disconnected())?
            .map_err(|_| DataError::disconnected())?
    }
}
