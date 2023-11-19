use fla_common::auth::{RefreshTokenRequest, TokenRequest, TokenResult};

pub struct Config {
    auth_url: Option<String>,
    owner_url: Option<String>,
    streaming_url: Option<String>,
    token: Option<String>,
}

impl Config {
    pub fn new() -> Self {
        Self {
            auth_url: None,
            owner_url: None,
            streaming_url: None,
            token: None,
        }
    }

    pub fn auth_url(mut self, auth_url: impl Into<String>) -> Self {
        self.auth_url = Some(auth_url.into());
        self
    }

    pub fn owner_url(mut self, owner_url: impl Into<String>) -> Self {
        self.owner_url = Some(owner_url.into());
        self
    }

    pub fn streaming_url(mut self, streaming_url: impl Into<String>) -> Self {
        self.streaming_url = Some(streaming_url.into());
        self
    }

    pub fn token(mut self, token: impl Into<String>) -> Self {
        self.token = Some(token.into());
        self
    }

    pub fn build(self) -> Client {
        Client {
            auth_url: self
                .auth_url
                .unwrap_or_else(|| "https://auth.tesla.com/oauth2/v3/token".into()),
            owner_url: self
                .owner_url
                .unwrap_or_else(|| "https://owner-api.teslamotors.com/".into()),
            // FIXME: In China should be wss://streaming.vn.cloud.tesla.cn/streaming/
            streaming_url: self
                .streaming_url
                .unwrap_or_else(|| "wss://streaming.vn.teslamotors.com/streaming/".into()),
            token: self.token.unwrap_or_else(|| "test_token".into()),
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Self::new()
    }
}

/// The client configuration
pub struct Client {
    auth_url: String,
    owner_url: String,
    streaming_url: String,
    token: String,
}

impl Client {
    pub async fn refresh_token(&self, token: String) -> Result<TokenResult, reqwest::Error> {
        let body = TokenRequest::RefreshToken(RefreshTokenRequest {
            refresh_token: token,
            client_id: "ownerapi".into(),
            // scope has user_data removed but vehicle_device_data added
            scope: "openid offline_access vehicle_device_data vehicle_cmds vehicle_charging_cmds energy_device_data energy_cmds".into(),
        });

        let url = format!("{}oauth2/v3/token", self.auth_url);
        let new_token = reqwest::Client::new()
            .post(url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .error_for_status()?
            .json::<TokenResult>()
            .await?;

        Ok(new_token)
    }
}
