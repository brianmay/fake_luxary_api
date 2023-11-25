use std::{collections::HashSet, str::FromStr, time::Duration};

use chrono::{DateTime, Utc};
use fla_common::{
    auth::{RawToken, RefreshTokenRequest, TokenRequest},
    responses::{
        TeslaResponse, TeslaResponseSuccess, VehicleDataResponse, VehicleResponse, VehiclesResponse,
    },
    simulator::SimulationStateEnum,
    streaming::{
        DataError, FromServerStreamingMessage, StreamingData, StreamingFields,
        ToServerStreamingMessage,
    },
    types::{Timestamp, VehicleData, VehicleDataEndpoint, VehicleGuid, VehicleId},
};
use futures_util::{SinkExt, StreamExt};
use http::StatusCode;
use tap::{Pipe, Tap};
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
                .unwrap_or_else(|| "https://auth.tesla.com/".into())
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
    #[error("Invalid Time '{0}'")]
    InvalidTime(String),

    #[error("Field '{0}' was not expected")]
    FieldMissingError(usize),

    #[error("Error with field '{0:?}' number '{1}' value '{2}'")]
    FieldError(StreamingFields, usize, String),
}

fn deserialize_fields(
    id: VehicleGuid,
    str: &str,
    fields: &[StreamingFields],
) -> Result<StreamingData, StreamingFieldError> {
    let mut split = str.split(',');

    let time_str = split.next();
    let time = match time_str {
        Some(time_str) => time_str
            .parse::<Timestamp>()
            .map_err(|_| StreamingFieldError::InvalidTime(time_str.to_string()))?,
        None => return Err(StreamingFieldError::InvalidTime("<not supplied>".into())),
    };

    let mut data = StreamingData::new(id, time);

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
                StreamingFields::EstHeading => parse_field(&mut data.est_heading, value, field, n)?,
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

    Ok(data)
}

