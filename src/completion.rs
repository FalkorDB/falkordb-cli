use crate::cli::Cli;
use clap::CommandFactory;
use rustyline::completion::{Completer, Pair};
use rustyline::highlight::Highlighter;
use rustyline::hint::Hinter;
use rustyline::validate::Validator;
use rustyline::Context;
use rustyline::Helper;

#[derive(Clone)]
pub struct SimpleCompleter {
    keywords: Vec<String>,
}

impl SimpleCompleter {
    pub fn new() -> Self {
        // Derive keywords from the Clap `Cli` subcommands so the completer stays
        // in sync with the `Commands` enum. Clap provides the subcommand names
        // (kebab-case); convert to uppercase for REPL matching.
        let mut keywords: Vec<String> = Vec::new();
        let cmd = Cli::command();
        for sc in cmd.get_subcommands() {
            keywords.push(sc.get_name().to_uppercase());
        }

        // Add REPL-only tokens that are not subcommands.
        let extras = [
            "HELP",
            "USE",
            "EXIT",
            "QUIT",
            "MATCH",
            "INTERACTIVE",
            "QUERY",
            "RO-QUERY",
        ];
        for e in extras {
            if !keywords.contains(&e.to_string()) {
                keywords.push(e.to_string());
            }
        }

        keywords.sort();
        keywords.dedup();
        Self { keywords }
    }
}

impl Completer for SimpleCompleter {
    type Candidate = Pair;

    fn complete(
        &self,
        line: &str,
        pos: usize,
        _ctx: &Context<'_>,
    ) -> rustyline::Result<(usize, Vec<Pair>)> {
        let start = line[..pos].rfind(' ').map(|i| i + 1).unwrap_or(0);
        let prefix = &line[start..pos].to_uppercase();

        let matches: Vec<Pair> = self
            .keywords
            .iter()
            .filter(|k| k.starts_with(prefix))
            .map(|k| Pair {
                display: k.clone(),
                replacement: k.clone(),
            })
            .collect();

        Ok((start, matches))
    }
}

impl Helper for SimpleCompleter {}
impl Hinter for SimpleCompleter {
    type Hint = String;
}
impl Highlighter for SimpleCompleter {}
impl Validator for SimpleCompleter {}
