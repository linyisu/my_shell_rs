use rustyline::Context;
use rustyline::completion::{Completer, FilenameCompleter, Pair};
use rustyline_derive::{Helper, Highlighter, Hinter, Validator};

#[derive(Helper, Hinter, Validator, Highlighter)]
pub struct Helper {
    pub builtins: Vec<String>,
    pub executables: Vec<String>,
    pub filename_completer: FilenameCompleter,
}

impl Completer for Helper {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let word = &line[start..pos];

        let merged = self
            .builtins
            .iter()
            .chain(self.executables.iter())
            .cloned()
            .collect::<Vec<_>>();
        let candidates: Vec<Pair> = merged
            .iter()
            .filter(|builtin| builtin.starts_with(word))
            .map(|builtin| Pair {
                display: builtin.clone(),
                replacement: format!("{} ", builtin.clone()),
            })
            .collect();

        if !candidates.is_empty() {
            return Ok((start, candidates));
        }

        self.filename_completer.complete(line, pos, ctx)
    }
}
