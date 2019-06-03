use super::get_output_arg;
use clap::App;

pub fn get_fetch_subcommand<'a, 'b>() -> App<'a, 'b> {
    App::new("fetch")
        .about("Retrieves a local copy of a configuration file for the demo environment for one or more learning paths or sessions.")
        .arg(
            get_output_arg(),
        )
}
