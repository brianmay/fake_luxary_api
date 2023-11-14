use std::collections::HashSet;

use fake_luxury_api::tokens::{self, validate_access_token, validate_refresh_token};
use restest::{path, Context, Request};

use http::StatusCode;
use serde::{Deserialize, Serialize};

const CONTEXT: Context = Context::new().with_port(4080);

#[derive(Debug, Serialize, Deserialize)]
struct User {
    id: String,
}

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
        "openid",
        "offline_access",
        "user_data",
        //"vehicle_device_data",
        "vehicle_cmds",
        "vehicle_charging_cmds",
        "energy_device_data",
        "energy_cmds",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>();

    let token = tokens::Token::new(&config, &scopes).unwrap();

    let body = RefreshTokenRequest {
        grant_type: "refresh_token".into(),
        refresh_token: token.refresh_token,
        client_id: "ownerapi".into(),
        // scope has user_data removed but vehicle_device_data added
        scope: "openid offline_access vehicle_device_data vehicle_cmds vehicle_charging_cmds energy_device_data energy_cmds".into(),
    };

    // Test code that use `CONTEXT` for a specific route
    let request = Request::post(path!["oauth2", "v3", "token"])
        .with_header("Content-Type", "application/json")
        // .with_header("Authorization", format!("Bearer {}", token.refresh_token))
        .with_body(body);

    let new_token: TokenResponse = CONTEXT
        .run(request)
        .await
        .expect_status(StatusCode::OK)
        .await;

    // assert!(new_token.access_token != token.access_token);
    // assert!(new_token.refresh_token != token.refresh_token);
    assert!(new_token.expires_in > 0);

    // We do not expect user_data or vehicle_device_data to be in the scopes
    let expected_scopes = [
        "openid",
        "offline_access",
        // "user_data"
        // "vehicle_device_data",
        "vehicle_cmds",
        "vehicle_charging_cmds",
        "energy_device_data",
        "energy_cmds",
    ]
    .into_iter()
    .map(|s| s.to_string())
    .collect::<HashSet<String>>();

    let access_claims = validate_access_token(&new_token.access_token, &config).unwrap();
    assert_eq!(access_claims.purpose, tokens::Purpose::Access);
    assert_eq!(access_claims.scopes, expected_scopes);

    let refresh_claims = validate_refresh_token(&new_token.refresh_token, &config).unwrap();
    assert_eq!(refresh_claims.purpose, tokens::Purpose::Refresh);
    assert_eq!(refresh_claims.scopes, expected_scopes);
}
