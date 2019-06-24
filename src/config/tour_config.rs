use super::Command;
use super::Session;
use crate::up::UpCommand;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct TourConfig {
    pub subscription: Option<String>,
    pub sessions: Option<Vec<Session>>,
}

impl TourConfig {
    pub fn write(&self, file_path: &PathBuf) {
        if let Ok(content) = serde_yaml::to_string(self) {
            let mut file = File::create(file_path).expect("Failed to create the output file.");
            file.write_all(content.into_bytes().as_ref()).unwrap();
        } else {
            panic!("Failed to serialize the configuration.");
        };
    }

    pub fn filter_sessions(&self, session_names: &[String]) -> Vec<Session> {
        let mut current_sessions = self.sessions();
        current_sessions.retain(|s| session_names.contains(&s.name()));
        current_sessions
    }

    pub fn update<'a>(&'a mut self, cli_args: &UpCommand) -> &'a mut TourConfig {
        if !cli_args.subscription.is_empty() {
            self.subscription = Some(cli_args.subscription.clone());
        }
        let mut updated_sessions: Vec<Session> = Vec::new();
        for session in self.filter_sessions(&cli_args.session_names()) {
            let mut updated_session = session.clone();
            let mut updated_commands: Vec<Command> = Vec::new();
            for command in session.commands() {
                let mut updated_command = command.clone();
                if updated_command.resource_group.is_none() {
                    let resource_group_name = format!("{}-{}", session.name(), cli_args.event());
                    updated_command.resource_group = Some(resource_group_name);
                }
                if updated_command.location.is_none() {
                    updated_command.location = cli_args.location.clone();
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

impl Default for TourConfig {
    fn default() -> Self {
        TourConfig {
            subscription: None,
            sessions: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn session_filter_returns_correct_sessions() {
        let mut session_a = Session::default();
        session_a.name = Some("SRE10".to_string());
        let mut session_b = Session::default();
        session_b.name = Some("SRE20".to_string());
        let mut session_c = Session::default();
        session_c.name = Some("SRE30".to_string());

        let mut tour_config = TourConfig::default();
        tour_config.sessions = Some(vec![session_a, session_b, session_c]);

        let filter_list = vec!["SRE20".to_string()];

        let result = tour_config.filter_sessions(&filter_list);
        let mut session_d = Session::default();
        session_d.name = Some("SRE20".to_string());
        let expected = vec![session_d];

        assert_eq!(result, expected);
    }
}
