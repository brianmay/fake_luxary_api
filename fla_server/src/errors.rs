//! Error handling
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use http::StatusCode;
use tracing::error;

use fla_common::responses::error;

/// An error response
pub enum ResponseError {
    /// If the date_request or command is unknown
    InvalidCommand,

    /// If the new_field data isn't valid
    InvalidField,

    /// OAuth token has expired
    TokenExpired,

    /// An error occurred while processing the request
    InternalServerError(String),

    /// The operation has not been implemented yet
    NotImplemented(String),

    /// You do not have access to this resource, do you have the required scopes?
    MissingScopes,

    /// The requested resource does not exist
    NotFound,

    /// If the vehicle is not "online" when a request is made.
    DeviceNotAvailable,

    /// Vehicle responded with an error - might need a reboot, OTA update, or service
    DeviceUnexpectedResponse,
}

impl ResponseError {
    /// Create a new internal server error
    pub fn internal_error(message: impl Into<String>) -> Self {
        Self::InternalServerError(message.into())
    }

    /// Create a new not implemented error
    pub fn not_implemented(message: impl Into<String>) -> Self {
        Self::NotImplemented(message.into())
    }
}

impl IntoResponse for ResponseError {
    fn into_response(self) -> Response {
        match self {
            Self::InvalidCommand => {
                let error = error("error:invalid_command", "Invalid command");
                (StatusCode::BAD_REQUEST, Json(error)).into_response()
            }
            Self::InvalidField => {
                let error = error("error:invalid_field", "Invalid field");
                (StatusCode::BAD_REQUEST, Json(error)).into_response()
            }
            Self::TokenExpired => (StatusCode::UNAUTHORIZED, ()).into_response(),
            Self::InternalServerError(message) => {
                let error = error("Internal Server Error", "Something went wrong");
                error!("Internal error: {}", message);
                (StatusCode::INTERNAL_SERVER_ERROR, Json(error)).into_response()
            }
            Self::NotImplemented(msg) => {
                let error = error("Not Implemented", format!("Not Implemented: {msg}"));
                (StatusCode::NOT_IMPLEMENTED, Json(error)).into_response()
            }
            Self::MissingScopes => {
                let error = error("Unauthorized missing scopes", "Unauthorized missing scopes");
                (StatusCode::FORBIDDEN, Json(error)).into_response()
            }
            Self::NotFound => {
                let error = error("Not Found", "Not Found");
                (StatusCode::NOT_FOUND, Json(error)).into_response()
            }
            Self::DeviceNotAvailable => {
                error!("Device not available");
                (StatusCode::REQUEST_TIMEOUT, "Device not available").into_response()
            }
            Self::DeviceUnexpectedResponse => {
                error!("Device responded with an error");
                let code = StatusCode::try_from(540).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                (code, "Device responded with an error").into_response()
            }
        }
    }
}

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_error() {
        let error = error("error:invalid_command", "Invalid command");
        // assert_eq!(error.response, ());
        assert_eq!(error.error, "error:invalid_command");
        assert_eq!(error.error_description, "Invalid command");

        let string = serde_json::to_string(&error).unwrap();
        assert_eq!(
            string,
            r#"{"response":null,"error":"error:invalid_command","error_description":"Invalid command","messages":{}}"#
        );
    }
}
