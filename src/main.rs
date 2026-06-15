mod builtin;
mod command;
mod utils;

use command::{Command, CommandType};

use std::{
    fs,
    io::{self, Write},
};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = Command::new();
        command.parse();

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
}
