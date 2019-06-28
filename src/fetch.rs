use crate::config::get_config;
use custom_error::custom_error;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

custom_error! {
    pub DemoFetchError
    Failed = "Failed to retrieve the configuration file.",
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct FetchCommand {
    pub source_config_file_path: String,
    pub output_file_path: PathBuf,
}

impl FetchCommand {
    pub fn new(source_config_file_path: String, output_file_path: String) -> Self {
        FetchCommand {
            source_config_file_path,
            output_file_path: PathBuf::from(output_file_path),
        }
    }
    pub fn execute(&self) -> Result<(), DemoFetchError> {
        let config = get_config(&self.source_config_file_path);
        config.write(&self.output_file_path);
        Ok(())
    }
}
