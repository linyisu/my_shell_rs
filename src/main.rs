#[allow(unused_imports)]
use std::io::{self, Write};

struct Command {
    name: String,
    command: String,
    args: Vec<String>,
}

impl Command {
    fn new() -> Self {
        Self {
            args: Vec::new(),
            name: String::new(),
            command: String::new(),
        }
    }

    fn parse(&mut self) {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        self.command = String::from(command.trim());
        self.args = self
            .command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
        self.name = self.args[0].clone();
        self.args.remove(0);
    }
}

enum Builtin {
    Type,
    Exit,
    Echo,
}

impl Builtin {
    fn from_str(s: &str) -> Option<Self> {
        match s {
            "type" => Some(Self::Type),
            "exit" => Some(Self::Exit),
            "echo" => Some(Self::Echo),
            _ => None,
        }
    }
}

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = Command::new();
        command.parse();

        match command.name.as_str() {
            "type" => {
                if let Some(_) = Builtin::from_str(command.args[0].as_str()) {
                    println!("{} is a shell builtin", command.args[0]);
                } else {
                    println!("{}: command not found", command.args[0]);
                }
            }
            "exit" => break,
            "echo" => println!("{}", &command.command[5..]),
            _ => println!("{}: command not found", command.command),
        }
    }
}
