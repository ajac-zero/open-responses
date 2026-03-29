use std::collections::HashMap;
use std::marker::PhantomData;

use secrecy::ExposeSecret;
use tracing::debug;

use crate::client::core::{AsyncClient, Client, ClientCore};
use crate::client::error::{Error, Result};
use crate::client::mode::{Async as AsyncMode, Mode, Sync as SyncMode};
use crate::client::stream::{AsyncResponseStream, ResponseStream};
use crate::client::RESPONSES_PATH;
use crate::{
    CreateResponseBody, FunctionToolParam, IncludeEnum, InputItem, ReasoningParam,
    ResponseResource, ServiceTierEnum, StreamOptionsParam, TextParam, ToolChoiceParam,
    TruncationEnum,
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

pub(crate) struct ResponseRequestBuilderCore<'a, M: Mode> {
    core: &'a ClientCore<M>,
    body: CreateResponseBody,
    api_key: Option<&'a str>,
    _mode: PhantomData<M>,
}

impl<'a, M: Mode> ResponseRequestBuilderCore<'a, M> {
    fn new(core: &'a ClientCore<M>) -> Self {
        Self {
            core,
            body: empty_request_body(),
            api_key: core.api_key.as_ref().map(ExposeSecret::expose_secret),
            _mode: PhantomData,
        }
    }

    fn body(mut self, body: CreateResponseBody) -> Self {
        self.body = body;
        self
    }

    fn serialized_input(mut self, input: impl serde::Serialize) -> Self {
        self.body.input =
            Some(serde_json::to_value(input).expect("response request input should serialize"));
        self
    }

    fn input(mut self, input: serde_json::Value) -> Self {
        self.body.input = Some(input);
        self
    }

    fn input_item(self, input: InputItem) -> Self {
        self.input_items([input])
    }

    fn input_items(self, input: impl IntoIterator<Item = InputItem>) -> Self {
        self.serialized_input(input.into_iter().collect::<Vec<_>>())
    }

    fn input_text(self, text: impl Into<String>) -> Self {
        self.serialized_input(text.into())
    }

    fn model(mut self, model: impl Into<String>) -> Self {
        self.body.model = Some(model.into());
        self
    }

    fn instructions(mut self, instructions: impl Into<String>) -> Self {
        self.body.instructions = Some(instructions.into());
        self
    }

    fn previous_response_id(mut self, id: impl Into<String>) -> Self {
        self.body.previous_response_id = Some(id.into());
        self
    }

    fn include(mut self, include: impl IntoIterator<Item = IncludeEnum>) -> Self {
        self.body.include = Some(include.into_iter().collect());
        self
    }

    fn tools(mut self, tools: impl IntoIterator<Item = FunctionToolParam>) -> Self {
        self.body.tools = Some(tools.into_iter().collect());
        self
    }

    fn tool_choice(mut self, choice: ToolChoiceParam) -> Self {
        self.body.tool_choice = Some(choice);
        self
    }

    fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
        self.body.metadata = Some(metadata);
        self
    }

    fn text(mut self, text: TextParam) -> Self {
        self.body.text = Some(text);
        self
    }

    fn temperature(mut self, temperature: f64) -> Self {
        self.body.temperature = Some(temperature);
        self
    }

    fn top_p(mut self, top_p: f64) -> Self {
        self.body.top_p = Some(top_p);
        self
    }

    fn presence_penalty(mut self, presence_penalty: f64) -> Self {
        self.body.presence_penalty = Some(presence_penalty);
        self
    }

    fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
        self.body.frequency_penalty = Some(frequency_penalty);
        self
    }

    fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
        self.body.parallel_tool_calls = Some(parallel_tool_calls);
        self
    }

    fn stream_options(mut self, stream_options: StreamOptionsParam) -> Self {
        self.body.stream_options = Some(stream_options);
        self
    }

    fn background(mut self, background: bool) -> Self {
        self.body.background = background;
        self
    }

    fn max_output_tokens(mut self, max_output_tokens: i64) -> Self {
        self.body.max_output_tokens = Some(max_output_tokens);
        self
    }

    fn max_tool_calls(mut self, max_tool_calls: i64) -> Self {
        self.body.max_tool_calls = Some(max_tool_calls);
        self
    }

    fn reasoning(mut self, reasoning: ReasoningParam) -> Self {
        self.body.reasoning = Some(reasoning);
        self
    }

    fn safety_identifier(mut self, safety_identifier: impl Into<String>) -> Self {
        self.body.safety_identifier = Some(safety_identifier.into());
        self
    }

    fn prompt_cache_key(mut self, prompt_cache_key: impl Into<String>) -> Self {
        self.body.prompt_cache_key = Some(prompt_cache_key.into());
        self
    }

    fn truncation(mut self, truncation: TruncationEnum) -> Self {
        self.body.truncation = Some(truncation);
        self
    }

    fn store(mut self, store: bool) -> Self {
        self.body.store = store;
        self
    }

    fn service_tier(mut self, service_tier: ServiceTierEnum) -> Self {
        self.body.service_tier = Some(service_tier);
        self
    }

    fn top_logprobs(mut self, top_logprobs: i64) -> Self {
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

pub struct ResponseRequestBuilder<'a> {
    inner: ResponseRequestBuilderCore<'a, SyncMode>,
}

