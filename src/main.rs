mod builtin;
mod command;
use builtin::Builtin;
use command::Command;

use std::io::{self, Write};
use std::os::unix::fs::PermissionsExt;
use std::path::Path;
use std::{env, process};

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

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = Command::new();
        command.parse();

        match command.name.as_str() {
            "type" => {
                let arg = &command.args[0];
                if Builtin::from_str(arg).is_some() {
                    println!("{} is a shell builtin", arg);
                } else if let Some(path) = validate_file(arg) {
                    println!("{} is {}", arg, path);
                } else {
                    println!("{}: not found", arg);
                }
            }
            "exit" => break,
            "echo" => println!("{}", &command.command[5..]),
            "pwd" => println!("{}", env::current_dir().unwrap().to_string_lossy()),
            "cd" => {
                let arg = &command.args[0];
                let path = if let Some(rest) = arg.strip_prefix("~") {
                    format!("{}{}", env::var("HOME").unwrap_or_default(), rest)
                } else {
                    arg.to_string()
                };

                if let Ok(metadata) = Path::new(&path).metadata() {
                    if metadata.is_dir() {
                        env::set_current_dir(path).unwrap();
                    } else {
                        println!("cd: {}: No such file or directory", command.args[0]);
                    }
                } else {
                    println!("cd: {}: No such file or directory", command.args[0]);
                }
            }
            _ => {
                if validate_file(&command.name).is_some() {
                    process::Command::new(&command.name)
                        .args(&command.args)
                        .status()
                        .unwrap();
                } else {
                    println!("{}: command not found", command.command)
                }
            }
        }
    }
}
