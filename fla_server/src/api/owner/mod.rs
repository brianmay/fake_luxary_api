//! Tesla Owner API

use self::commands::{simulate_handler, wake_up_handler};
use self::vehicles::{vehicle_data_handler, vehicle_handler, vehicles_handler};
use crate::{middleware, Config};
use axum::routing::post;
use axum::{middleware::from_fn_with_state, routing::get, Router};

pub mod commands;
pub mod vehicles;

/// Retrieve router for Tesla Owner API
pub fn router(config: &Config) -> Router {
    Router::new()
        .route("/api/1/vehicles", get(vehicles_handler))
        .route("/api/1/vehicles/:id", get(vehicle_handler))
        .route("/api/1/vehicles/:id/simulate", post(simulate_handler))
        .route(
            "/api/1/vehicles/:id/vehicle_data",
            get(vehicle_data_handler),
        )
        .route("/api/1/vehicles/:id/wake_up", post(wake_up_handler))
        .layer(from_fn_with_state(config.clone(), middleware::access_token))
        .with_state(config.clone())
}
