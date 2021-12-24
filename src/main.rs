mod fs_watcher;
mod arg_parser;
mod command_parser;

use std::time::Duration;

fn main() {
    let args = arg_parser::get_matches();
    let path = args.value_of("PATH").unwrap_or(".");
    let command = args.value_of("COMMAND").unwrap_or("");
    let delay_in_ms = args.value_of("delay").unwrap_or("150").parse::<u64>().unwrap_or_else(|_| {
        println!("Error: Invalid delay value \"{}\".", args.value_of("delay").unwrap());
        std::process::exit(1);
    });
    let quiet = args.is_present("quiet");
    
    let mut piped_cmd_list = command_parser::parse_piped_command(command);
    
    let watch_args = fs_watcher::WatchArgs::default()
        .with_path(path)
        .with_delay(Duration::from_millis(delay_in_ms))
        .with_callback(move |event| {
            if !quiet {
                println!("Event triggered: {:?}", event);
            }
            
            piped_cmd_list.execute();
        }).with_on_watch_error(|error| {
            eprintln!("Watch error: {:?}", error);
        });
    
    let watch_result = fs_watcher::watch(watch_args);
    if watch_result.is_err() {
        eprintln!("Error: {:?}", watch_result.err().unwrap().to_string());
        std::process::exit(1);
    }
}
