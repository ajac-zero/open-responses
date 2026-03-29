use serde::{Deserialize, Serialize};

use crate::logprobs::LogProb;
use crate::response::ResponseResource;
use crate::unions::{Annotation, ContentPart, OutputItem, ReasoningContentPart};

/// Response created streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCreatedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.created"
    pub sequence_number: i64,
    pub response: ResponseResource,
}

/// Response in progress streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseInProgressStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.in_progress"
    pub sequence_number: i64,
    pub response: ResponseResource,
}

/// Response completed streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseCompletedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.completed"
    pub sequence_number: i64,
    pub response: ResponseResource,
}

/// Response failed streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFailedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.failed"
    pub sequence_number: i64,
    pub response: ResponseResource,
}

/// Response incomplete streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseIncompleteStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.incomplete"
    pub sequence_number: i64,
    pub response: ResponseResource,
}

/// Response queued streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseQueuedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.queued"
    pub sequence_number: i64,
    pub response: ResponseResource,
}

/// Response content part added streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseContentPartAddedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.content_part.added"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub part: ContentPart,
}

/// Response content part done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseContentPartDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.content_part.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub part: ContentPart,
}

/// Response output item added streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutputItemAddedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.output_item.added"
    pub sequence_number: i64,
    pub output_index: i64,
    pub item: Option<OutputItem>,
}

/// Response output item done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutputItemDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.output_item.done"
    pub sequence_number: i64,
    pub output_index: i64,
    pub item: Option<OutputItem>,
}

/// Response output text delta streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutputTextDeltaStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.output_text.delta"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProb>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// Response output text done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutputTextDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.output_text.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProb>>,
}

/// Response output text annotation added streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseOutputTextAnnotationAddedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.output_text.annotation.added"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub annotation_index: i64,
    pub annotation: Option<Annotation>,
}

/// Response function call arguments delta streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDeltaStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.function_call_arguments.delta"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// Response function call arguments done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseFunctionCallArgumentsDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.function_call_arguments.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub arguments: String,
}

/// Response reasoning delta streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReasoningDeltaStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.reasoning.delta"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// Response reasoning done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReasoningDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.reasoning.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub text: String,
}

/// Response reasoning summary delta streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryDeltaStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.reasoning_summary.delta"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub summary_index: i64,
    pub delta: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub obfuscation: Option<String>,
}

/// Response reasoning summary done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.reasoning_summary.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub summary_index: i64,
    pub text: String,
}

/// Response reasoning summary part added streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryPartAddedStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.reasoning_summary_part.added"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub summary_index: i64,
    pub part: ReasoningContentPart,
}

/// Response reasoning summary part done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseReasoningSummaryPartDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.reasoning_summary_part.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub summary_index: i64,
    pub part: ReasoningContentPart,
}

/// Response refusal delta streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRefusalDeltaStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.refusal.delta"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub delta: String,
}

/// Response refusal done streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ResponseRefusalDoneStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "response.refusal.done"
    pub sequence_number: i64,
    pub item_id: String,
    pub output_index: i64,
    pub content_index: i64,
    pub refusal: String,
}
