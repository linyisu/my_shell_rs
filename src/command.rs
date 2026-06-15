use crate::builtin::Builtin;
use crate::utils::validate_file;

use std::{
    env, fs, io,
    process::{self, Stdio, exit},
};

#[derive(Debug)]
pub enum CommandType {
    None,
    Normal,
    RedirectOut(String, bool),
    RedirectErr(String, bool),
}

#[derive(Debug)]
pub struct Command {
    pub name: String,
    pub args: Vec<String>,
    pub command_type: CommandType,
}

impl Command {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            name: String::new(),
            command_type: CommandType::None,
        }
    }

    fn tokenize(&mut self, command: String) {
        self.args = vec![String::new()];

        #[derive(Debug)]
        enum State {
            Normal,
            SingleQuote,
            DoubleQuote,
            NormalBackslash,
            DoubleQuoteBackslash,
        }
        let mut state = State::Normal;

        for char in command.chars() {
            match state {
                State::Normal => match char {
                    ' ' => {
                        if self.args.last().is_some_and(|s| !s.is_empty()) {
                            self.args.push(String::new())
                        }
                    }
                    '\\' => state = State::NormalBackslash,
                    '\'' => state = State::SingleQuote,
                    '\"' => state = State::DoubleQuote,
                    _ => self.args.last_mut().unwrap().push(char),
                },
                State::NormalBackslash => {
                    self.args.last_mut().unwrap().push(char);
                    state = State::Normal;
                }
                State::SingleQuote => match char {
                    '\'' => state = State::Normal,
                    _ => self.args.last_mut().unwrap().push(char),
                },
                State::DoubleQuote => match char {
                    '\"' => state = State::Normal,
                    '\\' => state = State::DoubleQuoteBackslash,
                    _ => self.args.last_mut().unwrap().push(char),
                },
                State::DoubleQuoteBackslash => {
                    let ch = match char {
                        '\"' => '\"',
                        '\\' => '\\',
                        _ => char,
                    };

                    self.args.last_mut().unwrap().push(ch);
                    state = State::DoubleQuote;
                }
            }
        }
    }

    pub fn parse(&mut self) {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = String::from(command.trim());

        if command.is_empty() {
            return;
        }
        self.tokenize(command);
        self.name = self.args[0].clone();
        self.args.remove(0);

        self.command_type = CommandType::Normal;
        for arg in &self.args {
            if arg == ">" || arg == "1>" || arg == ">>" || arg == "1>>" {
                self.command_type = CommandType::RedirectOut(
                    self.args.last().unwrap().clone(),
                    arg == ">>" || arg == "1>>",
                );
                self.args = self.args[..self.args.len() - 2].to_vec();
                break;
            } else if arg == "2>" || arg == "2>>" {
                self.command_type =
                    CommandType::RedirectErr(self.args.last().unwrap().clone(), arg == "2>>");
                self.args = self.args[..self.args.len() - 2].to_vec();
                break;
            }
        }
    }

    pub fn run_command(&mut self) -> Result<Option<String>, String> {
        match self.name.as_str() {
            "type" => {
                let arg = &self.args[0];
                if Builtin::from_str(arg).is_some() {
                    Ok(Some(format!("{} is a shell builtin", arg)))
                } else if let Some(path) = validate_file(arg) {
                    Ok(Some(format!("{} is {}", arg, path)))
                } else {
                    Err(format!("{}: not found", arg))
                }
            }
            "exit" => exit(0),
            "echo" => Ok(Some(self.args.join(" "))),
            "pwd" => Ok(Some(
                env::current_dir().unwrap().to_string_lossy().to_string(),
            )),
            "cd" => {
                let arg = &self.args[0];
                let path = if let Some(rest) = arg.strip_prefix("~") {
                    format!("{}{}", env::var("HOME").unwrap_or_default(), rest)
                } else {
                    arg.to_string()
                };

                if env::set_current_dir(path).is_ok() {
                    Ok(None)
                } else {
                    Err(format!("cd: {}: No such file or directory", self.args[0]))
                }
            }
            _ => {
                if validate_file(&self.name).is_some() {
                    let mut cmd = process::Command::new(&self.name);
                    cmd.args(&self.args);

                    if let CommandType::RedirectOut(ref stdout_path, is_append) = self.command_type
                    {
                        let file = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(is_append)
                            .open(stdout_path)
                            .unwrap();
                        cmd.stdout(Stdio::from(file));
                    }
                    if let CommandType::RedirectErr(ref stderr_path, is_append) = self.command_type
                    {
                        let file = fs::OpenOptions::new()
                            .create(true)
                            .write(true)
                            .append(is_append)
                            .open(stderr_path)
                            .unwrap();
                        cmd.stderr(Stdio::from(file));
                    }

                    cmd.status().unwrap();
                    Ok(None)
                } else {
                    Err(format!("{}: command not found", self.name))
                }
            }
        }
    }
}
