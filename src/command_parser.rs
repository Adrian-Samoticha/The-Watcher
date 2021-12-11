use std::process::Command;
use std::process;

/// Receives a command as a string and parses it into a Command struct.
pub fn parse_command(command: &str) -> Command {
    let mut command_vec: Vec<&str> = command.split_whitespace().collect();
    
    if command_vec.is_empty() {
        eprintln!("Error: No command given.");
        process::exit(1);
    }
    
    let mut command_struct = Command::new(command_vec.remove(0));
    for arg in command_vec {
        command_struct.arg(arg);
    }
    command_struct
}