fn parse_field<T: FromStr>(
    data: &mut Option<T>,
    value: &str,
    field: &StreamingFields,
    n: usize,
) -> Result<(), StreamingFieldError> {
    let result = if value.is_empty() {
        None
    } else {
        value
            .parse::<T>()
            .map_err(|_| StreamingFieldError::FieldError(*field, n, value.to_string()))?
            .pipe(Some)
    };

    *data = result;
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
        let text = reqwest::Client::new()
            .get(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        let jd = &mut serde_json::Deserializer::from_str(&text);
        let result: Result<VehiclesResponse, _> = serde_path_to_error::deserialize(jd);
        let vehicles = result
            .map_err(|err| {
                panic!("Error deserializing vehicle: {}", err);
            })
            .unwrap();

        Ok(vehicles)
    }

    pub async fn get_vehicle(&self, id: VehicleId) -> Result<VehicleResponse, reqwest::Error> {
        let url = format!("{}api/1/vehicles/{}", self.owner_url, id.to_string());
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
        id: VehicleId,
        endpoints: &HashSet<VehicleDataEndpoint>,
    ) -> Result<VehicleDataResponse, reqwest::Error> {
        let endpoints = endpoints
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(";");

        let query = [("endpoints", endpoints)];

        let url = format!(
            "{}api/1/vehicles/{}/vehicle_data",
            self.owner_url,
            id.to_string()
        );
        let text = reqwest::Client::new()
            .get(url)
            .query(&query)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            // .tap(|x| println!("Request: {:#?}", x))
            .send()
            .await?
            .error_for_status()?
            .text()
            .await?;

        {
            let json: serde_json::Value = serde_json::from_str(&text).unwrap();
            println!("Response: {:#?}", json);
        }

        let jd = &mut serde_json::Deserializer::from_str(&text);
        let result: Result<TeslaResponseSuccess<VehicleData>, _> =
            serde_path_to_error::deserialize(jd);
        let vehicles = result
            .map_err(|err| {
                panic!("Error deserializing vehicle: {}", err);
            })
            .unwrap();

        Ok(TeslaResponse::success(vehicles.response))
    }

    pub async fn wake_up(&self, id: VehicleId) -> Result<VehicleResponse, reqwest::Error> {
        let url = format!(
            "{}api/1/vehicles/{}/wake_up",
            self.owner_url,
            id.to_string()
        );
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

    pub async fn simulate(
        &self,
        id: VehicleId,
        state: SimulationStateEnum,
    ) -> Result<(), reqwest::Error> {
        let url = format!(
            "{}api/1/vehicles/{}/simulate",
            self.owner_url,
            id.to_string()
        );
        reqwest::Client::new()
            .post(url)
            .header("Content-Type", "application/json")
            .bearer_auth(&self.token.access_token)
            .json(&state)
            .send()
            .await?
            .error_for_status()?;

        Ok(())
    }

    // FIXME: This is yuck
    pub fn streaming(
        &self,
        id: VehicleGuid,
        fields: Vec<StreamingFields>,
    ) -> Result<mpsc::Receiver<StreamingData>, Error> {
        let (tx, rx) = mpsc::channel(10);

        let token = self.token.access_token.clone();
        let url = self.streaming_url.clone();

        let string_fields = fields
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",");

        tokio::spawn(async move {
            let (mut socket, response) = connect_async(url).await.unwrap();
            assert_eq!(response.status(), StatusCode::SWITCHING_PROTOCOLS);

            let msg = ToServerStreamingMessage::DataSubscribeOauth {
                token,
                value: string_fields,
                tag: id.to_string(),
            };
            let msg = serde_json::to_string(&msg).unwrap();
            debug!("Sending: {:#?}", msg);
            socket.send(Message::Text(msg)).await.unwrap();

            loop {
                let result = select! {
                  maybe_msg = socket.next()  => {
                    match maybe_msg {
                        Some(Ok(Message::Text(msg))) => {
                            msg
                            .tap(|x| debug!("Received text message: {:#?}", x))
                            .pipe(|msg| process_message(msg, &fields, &tx)).await
                        }
                        Some(Ok(Message::Binary(msg))) => {
                            let msg = String::from_utf8(msg);
                            match msg {
                                Ok(msg) => {
                                    debug!("Received binary: {msg:?}");
                                    process_message(msg, &fields, &tx).await
                                }
                                Err(err) => {
                                    error!("Error decoding message: {err}");
                                    Err(format!("Error decoding message: {err}"))
                                }
                            }
                        }
                        Some(Ok(msg)) => {
                            debug!("Received unexpected: {msg:?}");
                            Ok(())
                        }
                        Some(Err(e)) => {
                            error!("Error: {e:?}");
                            Err(format!("Error: {e:?}"))
                        }
                        None => {
                            debug!("Disconnected");
                            break;
                        }
                    }
                  }
                  _ = tx.closed() => {
                    debug!("Client disconnected");
                    break;
                  }
                };

                if let Err(err) = result {
                    error!("Error processing message: {err}");
                    break;
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

async fn process_message(
    msg: String,
    fields: &[StreamingFields],
    tx: &mpsc::Sender<StreamingData>,
) -> Result<(), String> {
    let msg: FromServerStreamingMessage = serde_json::from_str(&msg).unwrap();
    match msg {
        FromServerStreamingMessage::ControlHello {
            connection_timeout: _,
        } => {
            debug!("Received: {msg:?}");
            Ok(())
        }
        FromServerStreamingMessage::DataUpdate { tag, value } => {
            let vehicle_id = tag.parse::<VehicleGuid>().unwrap();

            match deserialize_fields(vehicle_id, &value, fields) {
                Ok(data) => {
                    tx.send(data)
                        .await
                        .unwrap_or_else(|err| error!("Error sending data: {err}"));
                    Ok(())
                }
                Err(err) => {
                    error!("Error deserializing data: {err}");
                    Ok(())
                }
            }
        }
        FromServerStreamingMessage::DataError(data_error) => {
            let DataError {
                tag,
                error_type,
                value,
            } = data_error;
            error!("Received data error: {tag} {error_type:?} {value}");
            match error_type {
                fla_common::streaming::ErrorType::VehicleDisconnected => {
                    error!("Vehicle disconnected");
                    Err(format!("Vehicle disconnected: {value}"))
                }
                fla_common::streaming::ErrorType::VehicleError => {
                    error!("Vehicle error");
                    Err(format!("Vehicle error: {value}"))
                }
                fla_common::streaming::ErrorType::ClientError => {
                    error!("Client error");
                    Err(format!("Client error: {value}"))
                }
            }
        }
    }
}
