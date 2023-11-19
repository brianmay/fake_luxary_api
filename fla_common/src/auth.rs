/// A request to refresh an existing token using an authorization code
#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct AuthorizationCodeRequest {
    client_id: String,
    client_secret: String,
    code: String,
    redirect_uri: String,
    scope: String,
    audience: String,
}

/// A request to refresh an existing token
#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct RefreshTokenRequest {
    pub refresh_token: String,
    pub client_id: String,
    pub scope: String,
}

/// A request to create a new token using client credentials
#[allow(dead_code)]
#[derive(serde::Deserialize, serde::Serialize)]
pub struct ClientCredentialsRequest {
    client_id: String,
    client_secret: String,
    scope: String,
    audience: String,
}

/// The request for a new token
#[derive(serde::Deserialize, serde::Serialize)]
#[serde(tag = "grant_type")]
pub enum TokenRequest {
    /// A request to refresh an existing token using an authorization code
    #[serde(rename = "authorization_code")]
    AuthorizationCode(AuthorizationCodeRequest),

    /// A request to refresh an existing token
    #[serde(rename = "refresh_token")]
    RefreshToken(RefreshTokenRequest),

    /// A request to create a new token using client credentials
    #[serde(rename = "client_credentials")]
    ClientCredentials(ClientCredentialsRequest),
}

/// Raw Tesla token from API
#[derive(serde::Deserialize, serde::Serialize)]
pub struct TokenResult {
    pub access_token: String,
    pub refresh_token: String,
    pub id_token: String,
    pub token_type: String,
    pub expires_in: u64,
}
