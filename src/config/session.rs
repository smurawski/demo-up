use super::Command;
use super::FileDownload;
use serde::{Deserialize, Serialize};

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
