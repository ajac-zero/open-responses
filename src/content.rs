use crate::enums::ImageDetail;
use crate::logprobs::LogProb;
use crate::unions::{Annotation, AnnotationParam};
use serde::{Deserialize, Serialize};

/// Input text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTextContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_text"
    pub text: String,
}

/// Input text content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTextContentParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_text"
    pub text: String,
}

/// Output text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputTextContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "output_text"
    pub text: String,
    pub annotations: Vec<Annotation>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub logprobs: Option<Vec<LogProb>>,
}

/// Output text content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputTextContentParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "output_text"
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AnnotationParam>>,
}

/// Text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "text"
    pub text: String,
}

/// Input image content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputImageContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_image"
    pub image_url: Option<String>,
    pub detail: ImageDetail,
}

/// Input image content parameter with auto detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputImageContentParamAutoParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_image"
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Input video content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputVideoContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_video"
    pub video_url: String,
}

/// Reasoning text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTextContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "reasoning_text"
    pub text: String,
}

/// Refusal content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefusalContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "refusal"
    pub refusal: String,
}

/// Refusal content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefusalContentParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "refusal"
    pub refusal: String,
}

/// Summary text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryTextContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "summary_text"
    pub text: String,
}
