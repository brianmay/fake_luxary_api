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
use chrono::Utc;
use fake_luxury_api::tokens::{self, RefreshClaims};
use fake_luxury_api::TeslaResponse;
use std::{collections::HashSet, sync::Arc};

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
        .route("/oauth2/v3/token", post(renew_token))
        .layer(from_fn_with_state(config.clone(), auth::refresh_token))
        .with_state(config);

    #[allow(clippy::expect_used)]
    axum::Server::bind(&"[::]:4080".parse().expect("Could not bind to port"))
        .serve(app.into_make_service())
        .await
        .expect("Could not start server");
}

#[derive(serde::Deserialize, Eq, PartialEq)]
enum GrantType {
    #[serde(rename = "authorization_code")]
    AuthorizationCode,
    #[serde(rename = "refresh_token")]
    RefreshToken,
    #[serde(rename = "client_credentials")]
    ClientCredentials,
}
#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct TokenRenewRequest {
    grant_type: GrantType,
    client_id: Option<String>,
    client_secret: Option<String>,
    code: Option<String>,
    redirect_uri: Option<String>,
    scope: Option<String>,
    audience: Option<String>,
}

/// Raw Tesla token from API
#[derive(serde::Serialize)]
pub struct TokenRenewResult {
    access_token: String,
    refresh_token: String,
    id_token: String,
    token_type: String,
    expires_in: u64,
}

async fn renew_token(
    State(config): State<Arc<tokens::Config>>,
    Extension(_claims): Extension<Arc<RefreshClaims>>,
    Json(body): Json<TokenRenewRequest>,
) -> Result<Json<TeslaResponse<TokenRenewResult>>, errors::ResponseError> {
    if body.grant_type != GrantType::RefreshToken {
        return Err(errors::ResponseError::not_implemented(
            "We only support refresh_token grant type for now.".to_string(),
        ));
    }

    let scopes = body.scope.map_or_else(HashSet::new, |scope| {
        scope
            .split(' ')
            .map(std::string::ToString::to_string)
            .collect::<HashSet<_>>()
    });

    if !scopes.contains("openid") {
        // We require openid scope for now.
        return Err(errors::ResponseError::not_implemented(
            "We require openid scope for now.".to_string(),
        ));
    }

    if !scopes.contains("offline_access") {
        // We require offline_access scope for now.
        return Err(errors::ResponseError::not_implemented(
            "We require offline_access scope for now.".to_string(),
        ));
    }

    let token = tokens::Token::new(&config, &scopes, body.audience.as_deref()).map_err(|err| {
        errors::ResponseError::internal_error(format!("Could not generate token: {err:?}"))
    })?;

    let expires_in = (token.expires_at - Utc::now()).num_seconds();
    let expires_in = u64::try_from(expires_in).map_err(|err| {
        errors::ResponseError::internal_error(format!("Could not convert timestamp: {err:?}"))
    })?;

    let response = TokenRenewResult {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        id_token: "zzzz".into(),
        token_type: "xxxx".into(),
        expires_in,
    };

    Ok(Json(TeslaResponse::success(response)))
}
