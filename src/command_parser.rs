use std::process::Command;

/// Receives a command as a string and parses it into a Command struct.
pub fn parse_command(command: &str) -> Command {
    let mut command_vec: Vec<&str> = command.split_whitespace().collect();
    let mut command_struct = Command::new(command_vec.remove(0));
    for arg in command_vec {
        command_struct.arg(arg);
    }
    command_struct
}