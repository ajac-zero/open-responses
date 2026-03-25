use serde::{Deserialize, Serialize};
use crate::enums::ImageDetail;
use crate::unions::{Annotation, AnnotationParam};

/// Input text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTextContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "input_text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Input text content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputTextContentParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "input_text"
    pub text: String,
}

/// Output text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputTextContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "output_text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<Annotation>>,
}

/// Output text content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputTextContentParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "output_text"
    pub text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub annotations: Option<Vec<AnnotationParam>>,
}

/// Text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Input image content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputImageContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "input_image"
    pub image_url: String,
    pub detail: ImageDetail,
}

/// Input image content parameter with auto detail
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputImageContentParamAutoParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "input_image"
    pub image_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<ImageDetail>,
}

/// Input video content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputVideoContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "input_video"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub video_url: Option<String>,
}

/// Reasoning text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReasoningTextContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "reasoning_text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}

/// Refusal content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefusalContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "refusal"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub refusal: Option<String>,
}

/// Refusal content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RefusalContentParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "refusal"
    pub refusal: String,
}

/// Summary text content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SummaryTextContent {
    #[serde(rename = "type")]
    pub type_: String, // Always "summary_text"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub text: Option<String>,
}
