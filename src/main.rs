//! The main HTTP server
#![warn(missing_docs)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use axum::routing::get;
use axum::{extract::FromRef, middleware::from_fn_with_state, routing::post, Router};
use fake_luxury_api::handlers::vehicles::{vehicle_handler, vehicles_handler};
use fake_luxury_api::{handlers::tokens::token_handler, tokens};
use std::sync::Arc;

use fake_luxury_api::auth;

#[derive(Clone, FromRef)]
struct Config {
    token: Arc<tokens::Config>,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();

    let config = Config {
        token: Arc::new(tokens::Config {
            secret: "mom-said-yes".to_string(),
        }),
    };

    let app = Router::new()
        .route("/api/1/vehicles", get(vehicles_handler))
        .route("/api/1/vehicles/:id", get(vehicle_handler))
        .layer(from_fn_with_state(config.clone(), auth::access_token))
        .route("/oauth2/v3/token", post(token_handler))
        .with_state(config);

    #[allow(clippy::expect_used)]
    axum::Server::bind(&"[::]:4080".parse().expect("Could not bind to port"))
        .serve(app.into_make_service())
        .await
        .expect("Could not start server");
}
