use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ArmTemplate {
    pub path: Option<String>,
    pub url: Option<String>,
    pub parameters: Option<HashMap<String, String>>,
}

impl ArmTemplate {
    pub fn path(&self) -> String {
        let template = self.clone();
        if let Some(p) = template.path {
            p
        } else {
            panic!("Failed to retrieve a local path.")
        }
    }

    pub fn url(&self) -> String {
        let template = self.clone();
        if let Some(u) = template.url {
            u
        } else {
            panic!("Failed to retrieve a template url.")
        }
    }

    pub fn parameters(&self) -> Vec<String> {
        let local_template = self.clone();
        let mut parameters = Vec::new();
        if let Some(p) = local_template.parameters {
            for (k, v) in &p {
                let parameter_string = format!("{}={}", k, v);
                parameters.push(parameter_string);
            }
        }
        parameters
    }
}

impl Default for ArmTemplate {
    fn default() -> Self {
        ArmTemplate {
            path: None,
            url: None,
            parameters: None,
        }
    }
}
