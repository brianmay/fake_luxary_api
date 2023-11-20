//! Simulate a car

use std::sync::Arc;

use fla_common::streaming::StreamingDataOptional;
use tokio::sync::{broadcast, mpsc, oneshot};

use crate::{errors, types::VehicleDataState};
pub mod server;

type WakeUpResponse = Result<(), errors::ResponseError>;
type VehicleDataResponse = Result<VehicleDataState, errors::ResponseError>;

enum Command {
    WakeUp(oneshot::Sender<WakeUpResponse>),
    GetVehicleData(oneshot::Sender<VehicleDataState>),
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
            .map_err(|_| errors::ResponseError::DeviceNotAvailable)
    }
}

/// A handle to the simulator streaming data
pub struct StreamReceiver(broadcast::Sender<Arc<StreamingDataOptional>>);

impl StreamReceiver {
    /// Subscribe to streaming data
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Arc<StreamingDataOptional>> {
        self.0.subscribe()
    }
}
