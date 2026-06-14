pub enum Builtin {
    Type,
    Exit,
    Echo,
    Pwd,
}

impl Builtin {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "type" => Some(Self::Type),
            "exit" => Some(Self::Exit),
            "echo" => Some(Self::Echo),
            "pwd" => Some(Self::Pwd),
            _ => None,
        }
    }
}
