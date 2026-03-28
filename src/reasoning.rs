use crate::enums::{ReasoningEffortEnum, ReasoningSummaryEnum};
use crate::unions::ReasoningContentPart;
use serde::{Deserialize, Serialize};

/// Reasoning information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reasoning {
    pub effort: ReasoningEffortEnum,
    pub summary: ReasoningSummaryEnum,
}

/// Reasoning body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningBody {
    #[serde(rename = "type")]
    pub type_: String, // Always "reasoning"
    pub id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningContentPart>>,
    pub summary: Vec<ReasoningContentPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
}

/// Reasoning parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningParam {
    pub effort: ReasoningEffortEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryEnum>,
}

/// Reasoning item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "reasoning"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    pub summary: Vec<ReasoningContentPart>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningContentPart>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub encrypted_content: Option<String>,
}

/// Reasoning summary content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningSummaryContentParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "reasoning_summary"
    pub text: String,
}
