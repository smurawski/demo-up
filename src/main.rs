#[macro_use]
extern crate clap;
extern crate custom_error;
extern crate duct;
extern crate git2;
extern crate regex;
extern crate reqwest;
extern crate serde;
extern crate serde_json;
extern crate serde_yaml;
extern crate uuid;

mod cli;
mod config;
mod fetch;
mod pkg;
mod tasks;
mod up;
use cli::get_app_cli;

use fetch::FetchCommand;
use pkg::PkgCommand;
use up::UpCommand;

fn main() {
    let version = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_BUILDID").unwrap_or("0")
    );

    let matches = get_app_cli(&version).get_matches();

    if let Some(sub_matches) = matches.subcommand_matches("up") {
        let mut up_config = UpCommand::default();

        up_config.config_path = get_single_argument(&sub_matches, "config_file");
        up_config.subscription = get_single_argument(&sub_matches, "subscription");
        up_config.event = Some(get_single_argument(&sub_matches, "event"));

        if sub_matches.is_present("learning_path") {
            let learning_paths = get_vec_argument(&sub_matches, "learning_path");
            let mut sessions = Vec::new();
            for l in learning_paths {
                sessions.extend(map_learning_path_to_session_name(&l));
            }
            up_config.session_names = Some(sessions);
        };

        if sub_matches.is_present("session_name") {
            let sessions = get_vec_argument(&sub_matches, "session_name");
            up_config.session_names = Some(sessions);
        };

        if sub_matches.is_present("exclude") {
            let excludes = get_vec_argument(&sub_matches, "exclude");
            up_config.exclude = Some(excludes);
        };

        up_config.execute().unwrap();
    }

    if let Some(sub_matches) = matches.subcommand_matches("fetch") {
        let output_file = get_single_argument(&sub_matches, "OUTPUT");
        let config_path = get_single_argument(&sub_matches, "config_file");
        let fetch_config = FetchCommand::new(config_path, output_file);
        fetch_config.execute().unwrap();
    };

    // if let Some(sub_matches) = matches.subcommand_matches("down") {
    // }

    if let Some(sub_matches) = matches.subcommand_matches("pkg") {
        let mut pkg_config = PkgCommand::default();
        pkg_config.set_parameters_path(get_single_argument(&sub_matches, "parameters_file_path"));
        pkg_config.set_environment_path(get_single_argument(&sub_matches, "environment_file_path"));
        if sub_matches.is_present("variables_file_path") {
            pkg_config
                .set_variables_path(get_single_argument(&sub_matches, "environment_file_path"));
        };
        pkg_config.load().update().write().unwrap();
    }
}

fn get_single_argument(matches: &clap::ArgMatches, arg: &str) -> String {
    match matches.value_of(arg) {
        Some(s) => s.to_string(),
        _ => panic!("Failed find {}.", arg),
    }
}

fn get_vec_argument(matches: &clap::ArgMatches, arg: &str) -> Vec<String> {
    matches
        .values_of(arg)
        .unwrap()
        .map(|x| x.to_string())
        .collect()
}

fn map_learning_path_to_session_name(learning_path: &str) -> Vec<String> {
    let session_numbers = vec!["10", "20", "30", "40", "50"];
    session_numbers
        .iter()
        .map(|n| session_name(learning_path, *n))
        .collect()
}

fn session_name(learning_path: &str, session_number: &str) -> String {
    format!("{}{}", learning_path, session_number).to_string()
}
