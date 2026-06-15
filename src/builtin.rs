pub enum Builtin {
    Type,
    Exit,
    Echo,
    Pwd,
}

impl Builtin {
    pub const ALL: [Self; 4] = [Self::Type, Self::Exit, Self::Echo, Self::Pwd];

    pub fn name(&self) -> &'static str {
        match self {
            Builtin::Type => "type",
            Builtin::Echo => "echo",
            Builtin::Pwd => "pwd",
            Builtin::Exit => "exit",
        }
    }

    pub fn names() -> Vec<String> {
        Self::ALL
            .iter()
            .map(|builtin| builtin.name().to_string())
            .collect()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|builtin| builtin.name() == s)
    }
}
