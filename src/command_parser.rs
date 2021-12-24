use std::path::Path;
use std::process::{Command, Stdio, Child};
use std::{process, env};

pub enum Cmd {
    CommandStruct(Command),
    Cd(Box<Path>),
    Exit,
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
    
    fn add_exit(&mut self) {
        self.cmds.push(Cmd::Exit);
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
                        command_struct
                            .stdin(previous_command.map_or(Stdio::inherit(), |cmd: Child| Stdio::from(cmd.stdout.unwrap())))
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
                    env::set_current_dir(path).unwrap();
                    previous_command = None;
                }
                Cmd::Exit => {
                    process::exit(0);
                }
            }
        }
    }
}


/// Receives a command as a string and parses it into a Command struct.
fn parse_command(command: &str) -> Command {
    let mut command_vec = command.split_whitespace().peekable();
    
    if command_vec.peek().is_none() {
        eprintln!("Error: No command given.");
        process::exit(1);
    }
    
    let mut command_struct = Command::new(command_vec.next().unwrap());
    for arg in command_vec {
        command_struct.arg(arg);
    }
    command_struct
}

/// Receives a list of commands separated by pipe characters and parses it into a PipedCmdList.
pub fn parse_piped_command(command: &str) -> CmdList {
    let mut piped_cmd_list = CmdList::new();
    let commands = command.split("|").map(|x| x.trim());
    
    for cmd in commands {
        if cmd.is_empty() {
            continue;
        }
        
        if cmd.starts_with("cd ") {
            let path = cmd.split(" ").collect::<Vec<&str>>()[1];
            piped_cmd_list.add_cd(Path::new(path).to_owned().into_boxed_path());
            continue;
        }
        
        if cmd.starts_with("exit ") {
            piped_cmd_list.add_exit();
            continue;
        }
        
        piped_cmd_list.add_cmd(Cmd::CommandStruct(parse_command(cmd)));
    }
    piped_cmd_list
}