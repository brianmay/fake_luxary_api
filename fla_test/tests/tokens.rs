#![allow(clippy::unwrap_used)]

use fla_server::tokens::{self, validate_access_token, validate_refresh_token, Token};
use serde::{Deserialize, Serialize};
use std::collections::HashSet;

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

    let token = Token::new(&config, &scopes).unwrap();
    let client = fla_test::get_client();
    let new_token = client.refresh_token(token.refresh_token).await.unwrap();

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
