use serde::Deserialize;
use thiserror::Error;

#[derive(Debug, Deserialize)]
struct ApiErrorEnvelope {
    error: ApiErrorBody,
}

#[derive(Debug, Deserialize)]
struct ApiErrorBody {
    message: String,
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("Missing env var: {0}")]
    Env(#[from] std::env::VarError),

    #[error("String {0} did not parse as valid URL")]
    InvalidUrl(#[from] url::ParseError),

    #[error("Request error: {0}")]
    Reqwest(#[from] reqwest::Error),

    #[error("de/serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("API error {status}: {message}")]
    Api { status: u16, message: String },

    #[error("HTTP {status}: {body}")]
    Unexpected { status: u16, body: String },

    #[error("Stream closed before producing a result")]
    StreamClosed,
}

pub type Result<T, E = Error> = core::result::Result<T, E>;

impl Error {
    pub(crate) fn parse_api_error(status: u16, body: &str) -> Self {
        match serde_json::from_str::<ApiErrorEnvelope>(body) {
            Ok(envelope) => Self::Api {
                status,
                message: envelope.error.message,
            },
            Err(_) => Self::Unexpected {
                status,
                body: body.to_string(),
            },
        }
    }
}
