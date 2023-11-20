//! Test infrastructure
#![allow(clippy::unwrap_used)]

use std::collections::HashSet;

use fla_client::Token;
use fla_server::tokens::{self, new_token, ScopeEnum};

fn get_token_config() -> tokens::Config {
    // This config must match the server.
    tokens::Config {
        secret: "mom-said-yes".to_string(),
    }
}

/// Get a token for all scopes
///
/// # Returns
///
/// A token with all scopes
///
/// # Panics
///
/// Panics if the token cannot be generated
#[must_use]
// #[allow(dead_code)]
pub fn get_token_for_all_scopes() -> Token {
    let scopes = [
        tokens::ScopeEnum::Openid,
        tokens::ScopeEnum::OfflineAccess,
        tokens::ScopeEnum::UserData,
        tokens::ScopeEnum::VehicleDeviceData,
        tokens::ScopeEnum::VehicleCmds,
        tokens::ScopeEnum::VehicleChargingCmds,
        tokens::ScopeEnum::EnergyDeviceData,
        tokens::ScopeEnum::EnergyCmds,
    ]
    .into_iter()
    .collect::<HashSet<tokens::ScopeEnum>>();
    get_token_with_scopes(&scopes)
}

/// Get a token for the specified scopes
///
/// # Parameters
///
/// * `scopes` - The scopes to get a token for
///
/// # Returns
///
/// A token with the specified scopes
///
/// # Panics
///
/// Panics if the token cannot be generated
#[must_use]
#[allow(clippy::implicit_hasher)]
pub fn get_token_with_scopes(scopes: &HashSet<ScopeEnum>) -> Token {
    let config = get_token_config();
    new_token(&config, scopes).unwrap().into()
}

/// Get a client for connecting to the server
#[must_use]
pub fn get_client() -> fla_client::Client {
    let token = get_token_for_all_scopes();
    get_client_with_token(token)
}

/// Get a client with a specified token
///
/// # Panics
///
/// Panics if the client cannot be created
#[must_use]
pub fn get_client_with_token(token: Token) -> fla_client::Client {
    fla_client::Config::new()
        .auth_url("http://localhost:4080/")
        .owner_url("http://localhost:4080/")
        .streaming_url("ws://localhost:4080/streaming/")
        .token(token)
        .build()
        .unwrap()
}
