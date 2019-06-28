use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Environment {
    pub name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub value: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "secureValue")]
    pub secure_value: Option<String>,
}

impl Default for Environment {
    fn default() -> Self {
        Environment {
            name: None,
            value: None,
            secure_value: None,
        }
    }
}
