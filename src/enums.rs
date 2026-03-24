use serde::{Deserialize, Serialize};

/// Image detail level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ImageDetail {
    /// Restricts the model to a lower-resolution version of the image
    Low,
    /// Allows the model to "see" a higher-resolution version of the image, usually increasing input token costs
    High,
    /// Choose the detail level automatically
    Auto,
}

/// Include options for response data
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IncludeEnum {
    /// Includes encrypted reasoning content so that it may be rehydrated on a subsequent request
    #[serde(rename = "reasoning.encrypted_content")]
    ReasoningEncryptedContent,
    /// Includes sampled logprobs in assistant messages
    #[serde(rename = "message.output_text.logprobs")]
    MessageOutputTextLogprobs,
}

/// Truncation mode for context handling
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum TruncationEnum {
    /// Let the service decide how to truncate
    Auto,
    /// Disable service truncation. Context over the model's context limit will result in a 400 error
    Disabled,
}

/// Service tier selection
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ServiceTierEnum {
    /// Choose a service tier automatically based on current account state
    Auto,
    /// Choose the default service tier
    Default,
    /// Choose the flex service tier
    Flex,
    /// Choose the priority service tier
    Priority,
}

/// Reasoning effort level
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningEffortEnum {
    /// Restrict the model from performing any reasoning before emitting a final answer
    None,
    /// Use a lower reasoning effort for faster responses
    Low,
    /// Use a balanced reasoning effort
    Medium,
    /// Use a higher reasoning effort to improve answer quality
    High,
    /// Use the maximum reasoning effort available
    Xhigh,
}

/// Reasoning summary mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ReasoningSummaryEnum {
    /// Emit concise summaries of reasoning content
    Concise,
    /// Emit detailed summaries of reasoning content
    Detailed,
    /// Allow the model to decide when to summarize
    Auto,
}

/// Tool choice mode
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum ToolChoiceValueEnum {
    /// Restrict the model from calling any tools
    None,
    /// Let the model choose the tools from among the provided set
    Auto,
    /// Require the model to call a tool
    Required,
}

/// Message status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MessageStatus {
    /// Model is currently sampling this item
    InProgress,
    /// Model has finished sampling this item
    Completed,
    /// Model was interrupted from sampling this item partway through
    Incomplete,
}

/// Message role
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MessageRole {
    /// End-user input in the conversation
    User,
    /// Model-generated content in the conversation
    Assistant,
    /// System-level instructions that set global behavior
    System,
    /// Developer-supplied guidance that shapes the assistant's behavior
    Developer,
}

/// Function call status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum FunctionCallStatus {
    /// Model is currently sampling this item
    InProgress,
    /// Model has finished sampling this item
    Completed,
    /// Model was interrupted from sampling this item partway through
    Incomplete,
}
