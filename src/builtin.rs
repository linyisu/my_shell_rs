pub enum Builtin {
    Type,
    Exit,
    Echo,
}

impl Builtin {
    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            "type" => Some(Self::Type),
            "exit" => Some(Self::Exit),
            "echo" => Some(Self::Echo),
            _ => None,
        }
    }
}
