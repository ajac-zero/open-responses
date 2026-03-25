use serde::{Deserialize, Serialize};

/// Error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    pub code: String,
    pub message: String,
}

/// Error payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    #[serde(rename = "type")]
    pub type_: String,
    pub code: String,
    pub message: String,
    pub param: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub headers: Option<serde_json::Value>,
}

/// Error streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "error"
    pub sequence_number: i64,
    pub error: Error,
}
