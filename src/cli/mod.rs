mod args;
mod fetch;
mod up;

use self::args::{
    get_event_arg, get_exclude_arg, get_learning_path_arg, get_output_arg, get_session_name_arg,
};
use self::fetch::get_fetch_subcommand;
use self::up::get_up_subcommand;
use clap::{App, Arg};
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliArgs {
    pub config_path: String,
    pub subscription: String,
    pub event: String,
    pub session_names: Vec<String>,
    pub location: String,
    pub exclude: Vec<String>,
}

impl Default for CliArgs {
    fn default() -> CliArgs {
        CliArgs {
            config_path: "".to_string(),
            subscription: "".to_string(),
            event: "".to_string(),
            session_names: Vec::new(),
            location: "".to_string(),
            exclude: Vec::new(),
        }
    }
}

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("demo")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Sets up or tears down demo environments for Microsoft Ignite | The Tour")
        .subcommand(get_up_subcommand())
        .subcommand(get_fetch_subcommand())
        // .subcommand(get_down_subcommand())
        // .subcommand(get_pkg_subcommand())
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .short("c")
                .takes_value(true)
                .default_value(default_config_path()),
        )
        .arg(
            Arg::with_name("subscription")
                .long("subscription")
                .short("S")
                .takes_value(true),
        )
}

fn default_config_path() -> &'static str {
    if Path::new("demo.yml").exists() {
        "demo.yml"
    } else {
        "https://aka.ms/demo-up"
    }
}

// fn get_down_subcommand<'a, 'b>() -> App<'a, 'b> {
//     return App::new("down");
// }

// fn get_pkg_subcommand<'a, 'b>() -> App<'a, 'b> {
//     return App::new("pkg");
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_no_parameters() {
        let args = vec!["demo"];

        let cli = get_app_cli("0.0.1");
        let matches = cli.get_matches_from(args);

        let config_file = matches.value_of("config_file");
        let subscription = matches.values_of("subscription");

        assert!(config_file.is_some());
        assert!(subscription.is_none());
    }

    #[test]
    fn demo_no_parameters_no_local_config() {
        let args = vec!["demo"];

        let cli = get_app_cli("0.0.1");
        let matches = cli.get_matches_from(args);

        let config_file = matches.value_of("config_file");

        assert!(config_file.is_some());
        assert_eq!(config_file.unwrap(), "https://aka.ms/demo-up");
    }

    #[test]
    fn demo_with_config_path() {
        let args = vec!["demo", "--config-file", "some_local_file"];

        let cli = get_app_cli("0.0.1");
        let matches = cli.get_matches_from(args);

        let config_file = matches.value_of("config_file");

        assert!(config_file.is_some());
        assert_eq!(config_file.unwrap(), "some_local_file");
    }

    #[test]
    fn demo_with_subscription() {
        let args = vec!["demo", "--subscription", "your_azure_subscription"];

        let cli = get_app_cli("0.0.1");
        let matches = cli.get_matches_from(args);

        let subscription = matches.value_of("subscription");

        assert!(subscription.is_some());
        assert_eq!(subscription.unwrap(), "your_azure_subscription");
    }
}
