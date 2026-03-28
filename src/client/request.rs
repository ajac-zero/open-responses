use std::collections::HashMap;
use std::marker::PhantomData;

use secrecy::ExposeSecret;
use tracing::debug;

use crate::client::{ClientCore, Error, Mode, ResponseStream, Result, RESPONSES_PATH};
use crate::{
    CreateResponseBody, FunctionToolParam, IncludeEnum, ReasoningParam, ResponseResource,
    ServiceTierEnum, StreamOptionsParam, TextParam, ToolChoiceParam, TruncationEnum,
};

fn empty_request_body() -> CreateResponseBody {
    CreateResponseBody {
        model: None,
        input: None,
        previous_response_id: None,
        include: None,
        tools: None,
        tool_choice: None,
        metadata: None,
        text: None,
        temperature: None,
        top_p: None,
        presence_penalty: None,
        frequency_penalty: None,
        parallel_tool_calls: None,
        stream: false,
        stream_options: None,
        background: false,
        max_output_tokens: None,
        max_tool_calls: None,
        reasoning: None,
        safety_identifier: None,
        prompt_cache_key: None,
        truncation: None,
        instructions: None,
        store: false,
        service_tier: None,
        top_logprobs: None,
    }
}

/// Public builder for a response request.
pub struct ResponseRequestBuilder<'a, M: Mode> {
    core: &'a ClientCore<M>,
    body: CreateResponseBody,
    api_key: Option<&'a str>,
    _mode: PhantomData<M>,
}

impl<'a, M: Mode> ResponseRequestBuilder<'a, M> {
    pub(crate) fn new(core: &'a ClientCore<M>) -> Self {
        Self {
            core,
            body: empty_request_body(),
            api_key: core.api_key.as_ref().map(ExposeSecret::expose_secret),
            _mode: PhantomData,
        }
    }

    pub fn body(mut self, body: CreateResponseBody) -> Self {
        self.body = body;
        self
    }

    pub fn input(mut self, input: serde_json::Value) -> Self {
        self.body.input = Some(input);
        self
    }

    pub fn model(mut self, model: impl Into<String>) -> Self {
        self.body.model = Some(model.into());
        self
    }

    pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.body.instructions = Some(instructions.into());
        self
    }

    pub fn previous_response_id(mut self, id: impl Into<String>) -> Self {
        self.body.previous_response_id = Some(id.into());
        self
    }

    pub fn include(mut self, include: impl IntoIterator<Item = IncludeEnum>) -> Self {
        self.body.include = Some(include.into_iter().collect());
        self
    }

    pub fn tools(mut self, tools: impl IntoIterator<Item = FunctionToolParam>) -> Self {
        self.body.tools = Some(tools.into_iter().collect());
        self
    }

    pub fn tool_choice(mut self, choice: ToolChoiceParam) -> Self {
        self.body.tool_choice = Some(choice);
        self
    }

    pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.body.metadata = Some(metadata);
        self
    }

    pub fn text(mut self, text: TextParam) -> Self {
        self.body.text = Some(text);
        self
    }

    pub fn temperature(mut self, temperature: f64) -> Self {
        self.body.temperature = Some(temperature);
        self
    }

    pub fn top_p(mut self, top_p: f64) -> Self {
        self.body.top_p = Some(top_p);
        self
    }

    pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.body.presence_penalty = Some(presence_penalty);
        self
    }

    pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.body.frequency_penalty = Some(frequency_penalty);
        self
    }

    pub fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.body.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }

    pub fn stream_options(mut self, stream_options: StreamOptionsParam) -> Self {
        self.body.stream_options = Some(stream_options);
        self
    }

    pub fn background(mut self, background: bool) -> Self {
        self.body.background = background;
        self
    }

    pub fn max_output_tokens(mut self, max_output_tokens: i64) -> Self {
        self.body.max_output_tokens = Some(max_output_tokens);
        self
    }

    pub fn max_tool_calls(mut self, max_tool_calls: i64) -> Self {
        self.body.max_tool_calls = Some(max_tool_calls);
        self
    }

    pub fn reasoning(mut self, reasoning: ReasoningParam) -> Self {
        self.body.reasoning = Some(reasoning);
        self
    }

    pub fn safety_identifier(mut self, safety_identifier: impl Into<String>) -> Self {
        self.body.safety_identifier = Some(safety_identifier.into());
        self
    }

    pub fn prompt_cache_key(mut self, prompt_cache_key: impl Into<String>) -> Self {
        self.body.prompt_cache_key = Some(prompt_cache_key.into());
        self
    }

    pub fn truncation(mut self, truncation: TruncationEnum) -> Self {
        self.body.truncation = Some(truncation);
        self
    }

    pub fn store(mut self, store: bool) -> Self {
        self.body.store = store;
        self
    }

    pub fn service_tier(mut self, service_tier: ServiceTierEnum) -> Self {
        self.body.service_tier = Some(service_tier);
        self
    }

    pub fn top_logprobs(mut self, top_logprobs: i64) -> Self {
        self.body.top_logprobs = Some(top_logprobs);
        self
    }

    async fn send_request(&self) -> Result<reqwest::Response> {
        let mut url = self.core.base_url.clone();
        url.path_segments_mut()
            .expect("valid base URL")
            .extend(RESPONSES_PATH);

        let mut builder = self.core.client.post(url);

        for (key, value) in &self.core.headers {
            builder = builder.header(key.as_str(), value.as_str());
        }

        if let Some(token) = self.api_key {
            builder = builder.header("authorization", format!("Bearer {token}"));
        }

        let response = builder.json(&self.body).send().await?;

        if !response.status().is_success() {
            let status = response.status().as_u16();
            let body = response.text().await.unwrap_or_default();
            return Err(Error::parse_api_error(status, &body));
        }

        Ok(response)
    }
}

impl<'a> ResponseRequestBuilder<'a, crate::client::Sync> {
    /// Send the request and return a complete response.
    pub fn send(mut self) -> Result<ResponseResource> {
        self.body.stream = false;
        debug!(request_body = ?self.body);

        let rt = self.core.rt.as_ref().expect("sync mode has runtime");
        let response = rt.block_on(self.send_request())?;
        let resource: ResponseResource = rt.block_on(response.json())?;
        debug!(response = ?resource);

        Ok(resource)
    }

    /// Send the request and return a streaming response.
    pub fn stream(mut self) -> Result<ResponseStream> {
        self.body.stream = true;

        let rt = self.core.rt.clone().expect("sync mode has runtime");
        let response = rt.block_on(self.send_request())?;

        Ok(ResponseStream::spawn(response, Some(rt)))
    }
}

impl<'a> ResponseRequestBuilder<'a, crate::client::Async> {
    /// Asynchronously send the request and return a complete response.
    pub async fn send(mut self) -> Result<ResponseResource> {
        self.body.stream = false;
        debug!(request_body = ?self.body);

        let response = self.send_request().await?;
        let resource: ResponseResource = response.json().await?;
        debug!(response = ?resource);

        Ok(resource)
    }

    /// Asynchronously send the request and return a streaming response.
    pub async fn stream(mut self) -> Result<ResponseStream> {
        self.body.stream = true;

        let response = self.send_request().await?;

        Ok(ResponseStream::spawn(response, None))
    }
}

impl<M: Mode> std::fmt::Debug for ResponseRequestBuilder<'_, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseRequestBuilder")
            .field("body", &self.body)
            .finish()
    }
}
