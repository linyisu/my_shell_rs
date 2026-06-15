use std::io;

pub struct Command {
    pub name: String,
    pub args: Vec<String>,
}

impl Command {
    pub fn new() -> Self {
        Self {
            args: Vec::new(),
            name: String::new(),
        }
    }

    fn tokenize(&mut self, command: String) {
        self.args = vec![String::new()];

        enum State {
            Normal,
            NormalBackslash,
            SingleQuote,
            DoubleQuote,
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
                    let ch = match char {
                        _ => char,
                    };

                    self.args.last_mut().unwrap().push(ch);
                    state = State::Normal;
                }
                State::SingleQuote => match char {
                    '\'' => state = State::Normal,
                    _ => self.args.last_mut().unwrap().push(char),
                },
                State::DoubleQuote => match char {
                    '\"' => state = State::Normal,
                    _ => self.args.last_mut().unwrap().push(char),
                },
            }
        }
    }

    pub fn parse(&mut self) {
        let mut command = String::new();
        io::stdin().read_line(&mut command).unwrap();
        let command = String::from(command.trim());

        self.args = command.split_whitespace().map(|s| s.to_string()).collect();
        self.name = self.args[0].clone();
        self.args.remove(0);

        let command = command
            .strip_prefix(&self.name)
            .unwrap_or_default()
            .trim()
            .to_string();

        self.tokenize(command);
    }
}
