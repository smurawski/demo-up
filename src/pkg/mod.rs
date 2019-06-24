pub mod environment;
pub mod parameter_option;
pub mod pkg_command;

pub use self::environment::Environment;
pub use self::parameter_option::ParameterOption;
pub use self::pkg_command::PkgCommand;

use custom_error::custom_error;

custom_error! {
    pub DemoPkgError
    Failed = "Failed to create the package.",
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::collections::HashMap;

    impl PkgCommand {
        pub fn parameters(&self) -> Value {
            self.base_template["parameters"].clone()
        }

        pub fn variables(&self) -> Value {
            self.base_template["variables"].clone()
        }

        pub fn environment(&self) -> Value {
            self.base_template["resources"][3]["properties"]["containers"][0]["properties"]
                ["environmentVariables"]
                .clone()
        }
    }

    #[test]
    fn default_base_template_is_not_empty() {
        let default = PkgCommand::default();
        assert!(!default.base_template.to_string().is_empty());
    }

    #[test]
    fn default_base_template_has_no_parameters() {
        let default = PkgCommand::default();
        let parameters = default.parameters();
        assert!(parameters.is_object());
        assert_eq!(parameters.as_object().unwrap().len(), 0);
    }

    #[test]
    fn default_base_template_has_no_variables() {
        let default = PkgCommand::default();
        let variables = default.variables();
        assert!(variables.is_object());
        assert_eq!(variables.as_object().unwrap().len(), 0);
    }

    #[test]
    fn default_base_template_has_no_environment_variables() {
        let default = PkgCommand::default();
        let environment = default.environment();

        assert!(environment.is_array());
        assert!(environment.as_array().unwrap().is_empty());
    }

    #[test]
    fn add_parameters_to_output() {
        let mut default = PkgCommand::default();
        let mut params = HashMap::new();
        params.insert("one", ParameterOption::default());
        let mut parameter_with_default_value = ParameterOption::default();
        parameter_with_default_value.default_value = Some("hi".to_string());
        params.insert("three", parameter_with_default_value);
        default.base_template["parameters"] = serde_json::to_value(params).unwrap();
    }
}
