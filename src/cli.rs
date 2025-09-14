use clap::{Parser, Subcommand};

/// `FalkorDB` Command Line Interface
#[derive(Parser)]
#[command(
    name = "falkordb-cli",
    version = "0.1.0",
    about = "A command-line interface for FalkorDB",
    long_about = "A Redis-cli like interface for FalkorDB graph database operations.\n\nAuthentication:\n  Use -u for username and -a for password.\n  Supports: no auth, password only, or username:password combinations."
)]
pub struct Cli {
    /// `FalkorDB` server hostname
    #[arg(long, default_value = "localhost")]
    pub hostname: String,

    /// `FalkorDB` server port
    #[arg(short = 'p', long, default_value = "6379")]
    pub port: u16,

    /// Database number
    #[arg(short = 'n', long, default_value = "0")]
    pub database: u8,

    /// Username for authentication
    #[arg(short = 'u', long)]
    pub username: Option<String>,

    /// Connection password
    #[arg(short = 'a', long)]
    pub auth: Option<String>,

    /// Graph name to operate on
    #[arg(short = 'g', long)]
    pub graph: Option<String>,

    /// Execute command and exit
    #[arg(long)]
    pub eval: Option<String>,

    /// Read commands from file
    #[arg(short = 'f', long)]
    pub file: Option<String>,

    /// Output format (json, table, csv)
    #[arg(long, default_value = "table")]
    pub format: String,

    /// Quiet mode - suppress non-essential output
    #[arg(short = 'q', long)]
    pub quiet: bool,

    /// Raw output mode
    #[arg(short = 'r', long)]
    pub raw: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Execute a Cypher query on a graph
    Query {
        /// Graph name
        graph: String,
        /// Cypher query
        query: String,
        /// Query parameters in JSON format
        #[arg(short = 'p', long)]
        params: Option<String>,
    },
    /// Execute a read-only Cypher query
    RoQuery {
        /// Graph name
        graph: String,
        /// Cypher query
        query: String,
        /// Query parameters in JSON format
        #[arg(short = 'p', long)]
        params: Option<String>,
    },
    /// Profile a query execution
    Profile {
        /// Graph name
        graph: String,
        /// Cypher query
        query: String,
    },
    /// Explain query execution plan
    Explain {
        /// Graph name
        graph: String,
        /// Cypher query
        query: String,
    },
    /// Delete a graph
    Delete {
        /// Graph name
        graph: String,
    },
    /// List all graphs
    List,
    /// Show graph schema
    Schema {
        /// Graph name
        graph: String,
    },
    /// Show slowlog
    Slowlog {
        /// Graph name
        graph: String,
    },
    /// Reset slowlog
    SlowlogReset {
        /// Graph name
        graph: String,
    },
    /// Show indices
    Indices {
        /// Graph name
        graph: String,
    },
    /// Create index
    CreateIndex {
        /// Graph name
        graph: String,
        /// Entity type (NODE or EDGE)
        entity_type: String,
        /// Label
        label: String,
        /// Property
        property: String,
    },
    /// Drop index
    DropIndex {
        /// Graph name
        graph: String,
        /// Entity type (NODE or EDGE)
        entity_type: String,
        /// Label
        label: String,
        /// Property
        property: String,
    },
    /// Call a procedure
    Call {
        /// Graph name
        graph: String,
        /// Procedure name
        procedure: String,
        /// Arguments in JSON format
        #[arg(short = 'a', long)]
        args: Option<String>,
    },
    /// Interactive mode
    Interactive,
}
