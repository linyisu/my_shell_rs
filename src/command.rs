use std::io;

pub struct Command {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            name: String::new(),
            command: String::new(),
        }
    }

    fn tokenize(&mut self) {
        let mut string = String::new();

        enum State {
            Normal,
            SingleQuote,
        }
        let mut state = State::Normal;

        for char in self.command.chars() {
            match state {
                State::Normal => match char {
                    ' ' if !string.is_empty() && string.ends_with(' ') => {}
                    '\'' => state = State::SingleQuote,
                    _ => string.push(char),
                },
                State::SingleQuote => match char {
                    '\'' => state = State::Normal,
                    _ => string.push(char),
                },
            }
        }

        self.command = string;
        self.args = self
            .command
            .split_whitespace()
            .map(|s| s.to_string())
            .collect();
    }

    pub fn parse(&mut self) {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = String::from(command.trim());

        self.args = command.split_whitespace().map(|s| s.to_string()).collect();
        self.name = self.args[0].clone();
        self.args.remove(0);
        self.command = command
            .strip_prefix(&self.name)
            .unwrap_or_default()
            .trim()
            .to_string();

        self.tokenize();
    }
}
