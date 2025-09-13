use anyhow::{Context, Result};
use colored::Colorize;
use rustyline::history::DefaultHistory;
use rustyline::{error::ReadlineError, Editor};
use std::env;
use std::path::PathBuf;

use crate::client::FalkorCli;
use crate::completion::SimpleCompleter;

impl FalkorCli {
    pub fn interactive_mode(&mut self) -> Result<()> {
        let mut rl: Editor<SimpleCompleter, DefaultHistory> =
            Editor::new().context("Failed to create readline editor")?;
        rl.set_helper(Some(SimpleCompleter::new()));

        // Determine history file path (~/.falkordb-cli_history)
        let history_path: PathBuf = env::var("HOME").map_or_else(
            |_| {
                env::var("USERPROFILE").map_or_else(
                    |_| PathBuf::from(".falkordb-cli_history"),
                    |up| PathBuf::from(up).join(".falkordb-cli_history"),
                )
            },
            |home| PathBuf::from(home).join(".falkordb-cli_history"),
        );
        let history_path_str = history_path.to_str().unwrap_or(".falkordb-cli_history");

        // Try to load history if present (best-effort)
        let _ = rl.load_history(history_path_str);

        println!("{}", "FalkorDB CLI - Interactive Mode".green().bold());
        println!("Type 'help' for commands, 'exit' to quit");

        if let Some(ref graph) = self.current_graph {
            println!("Current graph: {}", graph.yellow());
        }

        loop {
            let prompt = self.current_graph.as_ref().map_or_else(
                || "falkordb> ".to_string(),
                |graph| format!("{}> ", graph.yellow())
            );

            let readline = rl.readline(&prompt);
            match readline {
                Ok(line) => {
                    let _ = rl.add_history_entry(line.as_str());

                    if let Err(e) = self.handle_interactive_command(&line) {
                        eprintln!("{}: {}", "Error".red(), e);
                    }

                    if line.trim() == "exit" || line.trim() == "quit" {
                        break;
                    }
                }
                Err(ReadlineError::Interrupted) => {
                    println!("CTRL-C");
                    break;
                }
                Err(ReadlineError::Eof) => {
                    println!("CTRL-D");
                    break;
                }
                Err(err) => {
                    eprintln!("Error: {err:?}");
                    break;
                }
            }
        }

        // Save history on exit (best-effort)
        if let Err(e) = rl.save_history(history_path_str) {
            eprintln!("Failed to save history '{history_path_str}': {e:?}");
        }

        Ok(())
    }

    fn handle_interactive_command(&mut self, line: &str) -> Result<()> {
        let line = line.trim();

        if line.is_empty() {
            return Ok(());
        }

        // Parse interactive commands
        let parts: Vec<&str> = line.split_whitespace().collect();

        match parts.first().map(|s| s.to_uppercase()).as_deref() {
            Some("HELP") => {
                Self::show_help();
                Ok(())
            }
            Some("USE") => {
                if let Some(graph_name) = parts.get(1) {
                    self.set_graph((*graph_name).to_string());
                    println!("Switched to graph: {}", graph_name.yellow());
                } else {
                    println!("Usage: USE <graph_name>");
                }
                Ok(())
            }
            Some("LIST") => Self::list_graphs(),
            Some("SCHEMA") => {
                let graph_name = if let Some(name) = parts.get(1) {
                    (*name).to_string()
                } else {
                    self.get_graph_name(None)?
                };
                self.show_schema(&graph_name)
            }
            Some("EXIT" | "QUIT") => Ok(()),
            Some("QUERY") => {
                // Extract the query text after the leading 'QUERY' token (split on any whitespace)
                let raw = line
                    .split_once(|c: char| c.is_whitespace())
                    .map_or("", |(_, s)| s.trim());
                // Strip matching surrounding quotes if present
                let query_text = if (raw.starts_with('"') && raw.ends_with('"'))
                    || (raw.starts_with('\'') && raw.ends_with('\''))
                {
                    &raw[1..raw.len() - 1]
                } else {
                    raw
                };

                if query_text.trim().is_empty() {
                    println!("Usage: QUERY <cypher_query>");
                    return Ok(());
                }

                self.current_graph.clone().as_ref().map_or_else(
                    || Err(anyhow::anyhow!(
                        "No graph selected. Use 'USE <graph_name>' first or specify graph name"
                    )),
                    |graph_name| self.execute_query(graph_name, query_text.trim(), false)
                )
            }
            Some("RO-QUERY") => {
                // Extract the query text after the leading 'RO-QUERY' token (split on any whitespace)
                let raw = line
                    .split_once(|c: char| c.is_whitespace())
                    .map_or("", |(_, s)| s.trim());
                // Strip matching surrounding quotes if present
                let query_text = if (raw.starts_with('"') && raw.ends_with('"'))
                    || (raw.starts_with('\'') && raw.ends_with('\''))
                {
                    &raw[1..raw.len() - 1]
                } else {
                    raw
                };

                if query_text.trim().is_empty() {
                    println!("Usage: RO-QUERY <cypher_query>");
                    return Ok(());
                }

                self.current_graph.clone().as_ref().map_or_else(
                    || Err(anyhow::anyhow!(
                        "No graph selected. Use 'USE <graph_name>' first or specify graph name"
                    )),
                    |graph_name| self.execute_query(graph_name, query_text.trim(), true)
                )
            }
            _ => {
                // Treat as Cypher query if we have a current graph
                self.current_graph.clone().as_ref().map_or_else(
                    || Err(anyhow::anyhow!(
                        "No graph selected. Use 'USE <graph_name>' first or specify graph name"
                    )),
                    |graph_name| self.execute_query(graph_name, line, false)
                )
            }
        }
    }

    fn show_help() {
        println!("{}", "FalkorDB CLI Commands:".green().bold());
        println!(
            "  {}            - Execute Cypher query on current graph",
            "MATCH (n) RETURN n".cyan()
        );
        println!(
            "  {}       - Switch to specified graph",
            "USE <graph_name>".cyan()
        );
        println!("  {}              - List all graphs", "LIST".cyan());
        println!(
            "  {}            - Show current graph schema",
            "SCHEMA".cyan()
        );
        println!(
            "  {}   - Show schema for specific graph",
            "SCHEMA <graph>".cyan()
        );
        println!("  {}              - Show this help", "HELP".cyan());
        println!("  {}        - Exit interactive mode", "EXIT/QUIT".cyan());
        println!();
        println!("{}", "Query Examples:".yellow().bold());
        println!("  CREATE (n:Person {{name: 'John'}})");
        println!("  MATCH (n:Person) RETURN n.name");
        println!("  MATCH (a)-[r]->(b) RETURN a, r, b LIMIT 10");
    }
}
