//! Responses to Tesla API calls

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::{VehicleData, VehicleDefinition};

/// An error from the Tesla API
#[derive(Serialize, Deserialize, Debug)]
pub struct TeslaError {
    /// The error code
    pub error: String,

    /// The error description
    pub error_description: String,

    /// Additional messages
    pub messages: HashMap<String, String>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct TeslaResponseSuccess<T> {
    pub response: T,
}

/// A response from the Tesla API
#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
pub enum TeslaResponse<T> {
    Success { response: T },
    Error(TeslaError),
}

impl<T: Serialize> TeslaResponse<T> {
    /// Generate a success response
    pub fn success(response: T) -> Self {
        Self::Success { response }
    }

    /// Generate an error response
    pub fn error(error: impl Into<String>, description: impl Into<String>) -> Self {
        Self::Error(TeslaError {
            error: error.into(),
            error_description: description.into(),
            messages: HashMap::new(),
        })
    }

    /// Get the response
    #[must_use]
    pub fn get_response(self) -> Option<T> {
        match self {
            Self::Success { response } => Some(response),
            Self::Error(_) => None,
        }
    }

    /// Get the error
    #[must_use]
    pub fn get_error(&self) -> Option<&TeslaError> {
        match self {
            Self::Success { .. } => None,
            Self::Error(error) => Some(error),
        }
    }
}

/// Generate an error response
pub fn error(error: impl Into<String>, description: impl Into<String>) -> TeslaResponse<()> {
    TeslaResponse::error(error, description)
}

pub type VehiclesResponse = TeslaResponse<Vec<VehicleDefinition>>;
pub type VehicleResponse = TeslaResponse<VehicleDefinition>;
pub type VehicleDataResponse = TeslaResponse<VehicleData>;

#[cfg(test)]
mod test {
    #![allow(clippy::unwrap_used)]
    use std::assert_eq;

    use super::*;

    #[test]
    fn test_error() {
        let error = error("error:invalid_command", "Invalid command");
        let error = error.get_error().unwrap();
        assert_eq!(error.error, "error:invalid_command");
        assert_eq!(error.error_description, "Invalid command");

        let string = serde_json::to_string(&error).unwrap();
        assert_eq!(
            string,
            r#"{"error":"error:invalid_command","error_description":"Invalid command","messages":{}}"#
        );
    }
}
