use cli::CliArgs;
use reqwest::{Client, Url};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
                let argument = format!("{}", v);
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

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct FileDownload {
    pub file_name: Option<String>,
    pub url: Option<String>,
}

impl Default for FileDownload {
    fn default() -> Self {
        FileDownload {
            file_name: None,
            url: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Session {
    pub name: Option<String>,
    pub slides: Option<FileDownload>,
    pub videos: Option<Vec<FileDownload>>,
    pub git_repos: Option<Vec<String>>,
    pub commands: Option<Vec<Command>>,
}

impl Session {
    pub fn commands(&self) -> Vec<Command> {
        let s = self.clone();
        if let Some(c) = s.commands {
            c
        } else {
            Vec::new()
        }
    }

    pub fn name(&self) -> String {
        let s = self.clone();
        if let Some(n) = s.name {
            n
        } else {
            panic!("Sessions must have a name.")
        }
    }
}

impl Default for Session {
    fn default() -> Self {
        Session {
            name: None,
            slides: None,
            git_repos: None,
            videos: None,
            commands: None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Config {
    pub subscription: Option<String>,
    pub sessions: Option<Vec<Session>>,
}

impl Config {
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

fn read(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn load(yaml_str: &str) -> Config {
    match serde_yaml::from_str(&yaml_str) {
        Ok(s) => s,
        Err(_e) => Config::default(),
    }
}

fn read_from_url(url: Url) -> Result<String, io::Error> {
    let client = Client::new();
    let mut resp = client
        .get(url)
        .send()
        .expect("Failed to retrieve the configuration file.");
    let mut contents = String::new();
    resp.read_to_string(&mut contents)?;

    Ok(contents)
}

pub fn get_config(path: &str) -> Config {
    let content = match Url::parse(path) {
        Ok(url) => read_from_url(url).unwrap(),
        Err(_) => {
            if Path::new(path).exists() {
                read(path).unwrap()
            } else {
                let default_path = "./demo.yml";
                if Path::new(default_path).exists() {
                    read(default_path).unwrap()
                } else {
                    panic!("Failed to locate any valid configuration file.")
                }
            }
        }
    };
    load(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_empty_config() -> Config {
        get_config(&"./test/artifacts/empty_config.yml")
    }

    fn load_single_session_config() -> Config {
        get_config(&"./test/artifacts/single_session_config.yml")
    }

    fn get_single_session() -> Session {
        let config_result = load_single_session_config();

        let mut session_result = Session::default();

        for session in config_result.sessions.unwrap() {
            session_result = session;
        }
        session_result
    }

    #[test]
    fn empty_config_is_valid() {
        let config_result = load_empty_config();

        assert!(config_result.subscription.is_none());
        assert!(config_result.sessions.is_none());
    }

    #[test]
    fn single_session_config_is_valid() {
        let config_result = load_single_session_config();

        assert!(config_result.subscription.is_some());
        assert!(config_result.sessions.is_some());
    }

    #[test]
    fn single_session_is_valid() {
        let session_result = get_single_session();

        assert!(session_result.name.is_some());
    }

    #[test]
    fn single_session_has_name() {
        let session_result = get_single_session();

        assert_eq!(session_result.name.unwrap(), "SRE30")
    }

    #[test]
    fn single_session_has_command() {
        let session_result = get_single_session();

        assert!(session_result.commands.is_some());
    }

    #[test]
    fn single_session_has_command_without_order() {
        let session_result = get_single_session();
        let commands = session_result.commands.unwrap().to_vec();
        let command = &commands[0];
        let order = command.order;

        assert!(order.is_none());
    }

    #[test]
    fn single_session_command_has_template_no_parameters() {
        let session_result = get_single_session();
        let commands = session_result.commands.unwrap().to_vec();

        if let Some(template) = &commands[0].template {
            assert!(template.path.is_some());
            let template_path = template.clone().path.unwrap();
            assert_eq!(template_path, "./test/artifacts/demo_template.json");
            assert!(template.parameters.is_none());
        }
    }
}
