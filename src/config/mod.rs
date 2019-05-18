use serde::{Serialize, Deserialize};
use std::collections::HashMap;
use std::fmt;
use std::fs::File;
use std::io::prelude::*;
use std::io;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct Session {
    pub name: Option<String>,
    pub template: Option<ArmTemplate>,
}

impl Default for Session {
    fn default() -> Self {
        Session {
            name: None,
            template: None
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub struct ArmTemplate {
    pub path: Option<String>,
    pub parameters: Option<Vec<HashMap<String, String>>>
}

impl Default for ArmTemplate {
    fn default() -> Self {
        ArmTemplate {
            path: None,
            parameters: None,
        }
    }
}

fn read(path: &str) -> Result<String, io::Error> {
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    Ok(contents)
}

fn load(yaml_str: &str) -> Session {
    match serde_yaml::from_str(&yaml_str) {
        Ok(s) => s,
        Err(_e) => Session::default()
    }
}

pub fn get_config(path: &str) -> Session {
    let content = read(path).unwrap();
    load(&content)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn load_empty_config() -> Session {
        get_config(&"./test/artifacts/empty_config.yml")

    }

    fn load_single_session_config() -> Session {
        get_config(&"./test/artifacts/single_session_config.yml")
    }


    #[test]
    fn empty_config_is_valid() {
        let session_result = load_empty_config();

        assert!(session_result.name.is_none());
        assert!(session_result.template.is_none());
    }

    #[test]
    fn single_session_is_valid() {
        let session_result = load_single_session_config();

        assert!(session_result.name.is_some());
        assert!(session_result.template.is_some());
    }

    #[test]
    fn single_session_has_name() {
        let session = load_single_session_config();

        assert!(session.name.is_some());

        if let Some(name) = session.name {
            assert_eq!(name, "SRE30");
        }
    }

    #[test]
    fn single_session_has_template_no_parameters() {
        let session = load_single_session_config();

        assert!(session.template.is_some());

        if let Some(template) = session.template {
            assert!(template.path.is_some());
            let template_path = template.path.unwrap();
            assert_eq!(template_path, "./test/artifacts/demo_template.json");
            assert!(template.parameters.is_none());
        }
    }



}