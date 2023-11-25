use std::fmt::Formatter;

use crate::simulator;
use fla_common::types::VehicleDefinition;

/// A vehicle
pub struct Vehicle {
    /// The vehicle data
    pub data: VehicleDefinition,

    /// The command sender
    pub command: simulator::CommandSender,
}

impl Vehicle {
    /// Create a new vehicle
    #[must_use]
    pub fn new(data: VehicleDefinition) -> Self {
        let command = simulator::server::start(data.clone());
        Self { data, command }
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
