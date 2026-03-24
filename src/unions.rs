use serde::{Deserialize, Serialize};
use crate::content::*;
use crate::functions::*;
use crate::messages::*;
use crate::parameters::*;
use crate::reasoning::*;
use crate::response_format::*;
use crate::tools::*;
use crate::enums::ToolChoiceValueEnum;

/// Content part union for input content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum InputContentPart {
    #[serde(rename = "input_text")]
    InputText(InputTextContentParam),
    #[serde(rename = "input_image")]
    InputImage(InputImageContentParamAutoParam),
    #[serde(rename = "input_video")]
    InputVideo(InputVideoContent),
    #[serde(rename = "input_file")]
    InputFile(InputFileContentParam),
}

/// Content part union for output content (assistant messages)
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputContentPart {
    #[serde(rename = "output_text")]
    OutputText(OutputTextContent),
    #[serde(rename = "refusal")]
    Refusal(RefusalContent),
}

/// Content part union for assistant message parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AssistantContentPart {
    #[serde(rename = "output_text")]
    OutputText(OutputTextContentParam),
    #[serde(rename = "refusal")]
    Refusal(RefusalContentParam),
}

/// Content part union for reasoning content
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ReasoningContentPart {
    #[serde(rename = "reasoning_text")]
    ReasoningText(ReasoningTextContent),
    #[serde(rename = "summary_text")]
    SummaryText(SummaryTextContent),
}

/// General content part union for messages
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum ContentPart {
    #[serde(rename = "input_text")]
    InputText(InputTextContent),
    #[serde(rename = "output_text")]
    OutputText(OutputTextContent),
    #[serde(rename = "text")]
    Text(TextContent),
    #[serde(rename = "input_image")]
    InputImage(InputImageContent),
    #[serde(rename = "input_video")]
    InputVideo(InputVideoContent),
    #[serde(rename = "reasoning_text")]
    ReasoningText(ReasoningTextContent),
    #[serde(rename = "summary_text")]
    SummaryText(SummaryTextContent),
    #[serde(rename = "refusal")]
    Refusal(RefusalContent),
}

/// Item union for response output
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum OutputItem {
    #[serde(rename = "message")]
    Message(Message),
    #[serde(rename = "function_call")]
    FunctionCall(FunctionCall),
    #[serde(rename = "function_call_output")]
    FunctionCallOutput(FunctionCallOutput),
    #[serde(rename = "reasoning")]
    Reasoning(Reasoning),
}

/// Item union for input parameters
#[derive(Debug, Clone, Serialize)]
#[serde(untagged)]
pub enum InputItem {
    AssistantMessage(AssistantMessageItemParam),
    DeveloperMessage(DeveloperMessageItemParam),
    SystemMessage(SystemMessageItemParam),
    UserMessage(UserMessageItemParam),
    FunctionCall(FunctionCallItemParam),
    FunctionCallOutput(FunctionCallOutputItemParam),
    Reasoning(ReasoningItemParam),
    ItemReference(ItemReferenceParam),
}

impl<'de> Deserialize<'de> for InputItem {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use serde::de::Error;

        let value = serde_json::Value::deserialize(deserializer)?;

        // Extract type field directly from the Value
        let type_field = value.get("type")
            .and_then(|v| v.as_str())
            .ok_or_else(|| D::Error::missing_field("type"))?;

        match type_field {
            "message" => {
                // For messages, check the role field
                let role = value.get("role")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| D::Error::missing_field("role"))?;

                match role {
                    "user" => {
                        let msg = serde_json::from_value(value)
                            .map_err(D::Error::custom)?;
                        Ok(InputItem::UserMessage(msg))
                    }
                    "assistant" => {
                        let msg = serde_json::from_value(value)
                            .map_err(D::Error::custom)?;
                        Ok(InputItem::AssistantMessage(msg))
                    }
                    "system" => {
                        let msg = serde_json::from_value(value)
                            .map_err(D::Error::custom)?;
                        Ok(InputItem::SystemMessage(msg))
                    }
                    "developer" => {
                        let msg = serde_json::from_value(value)
                            .map_err(D::Error::custom)?;
                        Ok(InputItem::DeveloperMessage(msg))
                    }
                    _ => Err(D::Error::unknown_variant(role, &["user", "assistant", "system", "developer"]))
                }
            }
            "function_call" => {
                let fc = serde_json::from_value(value)
                    .map_err(D::Error::custom)?;
                Ok(InputItem::FunctionCall(fc))
            }
            "function_call_output" => {
                let fco = serde_json::from_value(value)
                    .map_err(D::Error::custom)?;
                Ok(InputItem::FunctionCallOutput(fco))
            }
            "reasoning" => {
                let r = serde_json::from_value(value)
                    .map_err(D::Error::custom)?;
                Ok(InputItem::Reasoning(r))
            }
            "item_reference" => {
                let ir = serde_json::from_value(value)
                    .map_err(D::Error::custom)?;
                Ok(InputItem::ItemReference(ir))
            }
            _ => Err(D::Error::unknown_variant(
                type_field,
                &["message", "function_call", "function_call_output", "reasoning", "item_reference"]
            ))
        }
    }
}

/// Tool choice parameter union
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum ToolChoiceParam {
    /// Simple string mode (none, auto, required)
    Mode(ToolChoiceValueEnum),
    /// Allowed tools configuration
    AllowedTools(AllowedToolsParam),
    /// Specific function to call
    SpecificFunction(SpecificFunctionParam),
}

/// Text format parameter union
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum TextFormatParam {
    /// Text response format
    Text(TextResponseFormat),
    /// JSON schema response format
    JsonSchema(JsonSchemaResponseFormatParam),
}

/// Annotation type for output text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Annotation {
    #[serde(rename = "url")]
    Url(UrlCitationBody),
}

/// Annotation parameter for output text
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnnotationParam {
    #[serde(rename = "url")]
    Url(UrlCitationParam),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input_item_user_message_deserialization() {
        let json = serde_json::json!({
            "type": "message",
            "role": "user",
            "content": [{
                "type": "input_text",
                "text": "Hello"
            }]
        });

        let result: Result<InputItem, _> = serde_json::from_value(json.clone());
        assert!(result.is_ok(), "Failed to deserialize: {:?}", result.err());

        match result.unwrap() {
            InputItem::UserMessage(_) => {},
            other => panic!("Expected UserMessage, got {:?}", other),
        }
    }

    #[test]
    fn test_input_item_array_deserialization() {
        let json = serde_json::json!([
            {
                "type": "message",
                "role": "user",
                "content": [{
                    "type": "input_text",
                    "text": "Hello"
                }]
            }
        ]);

        let result: Result<Vec<InputItem>, _> = serde_json::from_value(json);
        assert!(result.is_ok(), "Failed to deserialize: {:?}", result.err());
        assert_eq!(result.unwrap().len(), 1);
    }

    #[test]
    fn test_user_message_item_param_direct() {
        use crate::messages::UserMessageItemParam;

        let json = serde_json::json!({
            "type": "message",
            "role": "user",
            "content": [{
                "type": "input_text",
                "text": "Hello"
            }]
        });

        let result: Result<UserMessageItemParam, _> = serde_json::from_value(json.clone());
        assert!(result.is_ok(), "Failed to deserialize UserMessageItemParam: {:?}", result.err());
    }

    #[test]
    fn test_input_content_part_deserialization() {
        let json = serde_json::json!({
            "type": "input_text",
            "text": "Hello"
        });

        let result: Result<InputContentPart, _> = serde_json::from_value(json.clone());
        assert!(result.is_ok(), "Failed to deserialize InputContentPart: {:?}", result.err());
    }
}
