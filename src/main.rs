#[macro_use]
extern crate clap;
extern crate serde;
extern crate serde_yaml;
extern crate reqwest;

mod cli;
mod config;

use cli::*;
use config::get_config;

fn main() {

    let version = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_BUILDID").unwrap_or("0")
    );
    let matches = get_app_cli(&version).get_matches();
    let config_path = matches.value_of("config_file").unwrap();
    let config = get_config(config_path);

    if let Some(sub_matches) = matches.subcommand_matches("up") {
        let event = sub_matches.value_of("event");
        let learning_path = sub_matches.value_of("learning_path");
        let session_name = sub_matches.value_of("session_name");

        println!("{:?}", event);
        println!("{:?}", learning_path);
        println!("{:?}", session_name);
        println!("{:?}", config);

    }

    if let Some(sub_matches) = matches.subcommand_matches("down") {
    }

    if let Some(sub_matches) = matches.subcommand_matches("pkg") {
    }
}