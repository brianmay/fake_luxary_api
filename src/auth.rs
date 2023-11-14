//! Authentication middleware

use std::sync::Arc;

use crate::tokens;
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use axum_auth::AuthBearer;
use tracing::error;

use crate::errors::ResponseError;

#[derive(Clone)]
struct CurrentUser {/* ... */}

/// Extract the current access token claims from the request
///
/// # Errors
///
/// Returns `ResponseError::TokenExpired` if the token is invalid or expired
pub async fn access_token<B: Send>(
    State(config): State<Arc<tokens::Config>>,
    AuthBearer(token): AuthBearer,
    mut req: Request<B>,
    next: Next<B>,
) -> Result<Response, ResponseError> {
    match tokens::validate_access_token(&token, &config) {
        Ok(claims) => {
            req.extensions_mut().insert(Arc::new(claims));
            Ok(next.run(req).await)
        }
        Err(err) => {
            error!("Invalid token: {}", err);
            Err(ResponseError::TokenExpired)
        }
    }
}