pub struct AsyncResponseRequestBuilder<'a> {
    inner: ResponseRequestBuilderCore<'a, AsyncMode>,
}

macro_rules! impl_request_builder_methods {
    ($builder:ident) => {
        impl<'a> $builder<'a> {
            pub fn body(mut self, body: CreateResponseBody) -> Self {
                self.inner = self.inner.body(body);
                self
            }

            pub fn input(mut self, input: serde_json::Value) -> Self {
                self.inner = self.inner.input(input);
                self
            }

            pub fn input_item(mut self, input: InputItem) -> Self {
                self.inner = self.inner.input_item(input);
                self
            }

            pub fn input_items(mut self, input: impl IntoIterator<Item = InputItem>) -> Self {
                self.inner = self.inner.input_items(input);
                self
            }

            pub fn input_text(mut self, text: impl Into<String>) -> Self {
                self.inner = self.inner.input_text(text);
                self
            }

            pub fn model(mut self, model: impl Into<String>) -> Self {
                self.inner = self.inner.model(model);
                self
            }

            pub fn instructions(mut self, instructions: impl Into<String>) -> Self {
                self.inner = self.inner.instructions(instructions);
                self
            }

            pub fn previous_response_id(mut self, id: impl Into<String>) -> Self {
                self.inner = self.inner.previous_response_id(id);
                self
            }

            pub fn include(mut self, include: impl IntoIterator<Item = IncludeEnum>) -> Self {
                self.inner = self.inner.include(include);
                self
            }

            pub fn tools(mut self, tools: impl IntoIterator<Item = FunctionToolParam>) -> Self {
                self.inner = self.inner.tools(tools);
                self
            }

            pub fn tool_choice(mut self, choice: ToolChoiceParam) -> Self {
                self.inner = self.inner.tool_choice(choice);
                self
            }

            pub fn metadata(mut self, metadata: HashMap<String, String>) -> Self {
                self.inner = self.inner.metadata(metadata);
                self
            }

            pub fn text(mut self, text: TextParam) -> Self {
                self.inner = self.inner.text(text);
                self
            }

            pub fn temperature(mut self, temperature: f64) -> Self {
                self.inner = self.inner.temperature(temperature);
                self
            }

            pub fn top_p(mut self, top_p: f64) -> Self {
                self.inner = self.inner.top_p(top_p);
                self
            }

            pub fn presence_penalty(mut self, presence_penalty: f64) -> Self {
                self.inner = self.inner.presence_penalty(presence_penalty);
                self
            }

            pub fn frequency_penalty(mut self, frequency_penalty: f64) -> Self {
                self.inner = self.inner.frequency_penalty(frequency_penalty);
                self
            }

            pub fn parallel_tool_calls(mut self, parallel_tool_calls: bool) -> Self {
                self.inner = self.inner.parallel_tool_calls(parallel_tool_calls);
                self
            }

            pub fn stream_options(mut self, stream_options: StreamOptionsParam) -> Self {
                self.inner = self.inner.stream_options(stream_options);
                self
            }

            pub fn background(mut self, background: bool) -> Self {
                self.inner = self.inner.background(background);
                self
            }

            pub fn max_output_tokens(mut self, max_output_tokens: i64) -> Self {
                self.inner = self.inner.max_output_tokens(max_output_tokens);
                self
            }

            pub fn max_tool_calls(mut self, max_tool_calls: i64) -> Self {
                self.inner = self.inner.max_tool_calls(max_tool_calls);
                self
            }

            pub fn reasoning(mut self, reasoning: ReasoningParam) -> Self {
                self.inner = self.inner.reasoning(reasoning);
                self
            }

            pub fn safety_identifier(mut self, safety_identifier: impl Into<String>) -> Self {
                self.inner = self.inner.safety_identifier(safety_identifier);
                self
            }

            pub fn prompt_cache_key(mut self, prompt_cache_key: impl Into<String>) -> Self {
                self.inner = self.inner.prompt_cache_key(prompt_cache_key);
                self
            }

            pub fn truncation(mut self, truncation: TruncationEnum) -> Self {
                self.inner = self.inner.truncation(truncation);
                self
            }

            pub fn store(mut self, store: bool) -> Self {
                self.inner = self.inner.store(store);
                self
            }

            pub fn service_tier(mut self, service_tier: ServiceTierEnum) -> Self {
                self.inner = self.inner.service_tier(service_tier);
                self
            }

            pub fn top_logprobs(mut self, top_logprobs: i64) -> Self {
                self.inner = self.inner.top_logprobs(top_logprobs);
                self
            }
        }
    };
}

