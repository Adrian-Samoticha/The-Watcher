mod fs_watcher;
mod arg_parser;
mod command_parser;

use std::time::Duration;

fn main() {
    let args = arg_parser::get_matches();
    let path = args.value_of("PATH").unwrap_or(".");
    let command = args.value_of("COMMAND").unwrap_or("");
    let delay_in_ms = args.value_of("delay").unwrap_or("150").parse::<u64>().unwrap_or_else(|_| {
        println!("Invalid delay value \"{}\". Defaulting to 150.", args.value_of("delay").unwrap());
        150
    });
    let verbose = args.is_present("verbose");
    
    let mut command = command_parser::parse_command(command);
    
    let watch_args = fs_watcher::WatchArgs::default()
        .with_path(path)
        .with_delay(Duration::from_millis(delay_in_ms))
        .with_callback(move |event| {
            if verbose {
                println!("Event triggered: {:?}", event);
            }
            
            let result = command.spawn();
            if result.is_err() {
                println!("Error: {:?}", result.err());
            }
        }).with_on_watch_error(|error| {
            eprintln!("Watch error: {:?}", error);
        });
    
    let watch_result = fs_watcher::watch(watch_args);
    if watch_result.is_err() {
        eprintln!("Error: {:?}", watch_result.err().unwrap().to_string());
    }
}
