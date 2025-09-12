use anyhow::Result;
use clap::Parser;

mod cli;
mod client;
mod commands;
mod interactive;

#[cfg(test)]
mod tests;

use cli::Cli;
use client::FalkorCli;

fn main() -> Result<()> {
    let cli = Cli::parse();

    let mut falkor_cli = FalkorCli::new(
        &cli.hostname,
        cli.port,
        cli.database,
        cli.auth.as_deref(),
        cli.format,
        cli.quiet,
        cli.raw,
    )?;

    if let Some(graph) = cli.graph {
        falkor_cli.set_graph(graph);
    }

    // Handle eval mode
    if let Some(command) = cli.eval {
        let graph_name = falkor_cli.get_graph_name(None)?;
        return falkor_cli.execute_query(&graph_name, &command, false);
    }

    // Handle file mode
    if let Some(_file_path) = cli.file {
        return Err(anyhow::anyhow!("File mode not yet implemented"));
    }

    // Handle subcommands
    match cli.command {
        Some(command) => falkor_cli.handle_command(command),
        None => falkor_cli.interactive_mode(),
    }
}