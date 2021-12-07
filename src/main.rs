mod fs_watcher;

use std::process::Command;
use std::time::Duration;

fn main() {
    let mut command = Command::new("ls");
    
    let result = fs_watcher::watch("./dir", Duration::from_millis(150), |event| {
        println!("{:?}", event);
        
        let result = command.spawn();
        if result.is_err() {
            println!("Error: {:?}", result.err());
        }
    }, |error| {
        eprintln!("Watch error: {:?}", error);
    });
    
    if result.is_err() {
        eprintln!("Error: {:?}", result.err().unwrap().to_string());
    }
}
