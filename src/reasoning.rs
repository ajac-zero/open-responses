use serde::{Deserialize, Serialize};
use crate::enums::{ReasoningEffortEnum, ReasoningSummaryEnum};
use crate::unions::ReasoningContentPart;

/// Reasoning information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Reasoning {
    #[serde(rename = "type")]
    pub type_: String, // Always "reasoning"
    pub id: String,
    pub status: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<ReasoningContentPart>>,
}

/// Reasoning body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningBody {
    pub effort: ReasoningEffortEnum,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub summary: Option<ReasoningSummaryEnum>,
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
    pub content: Option<Vec<ReasoningContentPart>>,
}

/// Reasoning summary content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningSummaryContentParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "reasoning_summary"
    pub text: String,
}
