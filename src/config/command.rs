use super::ArmTemplate;
use super::CliCommand;
use cli::CliArgs;
use handlebars::{to_json, Handlebars};
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

    pub fn resource_group(&self, session_name: &str, cli_args: &CliArgs) -> Option<String> {
        let command = self.clone();
        if let Some(resource_group) = command.resource_group {
            let mut reg = Handlebars::new();
            match reg.render_template(&resource_group, &to_json(cli_args)) {
                Ok(r) => Some(r),
                Err(e) => panic!("Failed to evaluate the resource group template. {}", e),
            }
        } else {
            Some(format!("{}-{}", session_name, cli_args.event))
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

    pub fn update<'a>(&'a mut self, session_name: &str, cli_args: &CliArgs) -> &'a mut Command {
        self
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
