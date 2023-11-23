//! Fake Luxury API library

use std::sync::Arc;

use axum::extract::FromRef;

pub mod api;
pub mod errors;
pub mod middleware;
pub mod simulator;
pub mod tokens;
pub mod types;

/// The server configuration
#[derive(Clone, FromRef)]
pub struct Config {
    /// The token configuration
    pub token: Arc<tokens::Config>,

    /// The dummy test vehicles
    pub vehicles: Arc<Vec<types::Vehicle>>,
}
