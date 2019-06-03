use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct CliCommand {
    pub subcommand: Option<String>,
    pub parameters: Option<HashMap<String, String>>,
}

impl CliCommand {
    pub fn subcommand(&self) -> String {
        let cli_command = self.clone();
        if let Some(s) = cli_command.subcommand {
            s
        } else {
            panic!("Azure CLI commands must contain a subcommand.")
        }
    }
    pub fn parameters(&self) -> Vec<String> {
        let cli_command = self.clone();
        let mut parameters = Vec::new();
        if let Some(p) = cli_command.parameters {
            for (k, v) in &p {
                let parameter = format!("--{}", k);
                let argument = v.to_string();
                parameters.push(parameter);
                parameters.push(argument);
            }
        }
        parameters
    }
}

impl Default for CliCommand {
    fn default() -> Self {
        CliCommand {
            subcommand: None,
            parameters: None,
        }
    }
}
