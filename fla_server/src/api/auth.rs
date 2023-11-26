//! Auth API Handlers

use std::collections::HashSet;
use std::str::FromStr;
use std::sync::Arc;

use axum::extract::State;
use axum::routing::post;
use axum::Json;
use axum::Router;
use fla_common::auth::RawToken;
use fla_common::auth::RefreshTokenRequest;
use fla_common::auth::TokenRequest;
use tracing::error;

use crate::errors;
use crate::tokens;
use crate::tokens::new_token;
use crate::tokens::ScopeEnum;
use crate::Config;

/// Retrieve router for Tesla auth API
///
pub fn router(config: &Config) -> Router {
    Router::new()
        .route("/oauth2/v3/token", post(token_handler))
        .with_state(config.clone())
}

fn renew_token(
    request: &RefreshTokenRequest,
    config: &tokens::Config,
) -> Result<RawToken, errors::ResponseError> {
    let claims = match tokens::validate_refresh_token(&request.refresh_token, config) {
        Ok(claims) => claims,
        Err(err) => {
            error!("Invalid token: {}", err);
            return Err(errors::ResponseError::TokenExpired);
        }
    };

    let requested_scopes: HashSet<tokens::ScopeEnum> = request
        .scope
        .split(' ')
        .map(std::string::ToString::to_string)
        .map(|s| ScopeEnum::from_str(&s))
        .collect::<Result<HashSet<_>, ()>>()
        .map_err(|()| errors::ResponseError::internal_error("Could not parse scopes".to_string()))?
        .difference(&claims.scopes)
        .copied()
        .collect();

    if !requested_scopes.is_empty() {
        // We already have all the requested scopes.
        return Err(errors::ResponseError::internal_error(format!(
            "Scopes were requested but not available: {:?}",
            requested_scopes
        )));
    }

    if !claims.scopes.contains(&tokens::ScopeEnum::Openid) {
        // We require openid scope for now.
        return Err(errors::ResponseError::not_implemented(
            "We require openid scope for now.".to_string(),
        ));
    }

    if !claims.scopes.contains(&tokens::ScopeEnum::OfflineAccess) {
        // We require offline_access scope for now.
        return Err(errors::ResponseError::not_implemented(
            "We require offline_access scope for now.".to_string(),
        ));
    }

    let token = new_token(config, &claims.scopes).map_err(|err| {
        errors::ResponseError::internal_error(format!("Could not create token: {err:?}"))
    })?;

    Ok(token)
}

/// Handle a token request
///
/// # Errors
///
/// Returns `ResponseError::TokenExpired` if the token is invalid or expired.
/// Returns `ResponseError::NotImplemented` if the grant type is not supported yet.
/// Returns `ResponseError::InternalServerError` if the token could not be generated.
#[allow(clippy::unused_async)]
pub async fn token_handler(
    State(config): State<Arc<tokens::Config>>,
    Json(body): Json<TokenRequest>,
) -> Result<Json<RawToken>, errors::ResponseError> {
    match body {
        TokenRequest::RefreshToken(request) => Ok(Json(renew_token(&request, &config)?)),
        TokenRequest::ClientCredentials(_) | TokenRequest::AuthorizationCode(_) => {
            Err(errors::ResponseError::not_implemented(
                "We only support refresh_token grant type for now.".to_string(),
            ))
        }
    }
}
