use crate::enums::VerbosityEnum;
use crate::unions::TextFormatParam;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// Metadata parameter - key-value pairs attached to objects
/// Keys max length: 64 characters, Values max length: 512 characters, max 16 pairs
pub type MetadataParam = HashMap<String, String>;

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
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<VerbosityEnum>,
}

/// Text field
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TextField {
    pub format: TextFormatParam,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub verbosity: Option<VerbosityEnum>,
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
