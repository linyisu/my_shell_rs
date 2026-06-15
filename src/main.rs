mod builtin;
mod command;
mod helper;
mod utils;

use builtin::Builtin;
use command::{Command, CommandType};
use helper::Helper;

use rustyline::{Editor, error::ReadlineError};
use std::{env, fs, io::Write, process::exit};

fn main() -> anyhow::Result<()> {
    let helper = Helper {
        builtins: Builtin::names(),
        executables: env::var("PATH")
            .unwrap_or_default()
            .split(':')
            .map(|p| p.to_string())
            .collect(),
    };

    let mut reader = Editor::new()?;
    reader.set_helper(Some(helper));

    loop {
        match reader.readline("$ ") {
            Ok(line) => {
                let mut command = Command::new();
                command.parse(line);

                match command.command_type {
                    CommandType::None => continue,
                    CommandType::Normal => match command.run_command() {
                        Ok(Some(output)) => {
                            println!("{}", output);
                        }
                        Ok(None) => {}
                        Err(error) => {
                            println!("{}", error);
                        }
                    },
                    CommandType::RedirectOut(ref stdout_path, is_append) => {
                        let mut file = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(is_append)
                            .open(stdout_path)
                            .unwrap();

                        match command.run_command() {
                            Ok(Some(output)) => {
                                writeln!(file, "{}", output).unwrap();
                            }
                            Ok(None) => {}
                            Err(error) => {
                                println!("{}", error);
                            }
                        }
                    }
                    CommandType::RedirectErr(ref stderr_path, is_append) => {
                        let mut file = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(is_append)
                            .open(stderr_path)
                            .unwrap();

                        match command.run_command() {
                            Ok(Some(output)) => {
                                println!("{}", output);
                            }
                            Ok(None) => {}
                            Err(output) => {
                                writeln!(file, "{}", output).unwrap();
                            }
                        }
                    }
                }
            }
            Err(ReadlineError::Interrupted) => {
                exit(0);
            }
            Err(error) => {
                println!("{}", error);
            }
        }
    }
}
