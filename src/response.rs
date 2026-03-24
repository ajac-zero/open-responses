use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::tokens::{IncompleteDetails, Usage};
use crate::parameters::{StreamOptionsParam, TextParam};
use crate::reasoning::ReasoningParam;
use crate::enums::{IncludeEnum, TruncationEnum, ServiceTierEnum};
use crate::unions::{OutputItem, ToolChoiceParam};
use crate::tools::FunctionToolParam;

/// Response resource
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseResource {
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub object: Option<String>,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status_details: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub output: Option<Vec<OutputItem>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub usage: Option<Usage>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub created_at: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub incomplete_details: Option<IncompleteDetails>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
}

/// Request body for creating a response
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateResponseBody {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub model: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub input: Option<serde_json::Value>, // String or array of ItemParam
    #[serde(skip_serializing_if = "Option::is_none")]
    pub previous_response_id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include: Option<Vec<IncludeEnum>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tools: Option<Vec<FunctionToolParam>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub tool_choice: Option<ToolChoiceParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<HashMap<String, String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<TextParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub temperature: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_p: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub presence_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub frequency_penalty: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parallel_tool_calls: Option<bool>,
    #[serde(default)]
    pub stream: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub stream_options: Option<StreamOptionsParam>,
    #[serde(default)]
    pub background: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_output_tokens: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max_tool_calls: Option<i64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reasoning: Option<ReasoningParam>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub safety_identifier: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub prompt_cache_key: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub truncation: Option<TruncationEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub instructions: Option<String>,
    #[serde(default)]
    pub store: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub service_tier: Option<ServiceTierEnum>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub top_logprobs: Option<i64>,
}
