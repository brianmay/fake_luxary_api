use std::{collections::HashSet, str::FromStr, time::Duration};

use chrono::{DateTime, Utc};
use fla_common::{
    auth::{RawToken, RefreshTokenRequest, TokenRequest},
    responses::{VehicleDataResponse, VehicleResponse, VehiclesResponse},
    streaming::{
        FromServerStreamingMessage, StreamingDataOptional, StreamingFields,
        ToServerStreamingMessage,
    },
    types::{Timestamp, VehicleDataEndpoint, VehicleId},
};
use futures_util::{SinkExt, StreamExt};
use http::StatusCode;
use tap::Pipe;
use thiserror::Error;
use tokio::{select, sync::mpsc};
use tokio_tungstenite::{
    connect_async,
    tungstenite::{
        protocol::{frame::coding::CloseCode, CloseFrame},
        Error, Message,
    },
};
use tracing::{debug, error};
use url::Url;

/// A new token
#[derive(Debug)]
pub struct Token {
    /// The access token
    pub access_token: String,
    /// The refresh token
    pub refresh_token: String,
    /// Time we should renew the token
    pub renew_at: DateTime<Utc>,
    /// The expiration time of the token
    pub expires_at: DateTime<Utc>,
}

impl From<RawToken> for Token {
    fn from(token: RawToken) -> Self {
        let expires_in = Duration::from_secs(token.expires_in);
        let renew_in = expires_in
            .checked_sub(Duration::from_secs(60 * 60))
            .unwrap_or_default();

        let expires_in = chrono::Duration::from_std(expires_in)
            .unwrap_or_else(|_| chrono::Duration::seconds(60));

        let renew_in =
            chrono::Duration::from_std(renew_in).unwrap_or_else(|_| chrono::Duration::seconds(60));

        let now = chrono::Utc::now();
        let renew_at = now + renew_in;
        let expires_at = now + expires_in;

        Token {
            access_token: token.access_token,
            refresh_token: token.refresh_token,
            renew_at,
            expires_at,
        }
    }
}

pub struct NoTokens {}

pub struct HasToken {
    token: Token,
}

pub struct Config<T> {
    auth_url: Option<String>,
    owner_url: Option<String>,
    streaming_url: Option<String>,
    extra: T,
}

#[derive(Error, Debug)]
pub enum ConfigBuildError {
    #[error("{0}")]
    UrlParseError(#[from] url::ParseError),
}

impl Config<NoTokens> {
    pub fn new() -> Self {
        Self {
            auth_url: None,
            owner_url: None,
            streaming_url: None,
            extra: NoTokens {},
        }
    }

    pub fn token(self, token: Token) -> Config<HasToken> {
        Config::<HasToken> {
            auth_url: self.auth_url,
            owner_url: self.owner_url,
            streaming_url: self.streaming_url,
            extra: HasToken { token },
        }
    }
}

impl Default for Config<NoTokens> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> Config<T> {
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
}

impl Config<HasToken> {
    pub fn build(self) -> Result<Client, ConfigBuildError> {
        Client {
            auth_url: self
                .auth_url
                .unwrap_or_else(|| "https://auth.tesla.com/oauth2/v3/token".into())
                .pipe(|x| Url::parse(&x))?,
            owner_url: self
                .owner_url
                .unwrap_or_else(|| "https://owner-api.teslamotors.com/".into())
                .pipe(|x| Url::parse(&x))?,
            // FIXME: In China should be wss://streaming.vn.cloud.tesla.cn/streaming/
            streaming_url: self
                .streaming_url
                .unwrap_or_else(|| "wss://streaming.vn.teslamotors.com/streaming/".into())
                .pipe(|x| Url::parse(&x))?,
            token: self.extra.token,
        }
        .pipe(Ok)
    }
}

/// The client configuration
pub struct Client {
    auth_url: Url,
    owner_url: Url,
    streaming_url: Url,
    token: Token,
}

#[derive(Error, Debug)]
pub enum StreamingFieldError {
    #[error("Invalid Time")]
    InvalidTime,

