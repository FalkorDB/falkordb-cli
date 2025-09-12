# FalkorDB CLI

A comprehensive command-line interface for [FalkorDB](https://www.falkordb.com/), designed to behave similarly to `redis-cli` but with full support for FalkorDB's graph database operations.

## Project Structure

The CLI is organized into modular components for better maintainability:

```
src/
├── main.rs        # Application entry point and main logic
├── cli.rs         # Command-line argument parsing and CLI structure
├── client.rs      # FalkorDB client wrapper and query execution
├── commands.rs    # Command handlers for all CLI operations
├── interactive.rs # Interactive mode implementation
└── tests.rs       # Unit tests for CLI functionality
```

### Module Responsibilities

- **`cli.rs`**: Defines the CLI structure using clap, including all commands and arguments
- **`client.rs`**: Handles FalkorDB connection, query execution, and result formatting
- **`commands.rs`**: Implements command handlers for all FalkorDB operations
- **`interactive.rs`**: Provides the interactive shell experience with readline support
- **`tests.rs`**: Comprehensive unit tests covering CLI parsing and command validation

## Features

- **Interactive Mode**: Redis-cli like interactive shell for executing Cypher queries
- **Batch Operations**: Execute single commands or read from files
- **Multiple Output Formats**: Support for table, JSON, and CSV output formats
- **Graph Management**: Create, delete, and manage multiple graphs
- **Schema Inspection**: View node labels and relationship types
- **Index Management**: Create and drop indices on nodes and relationships
- **Performance Analysis**: Profile and explain query execution plans
- **Slowlog Analysis**: Monitor and analyze slow queries

## Installation

### Build from Source

```bash
git clone <repository-url>
cd falkordb-cli
cargo build --release
```

The binary will be available at `./target/release/falkordb-cli`.

## Usage

### Basic Connection

```bash
# Connect to local FalkorDB instance
falkordb-cli

# Connect to specific host and port
falkordb-cli --hostname myhost -p 6379

# Connect with authentication
falkordb-cli -a mypassword

# Connect to specific database
falkordb-cli -n 1
```

### Interactive Mode

```bash
# Start interactive mode
falkordb-cli interactive

# Start with a specific graph selected
falkordb-cli -g mygraph interactive
```

#### Interactive Commands

- `USE <graph_name>` - Switch to a specific graph
- `LIST` - List all available graphs
- `SCHEMA` - Show schema for current graph
- `SCHEMA <graph>` - Show schema for specific graph
- `HELP` - Show available commands
- `EXIT` or `QUIT` - Exit interactive mode

Any Cypher query can be executed directly:
```cypher
CREATE (n:Person {name: 'Alice', age: 30})
MATCH (n:Person) RETURN n.name, n.age
MATCH (a)-[r]->(b) RETURN a, r, b LIMIT 10
```

### Command-Line Operations

#### Execute Queries

```bash
# Execute a single query
falkordb-cli query mygraph "MATCH (n:Person) RETURN n.name"

# Execute a read-only query
falkordb-cli ro-query mygraph "MATCH (n) RETURN count(n)"

# Execute with different output formats
falkordb-cli --format json query mygraph "MATCH (n:Person) RETURN n"
falkordb-cli --format csv query mygraph "MATCH (n:Person) RETURN n.name, n.age"
```

#### Eval Mode

```bash
# Execute a command and exit
falkordb-cli -g mygraph --eval "CREATE (n:Person {name: 'Bob'})"
```

#### Performance Analysis

```bash
# Profile a query
falkordb-cli profile mygraph "MATCH (n:Person)-[r:KNOWS]->(m:Person) RETURN n, m"

# Explain query execution plan
falkordb-cli explain mygraph "MATCH (n:Person) WHERE n.age > 25 RETURN n"
```

#### Index Management

```bash
# Create an index on a node property
falkordb-cli create-index mygraph NODE Person name

# Create an index on a relationship property
falkordb-cli create-index mygraph EDGE KNOWS since

# Drop an index
falkordb-cli drop-index mygraph NODE Person name

# List all indices
falkordb-cli indices mygraph
```

#### Graph Management

```bash
# List all graphs
falkordb-cli list

# Show graph schema
falkordb-cli schema mygraph

# Delete a graph
falkordb-cli delete mygraph
```

#### Monitoring

```bash
# Show slowlog
falkordb-cli slowlog mygraph

# Reset slowlog
falkordb-cli slowlog-reset mygraph
```

### Output Formats

#### Table Format (Default)
```
| name            | age             |
|-----------------|-----------------|
| Alice           | 30              |
| Bob             | 25              |
```

#### JSON Format
```bash
falkordb-cli --format json query mygraph "MATCH (n:Person) RETURN n.name, n.age"
```

#### CSV Format
```bash
falkordb-cli --format csv query mygraph "MATCH (n:Person) RETURN n.name, n.age"
```

## Supported FalkorDB Commands

This CLI supports all major FalkorDB operations:

- **Graph Operations**: `GRAPH.QUERY`, `GRAPH.RO_QUERY`, `GRAPH.DELETE`
- **Performance**: `GRAPH.PROFILE`, `GRAPH.EXPLAIN`, `GRAPH.SLOWLOG`
- **Schema**: `GRAPH.SCHEMA`
- **Indices**: Index creation and management
- **Procedures**: Built-in procedure calls

## Environment Variables

- `FALKORDB_HOST` - Default hostname (overridden by `-h`)
- `FALKORDB_PORT` - Default port (overridden by `-p`)
- `FALKORDB_PASSWORD` - Default password (overridden by `-a`)

## Examples

### Basic Graph Operations

```bash
# Create a simple graph
falkordb-cli -g social --eval "CREATE (alice:Person {name: 'Alice', age: 30})"
falkordb-cli -g social --eval "CREATE (bob:Person {name: 'Bob', age: 25})"
falkordb-cli -g social --eval "MATCH (a:Person {name: 'Alice'}), (b:Person {name: 'Bob'}) CREATE (a)-[:KNOWS {since: 2020}]->(b)"

# Query the graph
falkordb-cli query social "MATCH (n:Person) RETURN n.name, n.age ORDER BY n.age"
falkordb-cli query social "MATCH (a:Person)-[r:KNOWS]->(b:Person) RETURN a.name, r.since, b.name"
```

### Performance Analysis

```bash
# Profile a complex query
falkordb-cli profile social "MATCH (a:Person)-[:KNOWS*2..3]->(b:Person) RETURN a.name, b.name"

# Explain query execution
falkordb-cli explain social "MATCH (n:Person) WHERE n.age > 25 RETURN n"
```

### Index Management

```bash
# Create indices for better performance
falkordb-cli create-index social NODE Person name
falkordb-cli create-index social NODE Person age
falkordb-cli create-index social EDGE KNOWS since

# Verify indices
falkordb-cli indices social
```

## Limitations

- File input mode (`-f`) is not yet implemented
- Procedure calls with parameters need manual query construction
- Result set iteration is limited due to the current falkordb-rs API design

## Contributing

This CLI is built using the [falkordb-rs](https://github.com/FalkorDB/falkordb-rs) Rust client library. Contributions are welcome!

## License

MIT License - see LICENSE file for details.
