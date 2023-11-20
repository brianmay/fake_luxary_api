//! Responses to Tesla API calls

use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::types::{VehicleData, VehicleDefinition};

/// A response from the Tesla API
#[derive(Serialize, Deserialize, Debug)]
pub struct TeslaResponse<T> {
    /// JSON representing the response. May be a scalar, an array or a object depending on the specific request
    pub response: T,

    /// Short error "enum" like "not_found", "invalid_resource", "invalid_password"
    pub error: String,

    /// Additional error information
    pub error_description: String,

    /// Data validation issues, especially on a 422 responses
    pub messages: HashMap<String, Vec<String>>,
}

impl<T: Serialize> TeslaResponse<T> {
    /// Generate a success response
    pub fn success(response: T) -> Self {
        Self {
            response,
            error: String::new(),
            error_description: String::new(),
            messages: HashMap::new(),
        }
    }

    /// Generate an error response
    pub fn error(response: T, error: impl Into<String>, description: impl Into<String>) -> Self {
        Self {
            response,
            error: error.into(),
            error_description: description.into(),
            messages: HashMap::new(),
        }
    }
}

/// Generate an error response
pub fn error(error: impl Into<String>, description: impl Into<String>) -> TeslaResponse<()> {
    TeslaResponse::error((), error, description)
}

pub type VehiclesResponse = TeslaResponse<Vec<VehicleDefinition>>;
pub type VehicleResponse = TeslaResponse<VehicleDefinition>;
pub type VehicleDataResponse = TeslaResponse<VehicleData>;
