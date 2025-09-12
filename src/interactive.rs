use anyhow::{Context, Result};
use colored::*;
use rustyline::{error::ReadlineError, DefaultEditor};

use crate::client::FalkorCli;

impl FalkorCli {
    pub fn interactive_mode(&mut self) -> Result<()> {
        let mut rl = DefaultEditor::new().context("Failed to create readline editor")?;
        
        println!("{}", "FalkorDB CLI - Interactive Mode".green().bold());
        println!("Type 'help' for commands, 'exit' to quit");
        
        if let Some(ref graph) = self.current_graph {
            println!("Current graph: {}", graph.yellow());
        }
        
        loop {
            let prompt = match &self.current_graph {
                Some(graph) => format!("{}> ", graph),
                None => "falkordb> ".to_string(),
            };
            
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
                    eprintln!("Error: {:?}", err);
                    break;
                }
            }
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
        
        match parts.get(0).map(|s| s.to_uppercase()).as_deref() {
            Some("HELP") => {
                self.show_help();
                Ok(())
            }
            Some("USE") => {
                if let Some(graph_name) = parts.get(1) {
                    self.set_graph(graph_name.to_string());
                    println!("Switched to graph: {}", graph_name.yellow());
                } else {
                    println!("Usage: USE <graph_name>");
                }
                Ok(())
            }
            Some("LIST") => self.list_graphs(),
            Some("SCHEMA") => {
                let graph_name = if let Some(name) = parts.get(1) {
                    name.to_string()
                } else {
                    self.get_graph_name(None)?
                };
                self.show_schema(&graph_name)
            }
            Some("EXIT") | Some("QUIT") => Ok(()),
            _ => {
                // Treat as Cypher query if we have a current graph
                if let Some(ref graph_name) = self.current_graph.clone() {
                    self.execute_query(graph_name, line, false)
                } else {
                    Err(anyhow::anyhow!("No graph selected. Use 'USE <graph_name>' first or specify graph name"))
                }
            }
        }
    }

    fn show_help(&self) {
        println!("{}", "FalkorDB CLI Commands:".green().bold());
        println!("  {}            - Execute Cypher query on current graph", "MATCH (n) RETURN n".cyan());
        println!("  {}       - Switch to specified graph", "USE <graph_name>".cyan());
        println!("  {}              - List all graphs", "LIST".cyan());
        println!("  {}            - Show current graph schema", "SCHEMA".cyan());
        println!("  {}   - Show schema for specific graph", "SCHEMA <graph>".cyan());
        println!("  {}              - Show this help", "HELP".cyan());
        println!("  {}        - Exit interactive mode", "EXIT/QUIT".cyan());
        println!();
        println!("{}", "Query Examples:".yellow().bold());
        println!("  CREATE (n:Person {{name: 'John'}})");
        println!("  MATCH (n:Person) RETURN n.name");
        println!("  MATCH (a)-[r]->(b) RETURN a, r, b LIMIT 10");
    }
}
