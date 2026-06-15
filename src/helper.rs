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

        if line[..start].trim().is_empty() {
            let candidates: Vec<Pair> = self
                .builtins
                .iter()
                .chain(self.executables.iter())
                .filter(|builtin| builtin.starts_with(word))
                .map(|builtin| Pair {
                    display: builtin.clone(),
                    replacement: format!("{} ", builtin.clone()),
                })
                .collect();

            return Ok((start, candidates));
        }

        let (start, candidates) = self.filename_completer.complete(line, pos, ctx)?;
        let candidates: Vec<Pair> = candidates
            .iter()
            .map(|candidate| {
                if candidate.replacement.ends_with('/') {
                    Pair {
                        display: candidate.display.clone(),
                        replacement: candidate.replacement.clone(),
                    }
                } else {
                    Pair {
                        display: candidate.display.clone(),
                        replacement: format!("{} ", candidate.replacement),
                    }
                }
            })
            .collect();

        Ok((start, candidates))
    }
}
