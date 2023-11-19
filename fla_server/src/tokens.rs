//! Tokens for authenticating with the API

use std::{collections::HashSet, str::FromStr};

use chrono::{DateTime, Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use thiserror::Error;

/// The purpose of the token
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq)]
pub enum Purpose {
    /// The token is for accessing the API
    Access,

    /// The token is for refreshing the access token
    Refresh,
}

/// Our claims struct for access tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct AccessClaims {
    /// The purpose of the token
    pub purpose: Purpose,
    /// The expiration time of the token
    pub exp: usize,
    /// The scopes of the token
    pub scopes: HashSet<ScopeEnum>,
}

/// The possible scopes of the token
#[derive(Debug, Serialize, Deserialize, Eq, PartialEq, Hash, Clone, Copy)]
#[serde(rename_all = "snake_case")]
pub enum ScopeEnum {
    /// The user's openid
    Openid,

    /// The user's offline access
    OfflineAccess,

    /// The user's data
    UserData,

    /// The user's vehicle device data
    VehicleDeviceData,

    /// The user's vehicle commands
    VehicleCmds,

    /// The user's vehicle charging commands
    VehicleChargingCmds,

    /// The user's energy device data
    EnergyDeviceData,

    /// The user's energy commands
    EnergyCmds,
}

impl FromStr for ScopeEnum {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "openid" => Ok(Self::Openid),
            "offline_access" => Ok(Self::OfflineAccess),
            "user_data" => Ok(Self::UserData),
            "vehicle_device_data" => Ok(Self::VehicleDeviceData),
            "vehicle_cmds" => Ok(Self::VehicleCmds),
            "vehicle_charging_cmds" => Ok(Self::VehicleChargingCmds),
            "energy_device_data" => Ok(Self::EnergyDeviceData),
            "energy_cmds" => Ok(Self::EnergyCmds),
            _ => Err(()),
        }
    }
}

/// Our claims struct for refresh tokens
#[derive(Debug, Serialize, Deserialize)]
pub struct RefreshClaims {
    /// The purpose of the token
    pub purpose: Purpose,
    /// The expiration time of the token
    pub exp: usize,
    /// The scopes of the token
    pub scopes: HashSet<ScopeEnum>,
}

/// The configuration for Tokens
pub struct Config {
    /// The secret used to sign the tokens
    pub secret: String,
}

/// A new token
#[derive(Debug)]
pub struct Token {
    /// The access token
    pub access_token: String,
    /// The refresh token
    pub refresh_token: String,
    /// The expiration time of the token
    pub expires_at: DateTime<Utc>,
}

/// An error generating a token
#[derive(Error, Debug)]
pub enum TokenGenerationError {
    /// The token was invalid
    #[error("{0}")]
    TokenGenerationError(#[from] jsonwebtoken::errors::Error),
    /// The date stamp was too big (should never happen)
    #[error("Could not convert timestamp")]
    TimestampError,
}

impl Token {
    /// Generate a new token with the given scopes
    ///
    /// # Errors
    ///
    /// If the token cannot be generated, an error will be returned.
    pub fn new(config: &Config, scopes: &HashSet<ScopeEnum>) -> Result<Self, TokenGenerationError> {
        let encoding_key = EncodingKey::from_secret(config.secret.as_ref());
        let expires_at = Utc::now() + Duration::minutes(10);

        let timestamp = usize::try_from(expires_at.timestamp())
            .map_err(|_| TokenGenerationError::TimestampError)?;

        let access_token = encode(
            &Header::default(),
            &AccessClaims {
                purpose: Purpose::Access,
                exp: timestamp,
                scopes: scopes.clone(),
            },
            &encoding_key,
        )?;

        let refresh_token = encode(
            &Header::default(),
            &RefreshClaims {
                purpose: Purpose::Refresh,
                exp: timestamp,
                scopes: scopes.clone(),
            },
            &encoding_key,
        )?;

        let token = Self {
            access_token,
            refresh_token,
            expires_at,
        };

        Ok(token)
    }
}

/// An error validating a token
#[derive(Error, Debug)]
pub enum TokenValidationError {
    /// The token was invalid
    #[error("{0}")]
    TokenValidationError(#[from] jsonwebtoken::errors::Error),

    /// The token was the wrong type
    #[error("The token was the wrong type")]
    WrongTokenType,
}

/// Validate an access token
///
/// # Errors
///
/// If the token is invalid, an error will be returned.
pub fn validate_access_token(
    token: &str,
    config: &Config,
) -> Result<AccessClaims, TokenValidationError> {
    let decoding_key = DecodingKey::from_secret(config.secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let claims: AccessClaims = decode(token, &decoding_key, &validation)?.claims;
    if claims.purpose != Purpose::Access {
        return Err(TokenValidationError::WrongTokenType);
    }
    Ok(claims)
}

/// Validate a refresh token
///
/// # Errors
///
/// If the token is invalid, an error will be returned.
pub fn validate_refresh_token(
    token: &str,
    config: &Config,
) -> Result<RefreshClaims, TokenValidationError> {
    let decoding_key = DecodingKey::from_secret(config.secret.as_ref());
    let validation = Validation::new(Algorithm::HS256);
    let claims: RefreshClaims = decode(token, &decoding_key, &validation)?.claims;
    if claims.purpose != Purpose::Refresh {
        return Err(TokenValidationError::WrongTokenType);
    }
    Ok(claims)
}
