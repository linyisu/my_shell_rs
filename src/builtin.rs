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

    pub fn is_builtin(s: &str) -> bool {
        Self::ALL.iter().any(|builtin| builtin.name() == s)
    }
}
