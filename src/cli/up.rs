use super::args::*;
use clap::App;

pub fn get_up_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("up")
        .about("Sets up the demo environment for one or more learning paths or sessions.")
        .arg(get_subscription_arg())
        .arg(get_config_file_arg())
        .arg(get_event_arg())
        .arg(get_learning_path_arg())
        .arg(get_session_name_arg())
        .arg(get_exclude_arg())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_up_no_parameters() {
        let args = vec!["up"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from(args);

        let event = matches.value_of("event");
        let learning_path = matches.values_of("learning_path");
        let session_name = matches.values_of("session_name");
        let config_file = matches.value_of("config_file");
        let subscription = matches.values_of("subscription");

        assert!(config_file.is_some());
        assert!(subscription.is_none());
        assert!(event.is_some());
        assert!(learning_path.is_none());
        assert!(session_name.is_none());
    }

    #[test]
    fn demo_no_parameters_no_local_config() {
        let args = vec!["up"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from(args);

        let config_file = matches.value_of("config_file");

        assert!(config_file.is_some());
        assert_eq!(config_file.unwrap(), "https://aka.ms/demo-up");
    }

    #[test]
    fn demo_with_config_path() {
        let args = vec!["up", "--config-file", "some_local_file"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from(args);

        let config_file = matches.value_of("config_file");

        assert!(config_file.is_some());
        assert_eq!(config_file.unwrap(), "some_local_file");
    }

    #[test]
    fn demo_with_subscription() {
        let args = vec!["up", "--azure-subscription", "your_azure_subscription"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from(args);

        let subscription = matches.value_of("subscription");

        assert!(subscription.is_some());
        assert_eq!(subscription.unwrap(), "your_azure_subscription");
    }

    #[test]
    fn demo_up_only_learning_path() {
        let args = vec!["up", "--learning-path", "DAT"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from(args);

        let event = matches.value_of("event");
        let learning_path = matches.values_of("learning_path");
        let session_name = matches.values_of("session_name");

        assert!(event.is_some());
        assert!(learning_path.is_some());
        assert!(session_name.is_none());
    }

    #[test]
    fn demo_up_only_learning_path_wrong_value() {
        let args = vec!["up", "--learning-path", "BOO"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from_safe(args);

        assert!(matches.is_err());
    }

    #[test]
    fn demo_up_only_session_name() {
        let args = vec!["up", "--session-name", "DAT10"];

        let cli = get_up_subcommand();
        let matches = cli.get_matches_from(args);

        let event = matches.value_of("event");
        let learning_path = matches.values_of("learning_path");
        let session_name = matches.values_of("session_name");

        assert!(event.is_some());
        assert!(learning_path.is_none());
        assert!(session_name.is_some());
    }

    // #[test]
    // fn demo_up_learning_path_and_session_error() {
    //     let cli = get_up_subcommand();
    //     let args = vec!["--learning-path SRE", "--session-name DAT10"];
    //     let good_response = cli.get_matches_from(args);

    //     assert!(good_response.is_err())
    // }

    // #[test]
    // fn demo_down_no_parameters() {
    //     let args = vec!["down"];

    //     let cli = get_down_subcommand();
    //     let matches = cli.get_matches_from_safe(args);

    //     assert!(matches.is_ok());
    // }

    // #[test]
    // fn demo_pkg_no_parameters() {
    //     let args = vec!["pkg"];

    //     let cli = get_pkg_subcommand();
    //     let matches = cli.get_matches_from_safe(args);

    //     assert!(matches.is_ok());
    // }
}
