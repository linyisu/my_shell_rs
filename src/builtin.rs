#[derive(Clone, Copy)]
pub enum Builtin {
    Pwd,
    Echo,
    Exit,
    Type,
    Complete,
}

impl Builtin {
    pub const ALL: &[Self] = &[
        Self::Type,
        Self::Exit,
        Self::Echo,
        Self::Pwd,
        Self::Complete,
    ];

    pub fn name(&self) -> &'static str {
        match self {
            Builtin::Pwd => "pwd",
            Builtin::Echo => "echo",
            Builtin::Exit => "exit",
            Builtin::Type => "type",
            Builtin::Complete => "complete",
        }
    }

    pub fn names() -> Vec<String> {
        Self::ALL
            .iter()
            .map(|builtin| builtin.name().to_string())
            .collect()
    }

    pub fn from_str(s: &str) -> Option<Self> {
        Self::ALL
            .iter()
            .copied()
            .find(|builtin| builtin.name() == s)
    }
}
