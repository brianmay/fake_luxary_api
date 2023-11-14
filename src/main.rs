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
use fake_luxury_api::tokens;
use fake_luxury_api::TeslaResponse;
use std::{collections::HashSet, sync::Arc};
use tracing::error;

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

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct AuthorizationCodeRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    scope: String,
    audience: String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct RefreshTokenRequest {
    refresh_token: String,
    client_id: String,
    scope: String,
}

#[allow(dead_code)]
#[derive(serde::Deserialize)]
struct ClientCredentialsRequest {
    client_id: String,
    client_secret: String,
    scope: String,
    audience: String,
}

// #[allow(dead_code)]
#[derive(serde::Deserialize)]
#[serde(tag = "grant_type")]
enum TokenRequest {
    #[serde(rename = "authorization_code")]
    AuthorizationCode(AuthorizationCodeRequest),
    #[serde(rename = "refresh_token")]
    RefreshToken(RefreshTokenRequest),
    #[serde(rename = "client_credentials")]
    ClientCredentials(ClientCredentialsRequest),
}

/// Raw Tesla token from API
#[derive(serde::Serialize)]
pub struct TokenResult {
    access_token: String,
    refresh_token: String,
    id_token: String,
    token_type: String,
    expires_in: u64,
}

fn renew_token(
    request: &RefreshTokenRequest,
    config: &tokens::Config,
) -> Result<TokenResult, errors::ResponseError> {
    let claims = match tokens::validate_refresh_token(&request.refresh_token, config) {
        Ok(claims) => claims,
        Err(err) => {
            error!("Invalid token: {}", err);
            return Err(errors::ResponseError::TokenExpired);
        }
    };

    let scopes: HashSet<String> = request
        .scope
        .split(' ')
        .map(std::string::ToString::to_string)
        .collect::<HashSet<_>>()
        .intersection(&claims.scopes)
        .cloned()
        .collect();

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

    let token = tokens::Token::new(config, &scopes).map_err(|err| {
        errors::ResponseError::internal_error(format!("Could not create token: {err:?}"))
    })?;

    let expires_in = (token.expires_at - Utc::now()).num_seconds();
    let expires_in = u64::try_from(expires_in).map_err(|err| {
        errors::ResponseError::internal_error(format!("Could not convert timestamp: {err:?}"))
    })?;

    let response = TokenResult {
        access_token: token.access_token,
        refresh_token: token.refresh_token,
        id_token: "zzzz".into(),
        token_type: "xxxx".into(),
        expires_in,
    };

    Ok(response)
}

async fn token_handler(
    State(config): State<Arc<tokens::Config>>,
    Json(body): Json<TokenRequest>,
) -> Result<Json<TokenResult>, errors::ResponseError> {
    match body {
        TokenRequest::RefreshToken(request) => Ok(Json(renew_token(&request, &config)?)),
        TokenRequest::ClientCredentials(_) | TokenRequest::AuthorizationCode(_) => {
            Err(errors::ResponseError::not_implemented(
                "We only support refresh_token grant type for now.".to_string(),
            ))
        }
    }
}

async fn dummy_handler(
    Extension(_): Extension<Arc<tokens::AccessClaims>>,
    State(_config): State<Arc<tokens::Config>>,
    Json(_body): Json<()>,
) -> Result<Json<TeslaResponse<()>>, errors::ResponseError> {
    Ok(Json(TeslaResponse::success(())))
}
