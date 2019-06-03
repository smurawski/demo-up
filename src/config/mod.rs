pub mod arm_template;
pub mod cli_command;
pub mod command;
pub mod config_struct;
pub mod file_download;
pub mod session;

pub use self::arm_template::ArmTemplate;
pub use self::cli_command::CliCommand;
pub use self::command::Command;
pub use self::config_struct::Config;
pub use self::file_download::FileDownload;
pub use self::session::Session;

use reqwest::{Client, Url};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;

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
                panic!("Failed to locate any valid configuration file.")
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
    #[should_panic]
    fn no_valid_config_provided_located_panics() {
        get_config(&"./missing.yml");
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
