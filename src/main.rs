mod fs_watcher;

use std::process::Command;
use std::time::Duration;

fn main() {
    let mut command = Command::new("ls");
    
    let watch_args = fs_watcher::WatchArgs::default()
        .with_path("./dir")
        .with_delay(Duration::from_millis(150))
        .with_callback(move |event| {
            println!("{:?}", event);
            
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
