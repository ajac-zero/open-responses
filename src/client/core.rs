use std::collections::HashMap;
use std::marker::PhantomData;
use std::sync::Arc;

use bon::bon;
use secrecy::SecretString;
use url::Url;

use crate::client::{
    Async, Mode, ResponseRequestBuilder, Result, Sync, API_KEY_ENV_VAR, BASE_URL_ENV_VAR,
    DEFAULT_BASE_URL,
};

/// Core client for the Open Responses API.
///
/// To initialize a proper client, use either `Client` or `AsyncClient`.
pub struct ClientCore<M: Mode> {
    pub(crate) client: reqwest::Client,
    pub(crate) base_url: Url,
    pub(crate) headers: HashMap<String, String>,
    pub(crate) rt: Option<Arc<tokio::runtime::Runtime>>,
    pub(crate) api_key: Option<SecretString>,
    _mode: PhantomData<M>,
}

/// Alias for the `ClientCore` in blocking mode.
pub type Client = ClientCore<Sync>;

/// Alias for the `ClientCore` in async mode.
pub type AsyncClient = ClientCore<Async>;

impl<M: Mode> Default for ClientCore<M> {
    fn default() -> Self {
        Self::builder().build()
    }
}

impl<M: Mode> ClientCore<M> {
    /// Create a new client with the provided API key.
    pub fn new(api_key: impl Into<SecretString>) -> Self {
        Self::builder().api_key(api_key).build()
    }

    /// Initialize a client with an API key from the environment.
    ///
    /// Valid env vars: `OPEN_RESPONSES_API_KEY`, `OPEN_RESPONSES_BASE_URL`
    pub fn from_env() -> Result<Self> {
        let api_key = std::env::var(API_KEY_ENV_VAR)?;

        let base_url = if let Ok(url) = std::env::var(BASE_URL_ENV_VAR) {
            Url::parse(&url)?
        } else {
            Url::parse(DEFAULT_BASE_URL).expect("default base URL is valid")
        };

        Ok(Self::builder().api_key(api_key).base_url(base_url).build())
    }

    /// Start building a response creation request.
    pub fn create_response(&self) -> ResponseRequestBuilder<'_, M> {
        ResponseRequestBuilder::new(self)
    }
}

#[bon]
impl<M: Mode> ClientCore<M> {
    /// Initialize a client builder to customize the transport configuration.
    #[builder(on(SecretString, into))]
    pub fn builder(
        #[builder(field)] headers: HashMap<String, String>,
        #[builder(default = Url::parse(DEFAULT_BASE_URL).expect("default base URL is valid"))]
        base_url: Url,
        api_key: Option<SecretString>,
    ) -> Self {
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

impl<M: Mode, S: client_core_builder::State> ClientCoreBuilder<M, S> {
    pub fn add_header(mut self, key: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(key.into(), value.into());
        self
    }
}
