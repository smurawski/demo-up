use clap::{App, Arg};

arg_enum! {
    #[derive(Debug)]
    enum TalkTrack {
        ALL,
        DAT,
        DEV,
        FUN,
        HYB,
        MIG,
        SRE
    }
}

#[derive(Clone, Debug)]
pub struct CliArgs {
    pub config_path_provided: bool,
    pub config_path: String,
    pub subscription: String,
    pub event: String,
    pub session_names: Vec<String>,
    pub location: String,
}

impl Default for CliArgs {
    fn default() -> CliArgs {
        CliArgs {
            config_path_provided: false,
            config_path: "".to_string(),
            subscription: "".to_string(),
            event: "".to_string(),
            session_names: Vec::new(),
            location: "".to_string(),
        }
    }
}

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    return App::new("demo")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Sets up or tears down demo environments for Microsoft Ignite | The Tour")
        .subcommand(get_up_subcommand())
        .subcommand(get_fetch_subcommand())
        // .subcommand(get_down_subcommand())
        // .subcommand(get_pkg_subcommand())
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .short("c")
                .takes_value(true)
                .default_value("https://aka.ms/demo-up"),
        )
        .arg(
            Arg::with_name("subscription")
                .long("subscription")
                .short("S")
                .takes_value(true),
        );
}

fn get_up_subcommand<'a, 'b>() -> App<'a, 'b> {
    return App::new("up")
        .about("Sets up the demo environment for one or more learning paths or sessions.")
        .arg(get_event_arg())
        .arg(get_learning_path_arg())
        .arg(get_session_name_arg());
}

fn get_fetch_subcommand<'a, 'b>() -> App<'a, 'b> {
    return App::new("fetch")
        .about("Retrieves a local copy of a configuration file for the demo environment for one or more learning paths or sessions.")
        .arg(
            Arg::with_name("OUTPUT")
                .help("Path to write the local configuration file to use.")
                .index(1)
                .default_value("./demo.yml"),
        );
}

// fn get_down_subcommand<'a, 'b>() -> App<'a, 'b> {
//     return App::new("down");
// }

// fn get_pkg_subcommand<'a, 'b>() -> App<'a, 'b> {
//     return App::new("pkg");
// }

fn get_user_environment_variable() -> &'static str {
    if cfg!(windows) {
        "USERNAME"
    } else {
        "USER"
    }
}

fn get_event_arg<'a, 'b>() -> Arg<'a, 'b> {
    return Arg::with_name("event")
        .long("event")
        .short("e")
        .help("Event name (to keep environments unique).  Defaults to your local user name.")
        .env(get_user_environment_variable());
}

fn get_learning_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    return Arg::with_name("learning_path")
        .multiple(true)
        .long("learning-path")
        .short("l")
        .help("Learning path.")
        .possible_values(&TalkTrack::variants())
        .takes_value(true);
}

fn get_session_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    return Arg::with_name("session_name")
        .multiple(true)
        .long("session-name")
        .short("s")
        .help("Session name.")
        .conflicts_with("learning_path")
        .takes_value(true);
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

        assert!(event.is_some());
        assert!(learning_path.is_none());
        assert!(session_name.is_none());
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

    #[test]
    fn demo_down_no_parameters() {
        let args = vec!["down"];

        let cli = get_down_subcommand();
        let matches = cli.get_matches_from_safe(args);

        assert!(matches.is_ok());
    }

    #[test]
    fn demo_pkg_no_parameters() {
        let args = vec!["pkg"];

        let cli = get_pkg_subcommand();
        let matches = cli.get_matches_from_safe(args);

        assert!(matches.is_ok());
    }
}