    #[error("Field {0} was not expected")]
    FieldMissingError(usize),

    #[error("Error with field {0:?} number {1}")]
    FieldError(StreamingFields, usize),
}

fn deserialize_fields(
    id: VehicleId,
    str: &str,
    fields: &[StreamingFields],
) -> Result<StreamingDataOptional, StreamingFieldError> {
    let mut split = str.split(',');

    let time_str = split.next();
    let time = match time_str {
        Some(time_str) => time_str
            .parse::<Timestamp>()
            .map_err(|_| StreamingFieldError::InvalidTime)?,
        None => return Err(StreamingFieldError::InvalidTime),
    };

    let mut data = StreamingDataOptional::new(id, time);

    split
        .enumerate()
        .map(|(n, value)| -> Result<(), StreamingFieldError> {
            let field = fields
                .get(n)
                .ok_or(StreamingFieldError::FieldMissingError(n))?;

            match field {
                StreamingFields::Speed => parse_field(&mut data.speed, value, field, n)?,
                StreamingFields::Odometer => parse_field(&mut data.odometer, value, field, n)?,
                StreamingFields::Soc => parse_field(&mut data.soc, value, field, n)?,
                StreamingFields::Elevation => parse_field(&mut data.elevation, value, field, n)?,
                StreamingFields::EstHeading => parse_field(&mut data.speed, value, field, n)?,
                StreamingFields::EstLat => parse_field(&mut data.est_lat, value, field, n)?,
                StreamingFields::EstLng => parse_field(&mut data.est_lng, value, field, n)?,
                StreamingFields::Power => parse_field(&mut data.power, value, field, n)?,
                StreamingFields::ShiftState => parse_field(&mut data.shift_state, value, field, n)?,
                StreamingFields::Range => parse_field(&mut data.range, value, field, n)?,
                StreamingFields::EstRange => parse_field(&mut data.est_range, value, field, n)?,
                StreamingFields::Heading => parse_field(&mut data.heading, value, field, n)?,
            }

            Ok(())
        })
        .collect::<Result<Vec<_>, StreamingFieldError>>()?;

    todo!()
}

fn parse_field<T: FromStr>(
    data: &mut Option<T>,
    value: &str,
    field: &StreamingFields,
    n: usize,
) -> Result<(), StreamingFieldError> {
    *data = value
        .parse::<T>()
        .map_err(|_| StreamingFieldError::FieldError(*field, n))?
        .pipe(Some);
    Ok(())
}

impl Client {
    pub async fn refresh_token(&mut self) -> Result<(), reqwest::Error> {
        let body = TokenRequest::RefreshToken(RefreshTokenRequest {
            refresh_token: self.token.refresh_token.clone(),
            client_id: "ownerapi".into(),
            // scope has user_data removed but vehicle_device_data added
            scope: "openid offline_access vehicle_device_data vehicle_cmds vehicle_charging_cmds energy_device_data energy_cmds".into(),
        });

        let url = format!("{}oauth2/v3/token", self.auth_url);
        let token: Token = reqwest::Client::new()
            .post(url)
            .json(&body)
            .header("Content-Type", "application/json")
            .send()
            .await?
            .error_for_status()?
            .json::<RawToken>()
            .await?
            .into();

        self.token = token;
        Ok(())
    }

    pub async fn check_refresh_token(&mut self) -> Result<(), reqwest::Error> {
        let now = chrono::Utc::now();
        let renew_at = self.token.renew_at;
        let expires_at = self.token.expires_at;

        if now > renew_at || now > expires_at {
            self.refresh_token().await?;
        }

        Ok(())
    }

