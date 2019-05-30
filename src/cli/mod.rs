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
    pub config_path: String,
    pub subscription: String,
    pub event: String,
    pub session_names: Vec<String>,
    pub location: String,
}

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    return App::new("demo")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Sets up or tears down demo environments for Microsoft Ignite | The Tour")
        .subcommand(get_up_subcommand())
        .subcommand(get_down_subcommand())
        .subcommand(get_pkg_subcommand())
        .arg(
            Arg::with_name("config_file")
                .long("config-file")
                .short("c")
                .takes_value(true)
                .default_value("https://gist.githubusercontent.com/smurawski/414b5cc1f72edbe26c82907d23e36eb5/raw/8b23668c2ad2614740f154ec250a3d99309be567/demo.yml")
        )
        .arg(
            Arg::with_name("subscription")
                .long("subscription")
                .short("S")
                .takes_value(true)
        );
}

fn get_up_subcommand<'a, 'b>() -> App<'a, 'b> {
    return App::new("up")
        .about("Sets up the demo environment for one or more learning paths or sessions.")
        .arg(get_event_arg())
        .arg(get_learning_path_arg())
        .arg(get_session_name_arg());
}

fn get_down_subcommand<'a, 'b>() -> App<'a, 'b> {
    return App::new("down");
}

fn get_pkg_subcommand<'a, 'b>() -> App<'a, 'b> {
    return App::new("pkg");
}

fn get_event_arg<'a, 'b>() -> Arg<'a, 'b> {
    return Arg::with_name("event")
        .long("event")
        .short("e")
        .help("Event name (to keep environments unique).  Defaults to your local user name.")
        .env("USERNAME");
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
