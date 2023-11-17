//! Simulator server

use std::sync::Arc;

use tokio::{
    select,
    sync::{broadcast, mpsc},
};
use tracing::debug;

use crate::types::{StreamingData, VehicleData};

use super::{Command, CommandSender, StreamReceiver};

/// Start the simulator
#[must_use]
#[allow(clippy::needless_pass_by_value)]
pub fn start(_vehicle: VehicleData) -> (CommandSender, StreamReceiver) {
    let (c_tx, mut c_rx) = mpsc::channel(1);
    let (s_tx, _s_rx) = broadcast::channel(1);

    let s_tx_clone = s_tx.clone();
    tokio::spawn(async move {
        loop {
            select! {
                () = tokio::time::sleep(std::time::Duration::from_secs(1)) => {
                    let data = StreamingData {
                        time: 0,
                        speed: 0,
                        odometer: 0,
                        soc: 0,
                        elevation: 0,
                        est_heading: 0,
                        est_lat: 0.0,
                        est_lng: 0.0,
                        power: "penguin".to_string(),
                        shift_state: "P".to_string(),
                        range: 0,
                        est_range: 0,
                        heading: 0,
                    };

                    // It is not an error if we are sending and nobody is listening.
                    _ = s_tx_clone.send(Arc::new(data));
                }
                cmd = c_rx.recv() => {
                    #[allow(clippy::single_match_else)]
                    match cmd {
                        Some(Command::WakeUp(tx)) => {
                            let rc = Ok(());
                            let _ = tx.send(rc);
                        }
                        None => {
                            debug!("Command channel closed, exiting simulator");
                            break;
                        }
                    }
                }
            }
        }
        while let Some(command) = c_rx.recv().await {
            match command {
                Command::WakeUp(tx) => {
                    let rc = Ok(());
                    let _ = tx.send(rc);
                }
            }
        }
    });

    (CommandSender(c_tx), StreamReceiver(s_tx))
}
