mod builtin;
mod command;
use builtin::Builtin;
use command::Command;

use std::{
    env, fs,
    io::{self, Write},
    os::unix::fs::PermissionsExt,
    path::Path,
    process::{self, Stdio, exit},
    sync::{Arc, Mutex},
};

use crate::command::CommandType;

fn validate_file(name: &str) -> Option<String> {
    let path = env::var("PATH").unwrap_or_default();
    for dir in path.split(':') {
        let full_path = Path::new(dir).join(name);
        if let Ok(meta) = full_path.metadata() {
            if meta.is_file() && meta.permissions().mode() & 0o111 != 0 {
                return Some(full_path.to_string_lossy().into_owned());
            }
        }
    }
    None
}

fn run_command(command: &Command) -> Result<Option<String>, String> {
    match command.name.as_str() {
        "type" => {
            let arg = &command.args[0];
            if Builtin::from_str(arg).is_some() {
                Ok(Some(format!("{} is a shell builtin", arg)))
            } else if let Some(path) = validate_file(arg) {
                Ok(Some(format!("{} is {}", arg, path)))
            } else {
                Err(format!("{}: not found", arg))
            }
        }
        "exit" => exit(0),
        "echo" => Ok(Some(command.args.join(" "))),
        "pwd" => Ok(Some(
            env::current_dir().unwrap().to_string_lossy().to_string(),
        )),
        "cd" => {
            let arg = &command.args[0];
            let path = if let Some(rest) = arg.strip_prefix("~") {
                format!("{}{}", env::var("HOME").unwrap_or_default(), rest)
            } else {
                arg.to_string()
            };

            if env::set_current_dir(path).is_ok() {
                Ok(None)
            } else {
                Err(format!(
                    "cd: {}: No such file or directory",
                    command.args[0]
                ))
            }
        }
        _ => {
            if validate_file(&command.name).is_some() {
                let mut cmd = process::Command::new(&command.name);
                cmd.args(&command.args);

                if let CommandType::RedirectOut(ref stdout_path) = command.command_type {
                    let file = fs::File::create(stdout_path).unwrap();
                    cmd.stdout(Stdio::from(file));
                }
                if let CommandType::RedirectErr(ref stderr_path) = command.command_type {
                    let file = fs::File::create(stderr_path).unwrap();
                    cmd.stderr(Stdio::from(file));
                }

                cmd.status().unwrap();
                Ok(None)
            } else {
                Err(format!("{}: command not found", command.name))
            }
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = Command::new();
        command.parse();

        match command.command_type {
            CommandType::None => continue,
            CommandType::Normal => match run_command(&command) {
                Ok(Some(output)) => {
                    println!("{}", output);
                }
                Ok(None) => {}
                Err(error) => {
                    println!("{}", error);
                }
            },
            CommandType::RedirectOut(ref stdout_path) => match run_command(&command) {
                Ok(Some(output)) => {
                    let mut file = fs::File::create(stdout_path).unwrap();
                    writeln!(file, "{}", output).unwrap();
                }
                _ => {}
            },
            CommandType::RedirectErr(ref stderr_path) => match run_command(&command) {
                Err(output) => {
                    let mut file = fs::File::create(stderr_path).unwrap();
                    writeln!(file, "{}", output).unwrap();
                }
                _ => {}
            },
        }
    }
}
