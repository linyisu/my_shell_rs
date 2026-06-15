use rustyline::Context;
use rustyline::completion::{Completer, Pair};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Hinter, Validator, Highlighter)]
pub struct Helper {
    pub builtins: Vec<String>,
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
        let candidates = self
            .builtins
            .iter()
            .filter(|builtin| builtin.starts_with(word))
            .map(|builtin| Pair {
                display: builtin.clone(),
                replacement: builtin.clone(),
            })
            .collect();

        Ok((start, candidates))
    }
}
