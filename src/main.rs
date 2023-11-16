//! The main HTTP server
#![warn(missing_docs)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use axum::Router;
use fake_luxury_api::{
    api::{auth, owner, streaming},
    tokens,
};
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use fake_luxury_api::Config;

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config {
        token: Arc::new(tokens::Config {
            secret: "mom-said-yes".to_string(),
        }),
    };

    let app = Router::new()
        .nest("/", owner::router(&config))
        .nest("/", streaming::router(&config))
        .nest("/", auth::router(&config))
        .layer(TraceLayer::new_for_http());

    #[allow(clippy::expect_used)]
    axum::Server::bind(&"[::]:4080".parse().expect("Could not bind to port"))
        .serve(app.into_make_service())
        .await
        .expect("Could not start server");
}
