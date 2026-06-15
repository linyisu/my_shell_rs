use rustyline::Context;
use rustyline::completion::{Completer, Pair};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Hinter, Validator, Highlighter)]
pub struct Helper {
    pub builtins: Vec<String>,
    pub executables: Vec<String>,
}

impl Completer for Helper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word = &line[start..pos];

        let merged = self
            .builtins
            .iter()
            .chain(self.executables.iter())
            .cloned()
            .collect::<Vec<_>>();
        println!("{:?}", self.executables);
        println!("{:?}", merged);
        let candidates = merged
            .iter()
            .filter(|builtin| builtin.starts_with(word))
            .map(|builtin| Pair {
                display: builtin.clone(),
                replacement: format!("{} ", builtin.clone()),
            })
            .collect();

        Ok((start, candidates))
    }
}
