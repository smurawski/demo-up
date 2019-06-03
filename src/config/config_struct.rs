use super::Command;
use super::Session;
use cli::CliArgs;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub subscription: Option<String>,
    pub sessions: Option<Vec<Session>>,
}

impl Config {
    pub fn write(&self, file_path: &str) {
        if let Ok(content) = serde_yaml::to_string(self) {
            let mut file = File::create(file_path).expect("Failed to create the output file.");
            file.write_all(content.into_bytes().as_ref()).unwrap();
        } else {
            panic!("Failed to serialize the configuration.");
        };
    }

    pub fn update<'a>(&'a mut self, cli_args: &CliArgs) -> &'a mut Config {
        if !cli_args.subscription.is_empty() {
            self.subscription = Some(cli_args.subscription.clone());
        }
        let mut updated_sessions: Vec<Session> = Vec::new();
        for session in self.sessions() {
            let mut updated_session = session.clone();
            let mut updated_commands: Vec<Command> = Vec::new();
            for command in session.commands() {
                let mut updated_command = command.clone();
                if updated_command.resource_group.is_none() {
                    let resource_group_name = format!("{}-{}", session.name(), cli_args.event);
                    updated_command.resource_group = Some(resource_group_name);
                }
                if updated_command.location.is_none() {
                    updated_command.location = Some(cli_args.location.clone());
                }
                updated_commands.push(updated_command);
            }
            updated_session.commands = Some(updated_commands);
            updated_sessions.push(updated_session);
        }
        self.sessions = Some(updated_sessions);
        self
    }

    pub fn subscription(&self) -> String {
        let config = self.clone();
        if let Some(s) = config.subscription {
            s
        } else {
            panic!("A subscription needs to be defined, either in the configuration file or on the command line.")
        }
    }

    pub fn sessions(&self) -> Vec<Session> {
        let config = self.clone();
        if let Some(s) = config.sessions {
            s
        } else {
            Vec::new()
        }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config {
            subscription: None,
            sessions: None,
        }
    }
}
