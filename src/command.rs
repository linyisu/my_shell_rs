use std::io;

#[derive(Debug)]
pub enum CommandType {
    None,
    Normal,
    Redirect(String),
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
            if arg == ">" || arg == "1>" {
                self.command_type = CommandType::Redirect(self.args.last().unwrap().clone());
                self.args = self.args[..self.args.len() - 2].to_vec();
                break;
            }
        }
    }
}
