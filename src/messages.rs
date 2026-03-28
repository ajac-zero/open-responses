use crate::enums::{MessageRole, MessageStatus};
use crate::unions::{AssistantContentPart, ContentPart, InputContentPart};
use serde::{Deserialize, Serialize};

/// A message to or from the model
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub id: String,
    pub status: MessageStatus,
    pub role: MessageRole,
    pub content: Vec<ContentPart>,
}

/// Assistant message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssistantMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "assistant"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<AssistantContentPart>>,
}

/// Developer message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "developer"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InputContentPart>>,
}

/// System message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "system"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InputContentPart>>,
}

/// User message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "user"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub content: Option<Vec<InputContentPart>>,
}
