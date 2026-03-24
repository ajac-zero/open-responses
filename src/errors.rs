use serde::{Deserialize, Serialize};

/// Error information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Error {
    #[serde(rename = "type")]
    pub type_: String,
    pub code: String,
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub param: Option<String>,
}

/// Error payload
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorPayload {
    pub error: Error,
}

/// Error streaming event
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ErrorStreamingEvent {
    #[serde(rename = "type")]
    pub type_: String, // Always "error"
    pub error: Error,
}
