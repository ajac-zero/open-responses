use crate::enums::{FunctionCallOutputStatusEnum, FunctionCallStatus};
use serde::{Deserialize, Serialize};

/// Function call
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCall {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "function_call"
    pub id: String,
    pub call_id: String,
    pub status: FunctionCallStatus,
    pub name: String,
    pub arguments: String,
}

/// Function call output
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallOutput {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "function_call_output"
    pub id: String,
    pub status: FunctionCallOutputStatusEnum,
    pub call_id: String,
    pub output: String,
}

/// Function call item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallItemParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "function_call"
    pub name: String,
    pub arguments: String,
    pub call_id: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Function call output item parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionCallOutputItemParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "function_call_output"
    pub call_id: String,
    // Can be a string or array of content parts - using serde_json::Value for flexibility
    pub output: serde_json::Value,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub id: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<String>,
}

/// Input file content
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFileContent {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_file"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}

/// Input file content parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct InputFileContentParam {
    #[serde(rename = "type", skip_deserializing)]
    pub type_: String, // Always "input_file"
    #[serde(skip_serializing_if = "Option::is_none")]
    pub filename: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_data: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub file_url: Option<String>,
}