impl_request_builder_methods!(ResponseRequestBuilder);
impl_request_builder_methods!(AsyncResponseRequestBuilder);

impl<'a> ResponseRequestBuilder<'a> {
    pub(crate) fn new(client: &'a Client) -> Self {
        Self {
            inner: ResponseRequestBuilderCore::new(&client.inner),
        }
    }

    /// Send the request and return a complete response.
    pub fn send(self) -> Result<ResponseResource> {
        self.inner.send()
    }

    /// Send the request and return a streaming response.
    pub fn stream(self) -> Result<ResponseStream> {
        self.inner.stream()
    }
}

impl<'a> AsyncResponseRequestBuilder<'a> {
    pub(crate) fn new(client: &'a AsyncClient) -> Self {
        Self {
            inner: ResponseRequestBuilderCore::new(&client.inner),
        }
    }

    /// Asynchronously send the request and return a complete response.
    pub async fn send(self) -> Result<ResponseResource> {
        self.inner.send().await
    }

    /// Asynchronously send the request and return a streaming response.
    pub async fn stream(self) -> Result<AsyncResponseStream> {
        self.inner.stream().await
    }
}

impl<'a> ResponseRequestBuilderCore<'a, SyncMode> {
    fn send(mut self) -> Result<ResponseResource> {
        self.body.stream = false;
        debug!(request_body = ?self.body);

        let rt = self.core.rt.as_ref().expect("sync mode has runtime");
        let response = rt.block_on(self.send_request())?;
        let resource: ResponseResource = rt.block_on(response.json())?;
        debug!(response = ?resource);

        Ok(resource)
    }

    fn stream(mut self) -> Result<ResponseStream> {
        self.body.stream = true;

        let rt = self.core.rt.clone().expect("sync mode has runtime");
        let response = rt.block_on(self.send_request())?;

        Ok(ResponseStream::spawn(response, rt))
    }
}

impl<'a> ResponseRequestBuilderCore<'a, AsyncMode> {
    async fn send(mut self) -> Result<ResponseResource> {
        self.body.stream = false;
        debug!(request_body = ?self.body);

        let response = self.send_request().await?;
        let resource: ResponseResource = response.json().await?;
        debug!(response = ?resource);

        Ok(resource)
    }

    async fn stream(mut self) -> Result<AsyncResponseStream> {
        self.body.stream = true;

        let response = self.send_request().await?;

        Ok(AsyncResponseStream::spawn(response))
    }
}

impl<M: Mode> std::fmt::Debug for ResponseRequestBuilderCore<'_, M> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("ResponseRequestBuilder")
            .field("body", &self.body)
            .finish()
    }
}

impl std::fmt::Debug for ResponseRequestBuilder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

impl std::fmt::Debug for AsyncResponseRequestBuilder<'_> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.inner.fmt(f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::client::Client;
    use crate::{InputContentPart, InputTextContentParam, UserMessageItemParam};

    fn user_message(text: &str) -> InputItem {
        InputItem::UserMessage(UserMessageItemParam {
            type_: "message".into(),
            role: "user".into(),
            content: serde_json::to_value(vec![InputContentPart::InputText(
                InputTextContentParam {
                    type_: "input_text".into(),
                    text: text.into(),
                },
            )])
            .expect("input content should serialize"),
            id: None,
            status: None,
        })
    }

    #[test]
    fn input_item_serializes_as_item_array() {
        let client = Client::default();
        let builder = client.create_response().input_item(user_message("hello"));

        assert_eq!(
            builder.inner.body.input,
            Some(serde_json::json!([
                {
                    "type": "message",
                    "role": "user",
                    "content": [{
                        "type": "input_text",
                        "text": "hello"
                    }]
                }
            ]))
        );
    }

    #[test]
    fn input_text_serializes_as_plain_string() {
        let client = Client::default();
        let builder = client.create_response().input_text("hello");

        assert_eq!(builder.inner.body.input, Some(serde_json::json!("hello")));
    }

    #[test]
    fn input_items_serializes_as_item_array() {
        let client = Client::default();
        let builder = client
            .create_response()
            .input_items([user_message("hello"), user_message("world")]);

        assert_eq!(
            builder.inner.body.input,
            Some(serde_json::json!([
                {
                    "type": "message",
                    "role": "user",
                    "content": [{
                        "type": "input_text",
                        "text": "hello"
                    }]
                },
                {
                    "type": "message",
                    "role": "user",
                    "content": [{
                        "type": "input_text",
                        "text": "world"
                    }]
                }
            ]))
        );
    }
}
