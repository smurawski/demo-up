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
mod tasks;

use cli::*;
use config::get_config;
use std::env;
use std::path::PathBuf;
use tasks::{
    deploy_template, download_file, get_filename, git_clone, run_cli_command, set_azure_environment,
};

fn main() {
    let version = format!(
        "{}.{}",
        env!("CARGO_PKG_VERSION"),
        option_env!("BUILD_BUILDID").unwrap_or("0")
    );
    let session_numbers = vec!["10", "20", "30", "40", "50"];

    let matches = get_app_cli(&version).get_matches();
    let mut cli_args = CliArgs {
        config_path_provided: matches.is_present("config_file"),
        config_path: matches.value_of("config_file").unwrap().to_string(),
        subscription: "".to_string(),
        event: "".to_string(),
        session_names: Vec::new(),
        location: "westus2".to_string(),
    };
    cli_args.subscription = match matches.value_of("subscription") {
        Some(s) => s.to_string(),
        _ => "".to_string(),
    };

    if let Some(sub_matches) = matches.subcommand_matches("up") {
        cli_args.event = match sub_matches.value_of("event") {
            Some(e) => e.to_string(),
            None => "".to_string(),
        };

        if sub_matches.is_present("learning_path") {
            let learning_paths: Vec<_> = sub_matches.values_of("learning_path").unwrap().collect();
            for learning_path in learning_paths {
                for session_number in &session_numbers {
                    cli_args
                        .session_names
                        .push(format!("{}{}", learning_path, session_number).to_string());
                }
            }
        };
        if sub_matches.is_present("session_name") {
            cli_args.session_names = sub_matches
                .values_of("session_name")
                .unwrap()
                .map(|x| x.to_string())
                .collect();
        }

        println!("Loading the configuration from {}\n", &cli_args.config_path);
        let mut config = get_config(&cli_args.config_path, &cli_args.config_path_provided);
        config.update(&cli_args);

        set_azure_environment(&config.subscription()).unwrap();

        let starting_directory = env::current_dir().unwrap();
        for session_name in cli_args.session_names {
            let mut current_sessions = config.sessions();
            println!("Setting up environment for {}\n", &session_name);
            current_sessions.retain(|s| s.name() == session_name);
            for s in current_sessions {
                println!("\t{}: Starting setup", &session_name);
                let session_directory = starting_directory.to_path_buf().join(s.name.unwrap());
                let slides_directory = session_directory.join("slides");
                let video_directory = session_directory.join("videos");
                let source_directory = session_directory.join("src");

                println!("\t{}: Creating session directory", &session_name);
                create_directory(&session_directory);

                if s.slides.is_some() {
                    println!("\t{}: Creating slides directory.", &session_name);
                    create_directory(&slides_directory);

                    env::set_current_dir(&slides_directory).unwrap();
                    let slides_url = s.slides.unwrap().url.unwrap();
                    let slides_filename = get_filename(&slides_url, "");
                    println!(
                        "\t{}: Downloading {} to the slides directory.",
                        &session_name, &slides_filename
                    );

                    download_file(&slides_url, &slides_filename);
                }

                if s.videos.is_some() {
                    println!("\t{}: Creating video directory.", &session_name);
                    create_directory(&video_directory);

                    env::set_current_dir(&video_directory).unwrap();
                    for video in s.videos.unwrap() {
                        let video_url = video.url.unwrap();
                        let video_filename = get_filename(&video_url, "");
                        println!(
                            "\t{}: Downloading {} to the video directory.",
                            &session_name, &video_filename
                        );

                        download_file(&video_url, &video_filename);
                    }
                }

                if s.git_repos.is_some() {
                    println!("\t{}: Creating source directory.", &session_name);
                    create_directory(&source_directory);
                    for repo_url in s.git_repos.unwrap() {
                        let repo_name = get_filename(&repo_url, "");
                        let repo_dir = &source_directory.join(&repo_name);
                        println!(
                            "\t{}: Cloning {} into {}",
                            &session_name,
                            &repo_name,
                            &repo_dir.to_str().unwrap()
                        );
                        git_clone(&repo_url, repo_dir.to_path_buf());
                    }
                }

                env::set_current_dir(&starting_directory).unwrap();

                if let Some(mut commands) = s.commands {
                    println!("\t{}: Creating the Azure environment.", &session_name);

                    commands.sort_by_key(|c| c.order());
                    for command in commands {
                        if command.template.is_some() {
                            println!("\t\t{}: Deploying an ARM template", &session_name);

                            deploy_template(&command).unwrap();
                        }
                        if command.cli.is_some() {
                            println!("\t\t{}: Running a CLI command", &session_name);

                            run_cli_command(&command).unwrap();
                        }
                    }
                }
            }
        }
    }

    if let Some(sub_matches) = matches.subcommand_matches("fetch") {
        let output_file = match sub_matches.value_of("OUTPUT") {
            Some(f) if !f.is_empty() => f.to_string(),
            _ => panic!("No output path provided."),
        };

        println!("Loading the configuration from {}\n", &cli_args.config_path);
        let mut config = get_config(&cli_args.config_path, &cli_args.config_path_provided);
        //config.update(&cli_args);

        println!("Writing the configuration to {}\n", &output_file);
        config.write(&output_file);
    }

    // if let Some(sub_matches) = matches.subcommand_matches("down") {
    // }

    // if let Some(sub_matches) = matches.subcommand_matches("pkg") {
    // }
}

fn create_directory(path: &PathBuf) {
    match std::fs::create_dir(path) {
        _ => {}
    };
}
