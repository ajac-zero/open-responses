use serde::{Deserialize, Serialize};

/// Text response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextResponseFormat {
    #[serde(rename = "type")]
    pub type_: String, // Always "text"
}

/// JSON object response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonObjectResponseFormat {
    #[serde(rename = "type")]
    pub type_: String, // Always "json_object"
}

/// JSON schema response format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchemaResponseFormat {
    #[serde(rename = "type")]
    pub type_: String, // Always "json_schema"
    pub name: String,
    pub description: Option<String>, // nullable in spec
    pub schema: serde_json::Value,   // nullable in spec but required
    pub strict: bool,
}

/// JSON schema response format parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JsonSchemaResponseFormatParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "json_schema"
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    pub schema: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Empty model parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmptyModelParam {}
