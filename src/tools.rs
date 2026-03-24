use serde::{Deserialize, Serialize};
use crate::enums::ToolChoiceValueEnum;

/// Defines a function tool
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionTool {
    #[serde(rename = "type")]
    pub type_: String, // Always "function"
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Function tool parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionToolParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "function"
    pub name: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub description: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parameters: Option<serde_json::Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub strict: Option<bool>,
}

/// Function tool choice
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FunctionToolChoice {
    #[serde(rename = "type")]
    pub type_: String, // Always "function"
    pub name: String,
}

/// Allowed tool choice configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedToolChoice {
    #[serde(rename = "type")]
    pub type_: String, // Always "allowed_tools"
    pub tools: Vec<FunctionToolChoice>,
    pub mode: ToolChoiceValueEnum,
}

/// Allowed tools parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AllowedToolsParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "allowed_tools"
    pub tools: Vec<String>,
    pub mode: ToolChoiceValueEnum,
}

/// Specific function parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SpecificFunctionParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "function"
    pub name: String,
}
