use super::args::{
    get_environment_file_arg, get_parameter_file_arg, get_session_name_arg, get_variable_file_arg,
};
use clap::{App, AppSettings};

pub fn get_pkg_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("pkg")
        .about("Creates the bootstrap ARM template.")
        .setting(AppSettings::Hidden)
        .arg(get_parameter_file_arg())
        .arg(get_environment_file_arg())
        .arg(get_variable_file_arg())
        .arg(get_session_name_arg())
}
