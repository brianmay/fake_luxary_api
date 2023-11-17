//! Shared types for all API

use serde::Serialize;

/// A vehicle
#[allow(dead_code)]
#[derive(Debug, Serialize, Clone)]
pub struct Vehicle {
    /// Vehicle ID for owner-api endpoint.
    pub id: u64,
    /// Vehicle ID for streaming or Auto park API.
    pub vehicle_id: u64,

    /// Vehicle identification number.
    pub vin: String,

    /// Vehicle display name.
    pub display_name: String,

    /// Vehicle option codes.
    pub option_codes: String,

    /// Vehicle color.
    pub color: Option<String>,

    /// Vehicle tokens.
    pub tokens: Vec<String>,

    /// Vehicle state.
    pub state: String,

    /// Vehicle in service.
    pub in_service: bool,

    /// Vehicle ID string.
    pub id_s: String,

    /// Vehicle calendar enabled.
    pub calendar_enabled: bool,

    /// Vehicle API version.
    pub api_version: u8,

    /// Vehicle backseat token.
    pub backseat_token: Option<String>,

    /// Vehicle backseat token updated at.
    pub backseat_token_updated_at: Option<String>,
}
