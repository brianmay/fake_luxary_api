//! The main HTTP server
#![warn(missing_docs)]
#![deny(clippy::pedantic)]
#![deny(clippy::nursery)]
#![deny(clippy::unwrap_used)]
#![deny(clippy::expect_used)]

use axum::{
    extract::{FromRef, State},
    middleware::from_fn_with_state,
    routing::post,
    Extension, Json, Router,
};
use fake_luxury_api::TeslaResponse;
use fake_luxury_api::{handlers::tokens::token_handler, tokens};
use std::sync::Arc;

use fake_luxury_api::{auth, errors};

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
        .route("/dummy", post(dummy_handler))
        .layer(from_fn_with_state(config.clone(), auth::access_token))
        .route("/oauth2/v3/token", post(token_handler))
        .with_state(config);

    #[allow(clippy::expect_used)]
    axum::Server::bind(&"[::]:4080".parse().expect("Could not bind to port"))
        .serve(app.into_make_service())
        .await
        .expect("Could not start server");
}

async fn dummy_handler(
    Extension(_): Extension<Arc<tokens::AccessClaims>>,
    State(_config): State<Arc<tokens::Config>>,
    Json(_body): Json<()>,
) -> Result<Json<TeslaResponse<()>>, errors::ResponseError> {
    Ok(Json(TeslaResponse::success(())))
}
