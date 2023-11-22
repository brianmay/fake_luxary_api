//! Test infrastructure
#![allow(clippy::unwrap_used)]

use std::collections::HashSet;

use chrono::Utc;
use envconfig::Envconfig;
use fla_client::Token;
use fla_server::tokens::{self, new_token, ScopeEnum};
use url::Url;

fn get_token_config() -> tokens::Config {
    // This config must match the server.
    tokens::Config {
        secret: "mom-said-yes".to_string(),
    }
}

#[derive(Envconfig, Debug)]
struct Environment {
    #[envconfig(from = "TESLA_ACCESS_TOKEN")]
    tesla_access_token: Option<String>,

    #[envconfig(from = "TESLA_REFRESH_TOKEN")]
    tesla_refresh_token: Option<String>,

    #[envconfig(from = "TESLA_AUTH_API")]
    tesla_auth_api: Option<Url>,

    #[envconfig(from = "TESLA_OWNER_API")]
    tesla_owner_api: Option<Url>,

    #[envconfig(from = "TESLA_STREAMING_API")]
    tesla_streaming_api: Option<Url>,
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
    let env = Environment::init_from_env().unwrap();
    let now = Utc::now();

    if env.tesla_access_token.is_some() {
        let token = Token {
            access_token: env.tesla_access_token.unwrap(),
            refresh_token: env.tesla_refresh_token.unwrap(),
            renew_at: now,
            expires_at: now,
        };

        fla_client::Config::new()
            .auth_url(env.tesla_auth_api.unwrap())
            .owner_url(env.tesla_owner_api.unwrap())
            .streaming_url(env.tesla_streaming_api.unwrap())
            .token(token)
            .build()
            .unwrap()
    } else {
        fla_client::Config::new()
            .auth_url("http://localhost:4080/")
            .owner_url("http://localhost:4080/")
            .streaming_url("ws://localhost:4080/streaming/")
            .token(token)
            .build()
            .unwrap()
    }
}
