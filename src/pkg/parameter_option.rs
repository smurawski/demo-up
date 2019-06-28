use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParameterOption {
    #[serde(rename = "type")]
    pub parameter_type: String,
    #[serde(rename = "defaultValue", skip_serializing_if = "Option::is_none")]
    pub default_value: Option<String>,
    #[serde(rename = "allowedValues", skip_serializing_if = "Option::is_none")]
    pub allowed_values: Option<Vec<String>>,
    #[serde(rename = "minValue", skip_serializing_if = "Option::is_none")]
    pub min_value: Option<i32>,
    #[serde(rename = "maxValue", skip_serializing_if = "Option::is_none")]
    pub max_value: Option<i32>,
    #[serde(rename = "minLength", skip_serializing_if = "Option::is_none")]
    pub min_length: Option<i32>,
    #[serde(rename = "maxLength", skip_serializing_if = "Option::is_none")]
    pub max_length: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub metadata: Option<ParameterMetadata>,
}

impl Default for ParameterOption {
    fn default() -> Self {
        ParameterOption {
            parameter_type: "String".to_string(),
            default_value: None,
            allowed_values: None,
            min_value: None,
            max_value: None,
            min_length: None,
            max_length: None,
            metadata: None,
        }
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ParameterMetadata {
    description: String,
}