    pub async fn get_vehicles(&self) -> Result<VehiclesResponse, reqwest::Error> {
        let url = format!("{}api/1/vehicles", self.owner_url);
        let vehicles = reqwest::Client::new()
            .get(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<VehiclesResponse>()
            .await?;

        Ok(vehicles)
    }

    pub async fn get_vehicle(&self, id: u64) -> Result<VehicleResponse, reqwest::Error> {
        let url = format!("{}api/1/vehicles/{}", self.owner_url, id);
        let vehicles = reqwest::Client::new()
            .get(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<VehicleResponse>()
            .await?;

        Ok(vehicles)
    }

    pub async fn get_vehicle_data(
        &self,
        id: u64,
        endpoints: HashSet<VehicleDataEndpoint>,
    ) -> Result<VehicleDataResponse, reqwest::Error> {
        let endpoints = endpoints
            .into_iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        let query = [("endpoints", endpoints)];

        let url = format!("{}api/1/vehicles/{}/vehicle_data", self.owner_url, id);
        let vehicles = reqwest::Client::new()
            .get(url)
            .query(&query)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<VehicleDataResponse>()
            .await?;

        Ok(vehicles)
    }

    pub async fn wake_up(&self, id: u64) -> Result<VehicleResponse, reqwest::Error> {
        let url = format!("{}api/1/vehicles/{}/wake_up", self.owner_url, id);
        let vehicle = reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .error_for_status()?
            .json::<VehicleResponse>()
            .await?;

        Ok(vehicle)
    }

    // FIXME: This is yuck
    pub fn streaming(
        &self,
        id: u64,
        fields: Vec<StreamingFields>,
    ) -> Result<mpsc::Receiver<StreamingDataOptional>, Error> {
        let (tx, rx) = mpsc::channel(10);

        let token = self.token.access_token.clone();
        let url = self.streaming_url.clone();

        let string_fields = fields
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        tokio::spawn(async move {
            let msg = ToServerStreamingMessage::DataSubscribeOauth {
                token,
                value: string_fields,
                tag: id,
            };
            let (mut socket, response) = connect_async(url).await.unwrap();
            assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);

            let msg = serde_json::to_string(&msg).unwrap();
            socket.send(Message::Text(msg)).await.unwrap();

            loop {
                select! {
                  maybe_msg = socket.next()  => {
                    match maybe_msg {
                        Some(Ok(Message::Text(msg))) => {
                            let msg: FromServerStreamingMessage = serde_json::from_str(&msg).unwrap();
                            match msg {
                                FromServerStreamingMessage::ControlHello {
                                    connection_timeout: _,
                                } => {
                                    debug!("Received: {msg:?}");
                                }
                                FromServerStreamingMessage::DataUpdate { tag, value } => {
                                    println!("Received: {tag} {value}");
                                    match deserialize_fields(tag, &value, &fields) {
                                        Ok(data) => {
                                            tx.send(data)
                                                .await
                                                .unwrap_or_else(|err| error!("Error sending data: {err}"));
                                        }
                                        Err(err) => {
                                            error!("Error deserializing data: {err}");
                                        }
                                    }
                                }
                                FromServerStreamingMessage::DataError {
                                    tag,
                                    error_type,
                                    value,
                                } => {
                                    error!("Received data error: {tag} {error_type:?} {value}");
                                }
                            }
                        }
                        Some(Ok(msg)) => {
                            println!("Received unexpected: {msg:?}");
                        }
                        Some(Err(e)) => {
                            println!("Error: {e:?}");
                        }
                        None => {
                            println!("Disconnected");
                            break;
                        }
                    }
                  }
                  _ = tx.closed() => {
                    println!("Client disconnected");
                    break;
                  }
                }
            }

            socket
                .close(Some(CloseFrame {
                    code: CloseCode::Normal,
                    reason: "I hate you".into(),
                }))
                .await
                .unwrap_or_else(|err| error!("Error closing socket: {err}"));
        });

        Ok(rx)
    }

    /// Get the token (for testing)
    pub fn token(&self) -> &Token {
        &self.token
    }
}
