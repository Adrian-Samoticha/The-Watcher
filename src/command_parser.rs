use std::path::Path;
use std::process::{Command, Stdio, Child};
use std::{process, env};

use crate::string_split_iterator::{StringSplitIterator, StringSplit};

pub enum Cmd {
    CommandStruct(Command),
    Cd(Box<Path>),
}

pub struct CmdList {
    cmds: Vec<Cmd>,
}

impl CmdList {
    fn new() -> CmdList {
        CmdList {
            cmds: Vec::new(),
        }
    }

    fn add_cmd(&mut self, cmd: Cmd) {
        self.cmds.push(cmd);
    }
    
    fn add_cd(&mut self, path: Box<Path>) {
        self.cmds.push(Cmd::Cd(path));
    }

    pub fn execute(&mut self) {
        let mut previous_command = None;
        let mut cmds_iterator = self.cmds.iter_mut().peekable();
        while let Some(cmd) = cmds_iterator.next() {
            match cmd {
                Cmd::CommandStruct(ref mut command_struct) => {
                    let output = if cmds_iterator.peek().is_some() {
                        command_struct
                            .stdout(process::Stdio::piped())
                            .spawn()
                    } else {
                        let stdin = previous_command.map_or(
                            Stdio::inherit(),
                            |cmd: Child| Stdio::from(cmd.stdout.unwrap())
                        );
                        
                        command_struct
                            .stdin(stdin)
                            .spawn()
                    };
                    
                    match output {
                        Ok(output) => {
                            previous_command = Some(output);
                        },
                        Err(error) => {
                            eprintln!("Error: {:?}", error);
                            std::process::exit(1);
                        },
                    }
                }
                Cmd::Cd(ref path) => {
                    env::set_current_dir(path).unwrap_or_else(|error| {
                        eprintln!("Error: Failed to change directory to \"{}\". Error: {:?}", path.display(), error);
                        std::process::exit(1);
                    });
                    previous_command = None;
                }
            }
        }
    }
}


/// Receives a command as a string and parses it into a Command struct.
fn parse_command(command: &str) -> Command {
    let mut split_command = StringSplitIterator::new(command, &[' '], &['"', '\''])
        .map(|x| x.string)
        .peekable();
    
    if split_command.peek().is_none() {
        eprintln!("Error: No command given.");
        process::exit(1);
    }
    
    let mut command_struct = Command::new(split_command.next().unwrap_or_else(|| {
        panic!("Error: Failed to parse command.");
    }));
    for arg in split_command {
        command_struct.arg(arg);
    }
    command_struct
}

/// Receives a list of commands separated by pipe characters and parses it into a CmdList.
pub fn parse_piped_command(command: &str) -> CmdList {
    let mut cmd_list = CmdList::new();
    let commands = StringSplitIterator::new(
        command,
        &['|', '&'],
        &['"', '\'']
    ).map(|string_split| StringSplit {
        string: string_split.string.trim(),
        left_delimiter: string_split.left_delimiter,
        right_delimiter: string_split.right_delimiter,
    }).filter(|string_split| !string_split.string.is_empty());
    
    for cmd in commands {
        if cmd.string.starts_with("cd ") {
            let path = {
              StringSplitIterator::new(cmd.string, &[' '], &['"', '\''])
                .collect::<Vec<StringSplit>>()[1]
                .string
            };
            cmd_list.add_cd(Path::new(path).to_owned().into_boxed_path());
            continue;
        }
        
        cmd_list.add_cmd(Cmd::CommandStruct(parse_command(cmd.string)));
    }
    cmd_list
}