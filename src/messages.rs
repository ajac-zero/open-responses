use crate::enums::{MessageRole, MessageStatus};
use crate::unions::ContentPart;
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
    // Can be array of content parts or a string
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Developer message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DeveloperMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "developer"
    // Can be array of content parts or a string
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// System message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SystemMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "system"
    // Can be array of content parts or a string
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// User message item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserMessageItemParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "message"
    pub role: String, // Always "user"
    // Can be array of content parts or a string
    pub content: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}
