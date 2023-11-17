//! Simulate a car

use std::sync::Arc;

use tokio::sync::{broadcast, mpsc, oneshot};

use crate::{errors, types};
pub mod server;

type WakeUpResponse = Result<(), errors::ResponseError>;

enum Command {
    WakeUp(oneshot::Sender<WakeUpResponse>),
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
}

/// A handle to the simulator streaming data
pub struct StreamReceiver(broadcast::Sender<Arc<types::StreamingData>>);

impl StreamReceiver {
    /// Subscribe to streaming data
    #[must_use]
    pub fn subscribe(&self) -> broadcast::Receiver<Arc<types::StreamingData>> {
        self.0.subscribe()
    }
}
