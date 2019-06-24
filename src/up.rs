use crate::config::get_config;
use serde::{Deserialize, Serialize};

use crate::tasks::*;
use custom_error::custom_error;
use std::env;
use std::path::PathBuf;

custom_error! {
    pub DemoUpError
    Failed = "Failed to create the environment",
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UpCommand {
    pub config_path: String,
    pub subscription: String,
    pub event: Option<String>,
    pub session_names: Option<Vec<String>>,
    pub location: Option<String>,
    pub exclude: Option<Vec<String>>,
}

impl UpCommand {
    pub fn event(&self) -> String {
        match self.event {
            Some(ref ev) => ev.to_string(),
            None => panic!("No event supplied and no default found."),
        }
    }

    pub fn session_names(&self) -> Vec<String> {
        match self.session_names.clone() {
            Some(s) => s,
            None => Vec::new(),
        }
    }

    pub fn execute(&self) -> Result<(), DemoUpError> {
        let cli_args = self.clone();

        println!("Loading the configuration from {}\n", &cli_args.config_path);
        let mut tour_config = get_config(&cli_args.config_path);
        tour_config.update(&cli_args);

        if !self.skip_section("Commands") {
            set_azure_environment(&tour_config.subscription()).unwrap();
        }

        let starting_directory = env::current_dir().unwrap();

        for s in tour_config.sessions() {
            let session_name = s.name();
            println!("\t{}: Starting setup", &session_name);

            let session_directory = starting_directory.to_path_buf().join(s.name.unwrap());
            let slides_directory = session_directory.join("slides");
            let video_directory = session_directory.join("videos");
            let source_directory = session_directory.join("src");

            println!("\t{}: Creating session directory", &session_name);
            create_directory(&session_directory);

            if !self.skip_section("Slides") && s.slides.is_some() {
                println!("\t{}: Creating slides directory.", &session_name);
                create_directory(&slides_directory);

                let slides = s.slides.unwrap();
                env::set_current_dir(&slides_directory).unwrap();
                let slides_url = slides.url.unwrap();
                let default_filename = match slides.file_name {
                    Some(n) => n,
                    None => "".to_string(),
                };
                let slides_filename = get_filename(&slides_url, &default_filename);
                println!(
                    "\t{}: Downloading {} to the slides directory.",
                    &session_name, &slides_filename
                );

                download_file(&slides_url, &slides_filename);
            };

            if !self.skip_section("Videos") && s.videos.is_some() {
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
            };

            if !self.skip_section("GitRepos") && s.git_repos.is_some() {
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
            };

            env::set_current_dir(&starting_directory).unwrap();

            if !self.skip_section("Commands") {
                if let Some(mut commands) = s.commands {
                    println!("\t{}: Creating the Azure environment.", &session_name);

                    commands.sort_by_key(|c| c.order());
                    for command in commands {
                        if command.template.is_some() {
                            println!("\t\t{}: Deploying an ARM template", &session_name);

                            deploy_template(&command).unwrap();
                        };
                        if command.cli.is_some() {
                            println!("\t\t{}: Running a CLI command", &session_name);

                            run_cli_command(&command).unwrap();
                        };
                    }
                };
            };
        }

        Ok(())
    }

    fn skip_section(&self, section: &str) -> bool {
        match self.exclude {
            Some(ref exclude) if exclude.contains(&section.to_string()) => true,
            _ => false,
        }
    }
}

fn create_directory(path: &PathBuf) {
    match std::fs::create_dir(path) {
        _ => {}
    };
}

impl Default for UpCommand {
    fn default() -> UpCommand {
        UpCommand {
            config_path: "".to_string(),
            subscription: "".to_string(),
            event: None,
            session_names: None,
            location: None,
            exclude: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn skips_correct_section() {
        let mut config = UpCommand::default();
        config.exclude = Some(vec!["Slides".to_string()]);

        assert!(config.skip_section("Slides"));
    }

    #[test]
    fn does_not_skip_correct_section() {
        let mut config = UpCommand::default();
        config.exclude = Some(vec!["Video".to_string()]);

        assert!(!config.skip_section("Slides"));
    }
}
