use super::Command;
use super::FileDownload;
use serde::{Deserialize, Serialize};
use cli::CliArgs;

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

    pub fn update<'a>(&'a mut self, cli_args: &CliArgs) -> &'a mut Session {
        let commands = self.commands().iter().map(|c| c.update(&self.name(), cli_args); c).collect();
        self.commands = Some(commands);
        self
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
