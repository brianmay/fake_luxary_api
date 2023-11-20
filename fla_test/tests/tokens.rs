#![allow(clippy::unwrap_used)]

use chrono::Utc;
use fla_client::Token;
use fla_server::tokens::{self, new_token, validate_access_token, validate_refresh_token};
use std::collections::HashSet;

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

    let token: Token = new_token(&config, &scopes).unwrap().into();
    let old_expires_at = token.expires_at;
    let old_renew_at = token.renew_at;

    let mut client = fla_test::get_client_with_token(token);
    client.refresh_token().await.unwrap();

    let new_token = client.token();
    assert!(new_token.expires_at > Utc::now());
    // assert!(new_token.renew_at > Utc::now());
    assert!(new_token.expires_at > old_expires_at);
    assert!(new_token.renew_at > old_renew_at);

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
