extern crate clap;
use clap::{Arg, App};

/// Gets the command line arguments
pub fn get_matches() -> clap::ArgMatches<'static> {
    App::new("The Watcher")
            .version("0.1.0")
            .author("Adrian Samoticha <adrian@samoticha.de>")
            .about("Utility for watching files or directories and running commands when changes are detected.")
            .arg(Arg::with_name("delay")
                .short("d")
                .long("delay")
                .value_name("INT")
                .help("Sets the number of milliseconds to wait before executing a command when a change is detected")
                .default_value("150")
                .takes_value(true))
            .arg(Arg::with_name("PATH")
                .help("The path of the file or directory to watch")
                .required(true)
                .index(1))
            .arg(Arg::with_name("COMMAND")
                .help("The command to be executed when a file or directory change is detected")
                .required(true)
                .index(2))
            .arg(Arg::with_name("quiet")
                .short("q")
                .long("quiet")
                .help("Disables output to stdout"))
            .get_matches()
}