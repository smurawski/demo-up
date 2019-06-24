use serde::{Deserialize, Serialize};

use super::up::UpConfig;

pub enum SubCommandConfig {
    UP(config: UpConfig),
    DOWN,
    PKG,
    FETCH,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CliConfig {
    config_path: Option<String>,
    subscription: Option<String>,
    subcommand_config: Option<SubCommandConfig>,
}

impl Default for CliConfig {
    fn default() -> Self {
        CliConfig {
            config_path: None,
            subscription: None,
            subcommand_config: None
        }
    }
}