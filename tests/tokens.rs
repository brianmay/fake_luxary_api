use fake_luxury_api::tokens::{self, validate_access_token, validate_refresh_token};
use reqwest::Client;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

mod common;

#[derive(Serialize)]
struct RefreshTokenRequest {
    grant_type: String,
    refresh_token: String,
    client_id: String,
    scope: String,
}

/// Raw Tesla token from API
#[allow(dead_code)]
#[derive(Deserialize, Debug)]
pub struct TokenResponse {
    access_token: String,
    refresh_token: String,
    id_token: String,
    token_type: String,
    expires_in: u64,
}

#[tokio::test]
async fn test_renew_token() {
    // This config must match the server.
    let config = tokens::Config {
        secret: "mom-said-yes".to_string(),
    };

    let scopes = [
        tokens::ScopeEnum::Openid,
        tokens::ScopeEnum::OfflineAccess,
        tokens::ScopeEnum::UserData,
        // tokens::ScopeEnum::VehicleDeviceData,
        tokens::ScopeEnum::VehicleCmds,
        tokens::ScopeEnum::VehicleChargingCmds,
        tokens::ScopeEnum::EnergyDeviceData,
        tokens::ScopeEnum::EnergyCmds,
    ]
    .into_iter()
    .collect::<HashSet<tokens::ScopeEnum>>();

    let token = tokens::Token::new(&config, &scopes).unwrap();

    let body = RefreshTokenRequest {
        grant_type: "refresh_token".into(),
        refresh_token: token.refresh_token,
        client_id: "ownerapi".into(),
        // scope has user_data removed but vehicle_device_data added
        scope: "openid offline_access vehicle_device_data vehicle_cmds vehicle_charging_cmds energy_device_data energy_cmds".into(),
    };

    let url = format!("{}oauth2/v3/token", common::URL);
    let new_token = Client::new()
        .post(url)
        .json(&body)
        .header("Content-Type", "application/json")
        .send()
        .await
        .unwrap()
        .error_for_status()
        .unwrap()
        .json::<TokenResponse>()
        .await
        .unwrap();

    // assert!(new_token.access_token != token.access_token);
    // assert!(new_token.refresh_token != token.refresh_token);
    assert!(new_token.expires_in > 0);

    // We do not expect user_data or vehicle_device_data to be in the scopes
    let expected_scopes = [
        tokens::ScopeEnum::Openid,
        tokens::ScopeEnum::OfflineAccess,
        // tokens::ScopeEnum::UserData,
        // tokens::ScopeEnum::VehicleDeviceData,
        tokens::ScopeEnum::VehicleCmds,
        tokens::ScopeEnum::VehicleChargingCmds,
        tokens::ScopeEnum::EnergyDeviceData,
        tokens::ScopeEnum::EnergyCmds,
    ]
    .into_iter()
    .collect::<HashSet<tokens::ScopeEnum>>();

    let access_claims = validate_access_token(&new_token.access_token, &config).unwrap();
    assert_eq!(access_claims.purpose, tokens::Purpose::Access);
    assert_eq!(access_claims.scopes, expected_scopes);

    let refresh_claims = validate_refresh_token(&new_token.refresh_token, &config).unwrap();
    assert_eq!(refresh_claims.purpose, tokens::Purpose::Refresh);
    assert_eq!(refresh_claims.scopes, expected_scopes);
}
