mod core;
mod error;
mod mode;
mod request;
mod stream;

pub use core::{AsyncClient, AsyncClientBuilder, Client, ClientBuilder};
pub use error::{Error, Result};
pub use request::{AsyncResponseRequestBuilder, ResponseRequestBuilder};
pub use stream::{AsyncResponseStream, ResponseEvent, ResponseStream};

const RESPONSES_PATH: [&str; 1] = ["responses"];

pub const DEFAULT_BASE_URL: &str = "https://api.openai.com/v1";
pub const BASE_URL_ENV_VAR: &str = "OPEN_RESPONSES_BASE_URL";
pub const API_KEY_ENV_VAR: &str = "OPEN_RESPONSES_API_KEY";
