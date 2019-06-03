use clap::Arg;

arg_enum! {
    #[derive(Debug)]
    enum TalkTrack {
        DAT,
        DEV,
        FUN,
        HYB,
        MIG,
        SRE,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum SessionNames {
        DAT10,
        DAT20,
        DAT30,
        DAT40,
        DAT50,
        DEV10,
        DEV20,
        DEV30,
        DEV40,
        DEV50,
        FUN10,
        FUN20,
        FUN30,
        FUN40,
        FUN50,
        HYB10,
        HYB20,
        HYB30,
        HYB40,
        HYB50,
        MIG10,
        MIG20,
        MIG30,
        MIG40,
        MIG50,
        SRE10,
        SRE20,
        SRE30,
        SRE40,
        SRE50,
    }
}

arg_enum! {
    #[derive(Debug)]
    enum SessionSections {
        Slides,
        Videos,
        GitRepos,
        Commands,
    }
}

pub fn get_user_environment_variable() -> &'static str {
    if cfg!(windows) {
        "USERNAME"
    } else {
        "USER"
    }
}

pub fn get_event_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("event")
        .long("event")
        .short("e")
        .help("Event name (to keep environments unique).  Defaults to your local user name.")
        .env(get_user_environment_variable())
}

pub fn get_learning_path_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("learning_path")
        .multiple(true)
        .long("learning-path")
        .short("l")
        .help("Learning path.")
        .possible_values(&TalkTrack::variants())
        .takes_value(true)
}

pub fn get_session_name_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("session_name")
        .multiple(true)
        .long("session-name")
        .short("s")
        .help("Session name.")
        .possible_values(&SessionNames::variants())
        .conflicts_with("learning_path")
        .takes_value(true)
}

pub fn get_exclude_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("exclude")
        .long("exclude")
        .help("Sections of the session to skip retrieval or exectution.")
        .possible_values(&SessionSections::variants())
        .multiple(true)
        .takes_value(true)
}

pub fn get_output_arg<'a, 'b>() -> Arg<'a, 'b> {
    Arg::with_name("OUTPUT")
        .help("Path to write the local configuration file to use.")
        .index(1)
        .default_value("./demo.yml")
}
