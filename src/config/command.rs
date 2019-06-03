use super::ArmTemplate;
use super::CliCommand;
use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Command {
    pub order: Option<u32>,
    pub resource_group: Option<String>,
    pub location: Option<String>,
    pub cli: Option<CliCommand>,
    pub template: Option<ArmTemplate>,
}

impl Command {
    pub fn cli(&self) -> CliCommand {
        let command = self.clone();
        if let Some(c) = command.cli {
            c
        } else {
            CliCommand::default()
        }
    }

    pub fn template(&self) -> ArmTemplate {
        let command = self.clone();
        if let Some(t) = command.template {
            t
        } else {
            ArmTemplate::default()
        }
    }

    pub fn order(&self) -> u32 {
        if let Some(o) = self.order {
            o
        } else {
            u32::max_value()
        }
    }
}

impl Default for Command {
    fn default() -> Self {
        Command {
            order: None,
            resource_group: None,
            location: None,
            cli: None,
            template: None,
        }
    }
}
