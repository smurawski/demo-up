use super::*;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;

use crate::config::read;
use std::fs::File;
use std::io::prelude::*;
use std::path::PathBuf;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct PkgCommand {
    pub base_template: Value,
    pub output_file_path: PathBuf,
    pub parameters_path: PathBuf,
    pub environment_path: PathBuf,
    pub variables_path: Option<PathBuf>,
    pub variables: HashMap<String, String>,
    pub parameters: HashMap<String, ParameterOption>,
    pub environment: Vec<Environment>,
}

impl PkgCommand {
    pub fn load(&mut self) -> &mut PkgCommand {
        self.parameters = match read(&*self.parameters_path) {
            Ok(p) => serde_json::from_str(&p).unwrap(),
            Err(_) => HashMap::new(),
        };
        self.environment = match read(&*self.environment_path) {
            Ok(en) => serde_json::from_str(&en).unwrap(),
            Err(_) => Vec::new(),
        };

        self
    }
    pub fn set_parameters_path(&mut self, path: String) -> &mut PkgCommand {
        self.parameters_path = PathBuf::from(path);
        self
    }

    pub fn set_variables_path(&mut self, path: String) -> &mut PkgCommand {
        self.variables_path = Some(PathBuf::from(path));
        self
    }

    pub fn set_environment_path(&mut self, path: String) -> &mut PkgCommand {
        self.environment_path = PathBuf::from(path);
        self
    }

    pub fn update(&mut self) -> &mut PkgCommand {
        self.base_template["parameters"] = serde_json::to_value(&self.parameters).unwrap();
        self.base_template["variables"] = serde_json::to_value(&self.variables).unwrap();
        self.base_template["resources"][3]["properties"]["containers"][0]["properties"]
            ["environmentVariables"] = serde_json::to_value(&self.environment).unwrap();
        self
    }

    pub fn write(&self) -> Result<(), DemoPkgError> {
        if let Ok(content) = serde_json::to_string(&self.base_template) {
            let mut file =
                File::create(&self.output_file_path).expect("Failed to create the output file.");
            file.write_all(content.into_bytes().as_ref()).unwrap();
            Ok(())
        } else {
            Err(DemoPkgError::Failed)
        }
    }
}

impl Default for PkgCommand {
    fn default() -> Self {
        PkgCommand {
            base_template: serde_json::from_str(include_str!("./azuredeploy.json")).unwrap(),
            output_file_path: PathBuf::from("./azuredeploy.json"),
            parameters_path: PathBuf::from("./test/artifacts/parameters.json"),
            variables_path: None,
            environment_path: PathBuf::from("./test/artifacts/environment.json"),
            variables: serde_json::from_str(include_str!("./default_variables.json")).unwrap(),
            parameters: HashMap::new(),
            environment: Vec::new(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parameters_path_modifies_in_place() {
        let mut test = PkgCommand::default();
        test.set_parameters_path("./test.json".to_string());
        assert_eq!(test.parameters_path, PathBuf::from("./test.json"));
    }

    #[test]
    fn environment_path_modifies_in_place() {
        let mut test = PkgCommand::default();
        test.set_environment_path("./test.json".to_string());
        assert_eq!(test.environment_path, PathBuf::from("./test.json"));
    }

    #[test]
    fn variables_path_modifies_in_place() {
        let mut test = PkgCommand::default();
        test.set_variables_path("./test.json".to_string());
        assert_eq!(test.variables_path, Some(PathBuf::from("./test.json")));
    }

    #[test]
    fn update_adds_variables() {
        let mut test = PkgCommand::default();
        test.update();
        assert_eq!(test.base_template["variables"]["sessionCode"], "DEV10");
        assert_eq!(
            test.base_template["variables"]["identityName"],
            "bootstrap-creds"
        );
    }
}
