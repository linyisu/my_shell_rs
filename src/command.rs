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

    pub fn parse(&mut self) {
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
