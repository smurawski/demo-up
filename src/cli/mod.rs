mod args;
mod fetch;
mod pkg;
mod up;

use self::fetch::get_fetch_subcommand;
use self::pkg::get_pkg_subcommand;
use self::up::get_up_subcommand;

use clap::App;

pub fn get_app_cli<'a, 'b>(version: &'b str) -> App<'a, 'b> {
    App::new("demo")
        .version(&*version)
        .author("Steven Murawski <steven.murawski@microsoft.com>")
        .about("Sets up or tears down demo environments for Microsoft Ignite | The Tour")
        .subcommand(get_up_subcommand())
        .subcommand(get_pkg_subcommand())
        .subcommand(get_fetch_subcommand())
    // .subcommand(get_down_subcommand())
}
