use std::{fmt::Formatter, sync::Arc};

use crate::simulator;
use fla_common::types::{VehicleDefinition, VehicleGuid, VehicleId};
use tokio::sync::RwLock;
use tracing::log::debug;

/// A vehicle
pub struct Vehicle {
    /// The vehicle ID
    pub id: VehicleId,

    /// The vehicle GUID
    pub vehicle_id: VehicleGuid,

    /// The vehicle data
    pub data: Arc<RwLock<VehicleDefinition>>,

    /// The command sender
    pub command: simulator::CommandSender,
}

impl Vehicle {
    /// Create a new vehicle
    #[must_use]
    pub fn new(data: VehicleDefinition) -> Vehicle {
        let id = data.id;
        let vehicle_id = data.vehicle_id;
        let command = simulator::server::start(data.clone());
        let data = Arc::new(RwLock::new(data));

        let d = data.clone();
        let c = command.clone();

        let vehicle = Self {
            id,
            vehicle_id,
            data,
            command,
        };

        tokio::spawn(async move {
            let mut stream = c.watch_state().await.unwrap_or_else(|_| {
                panic!("Failed to get state stream for vehicle {}", id.to_string());
            });
            // We must drop this so we don't force the vehicle to stay alive.
            drop(c);

            while let Ok(data) = stream.recv().await {
                let online_state = data.into();
                let current = d.read().await.state.clone();
                if current != online_state {
                    debug!(
                        "Vehicle {:?} was {:?} is now {:?}",
                        id, current, online_state
                    );
                    let mut data = d.write().await;
                    data.state = online_state;
                }
            }
        });

        vehicle
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
