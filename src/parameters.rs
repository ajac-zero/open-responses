use crate::unions::TextFormatParam;
use serde::{Deserialize, Serialize};

/// Stream options parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct StreamOptionsParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub include_obfuscation: Option<bool>,
}

/// Text parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextParam {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatParam>,
}

/// Text field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextField {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub format: Option<TextFormatParam>,
}

/// Item reference parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemReferenceParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "item_reference"
    pub id: String,
}

/// URL citation body
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlCitationBody {
    #[serde(rename = "type")]
    pub type_: String, // Always "url"
    pub url: String,
    pub start_index: i64,
    pub end_index: i64,
    pub title: String,
}

/// URL citation parameter
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UrlCitationParam {
    #[serde(rename = "type")]
    pub type_: String, // Always "url"
    pub url: String,
    pub start_index: i64,
    pub end_index: i64,
    pub title: String,
}
