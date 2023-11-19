//! The main HTTP server

use axum::Router;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

use fla_server::Config;
use fla_server::{
    api::{auth, owner, streaming},
    data, tokens,
};

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let config = Config {
        token: Arc::new(tokens::Config {
            secret: "mom-said-yes".to_string(),
        }),
        vehicles: Arc::new(data::get_vehicles()),
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
