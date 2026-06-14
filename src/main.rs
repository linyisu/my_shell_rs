#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();

        let command_args: Vec<String> = command
            .trim()
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();

        match command_args[0].as_str() {
            "exit" => break,
            "echo" => println!("{}", &command.trim()[5..]),
            _ => println!("{}: command not found", command.trim()),
        }
    }
}
