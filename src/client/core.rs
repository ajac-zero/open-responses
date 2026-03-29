use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use bon::bon;
use secrecy::SecretString;
use url::Url;

use crate::client::error::Result;
use crate::client::mode::{Async as AsyncMode, Mode, Sync as SyncMode};
use crate::client::request::{AsyncResponseRequestBuilder, ResponseRequestBuilder};
use crate::client::{API_KEY_ENV_VAR, BASE_URL_ENV_VAR, DEFAULT_BASE_URL};

pub(crate) struct ClientCore<M: Mode> {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) rt: Option<Arc<tokio::runtime::Runtime>>,
    pub(crate) api_key: Option<SecretString>,
    _mode: PhantomData<M>,
}

impl<M: Mode> ClientCore<M> {
    fn new(headers: HashMap<String, String>, base_url: Url, api_key: Option<SecretString>) -> Self {
        let mode = M::new();

        Self {
            client: reqwest::Client::new(),
            base_url,
            headers,
            rt: mode.runtime(),
            api_key,
            _mode: PhantomData,
        }
    }
}

pub struct Client {
    pub(crate) inner: ClientCore<SyncMode>,
}

pub struct AsyncClient {
    pub(crate) inner: ClientCore<AsyncMode>,
}

fn default_base_url() -> Url {
    Url::parse(DEFAULT_BASE_URL).expect("default base URL is valid")
}

fn env_base_url() -> Result<Url> {
    if let Ok(url) = std::env::var(BASE_URL_ENV_VAR) {
        Url::parse(&url).map_err(Into::into)
    } else {
        Ok(default_base_url())
    }
}

impl Default for Client {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl Client {
    /// Create a new blocking client with the provided API key.
    pub fn new(api_key: impl Into<SecretString>) -> Self {
        Self::builder().api_key(api_key).build()
    }

    /// Initialize a blocking client with an API key from the environment.
    ///
    /// Valid env vars: `OPEN_RESPONSES_API_KEY`, `OPEN_RESPONSES_BASE_URL`
    pub fn from_env() -> Result<Self> {
        Ok(Self::builder()
            .api_key(std::env::var(API_KEY_ENV_VAR)?)
            .base_url(env_base_url()?)
            .build())
    }

    /// Start building a response creation request.
    pub fn create_response(&self) -> ResponseRequestBuilder<'_> {
        ResponseRequestBuilder::new(self)
    }
}

#[bon]
impl Client {
    /// Initialize a blocking client builder to customize the transport configuration.
    #[builder(on(SecretString, into))]
    pub fn builder(
        #[builder(field)] headers: HashMap<String, String>,
        #[builder(default = default_base_url())] base_url: Url,
        api_key: Option<SecretString>,
    ) -> Self {
        Self {
            inner: ClientCore::<SyncMode>::new(headers, base_url, api_key),
        }
    }
}

impl<S: client_builder::State> ClientBuilder<S> {
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}

impl Default for AsyncClient {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl AsyncClient {
    /// Create a new async client with the provided API key.
    pub fn new(api_key: impl Into<SecretString>) -> Self {
        Self::builder().api_key(api_key).build()
    }

    /// Initialize an async client with an API key from the environment.
    ///
    /// Valid env vars: `OPEN_RESPONSES_API_KEY`, `OPEN_RESPONSES_BASE_URL`
    pub fn from_env() -> Result<Self> {
        Ok(Self::builder()
            .api_key(std::env::var(API_KEY_ENV_VAR)?)
            .base_url(env_base_url()?)
            .build())
    }

    /// Start building a response creation request.
    pub fn create_response(&self) -> AsyncResponseRequestBuilder<'_> {
        AsyncResponseRequestBuilder::new(self)
    }
}

#[bon]
impl AsyncClient {
    /// Initialize an async client builder to customize the transport configuration.
    #[builder(on(SecretString, into))]
    pub fn builder(
        #[builder(field)] headers: HashMap<String, String>,
        #[builder(default = default_base_url())] base_url: Url,
        api_key: Option<SecretString>,
    ) -> Self {
        Self {
            inner: ClientCore::<AsyncMode>::new(headers, base_url, api_key),
        }
    }
}

impl<S: async_client_builder::State> AsyncClientBuilder<S> {
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}
