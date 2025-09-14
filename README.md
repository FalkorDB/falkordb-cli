# FalkorDB CLI

A lightweight command-line interface for FalkorDB. This README is structured for three audiences:

- Getting started (users) — quick install and basic usage
- Functionality & API — what commands and formats are supported
- Developer — how to build, test and contribute

## Getting started

### Download prebuilt binaries

Prebuilt binaries are published on GitHub Releases: https://github.com/FalkorDB/falkordb-cli/releases

Quick download with the GitHub CLI:

```bash
# list releases
gh release list

# download assets for a release tag (example: v0.1.0)
gh release download v0.1.0 --pattern "*${OS}*" --dir ./downloads
```

Replace `${OS}` with a pattern such as `x86_64-unknown-linux-gnu`, `x86_64-unknown-linux-musl`, `x86_64-apple-darwin` or `x86_64-pc-windows-msvc` depending on your platform.

Verify downloads using the provided `COMBINED_SHA256SUMS.txt` or individual `SHA256SUMS.txt` files attached to the Release.

### Build from source (quick)

```bash
git clone <repository-url>
cd falkordb-cli
cargo build --release
```

The binary will be available at `./target/release/falkordb-cli`.

### First run

```bash
# show help
falkordb-cli --help

# connect to local FalkorDB
falkordb-cli
```

## Functionality & API

The CLI supports interactive and non-interactive operation, multiple output formats, and common graph management and inspection commands.

### Features

- Interactive Mode: Redis-cli-like interactive shell for executing Cypher queries
- Batch Operations: Execute commands or read from files
- Multiple Output Formats: table (default), JSON, CSV
- Graph Management: create, delete and list graphs
- Schema Inspection: view node labels and relationship types
- Index Management: create, drop and list indices
- Performance Analysis: profile and explain query execution plans
- Slowlog Analysis: monitor and analyze slow queries

### Usage examples

Basic connection:

```bash
falkordb-cli                    # connect to local FalkorDB
falkordb-cli --hostname host -p 6379
falkordb-cli -a mypassword
```

Run a query non-interactively:

```bash
falkordb-cli query mygraph "MATCH (n:Person) RETURN n.name, n.age"
```

Interactive mode:

```bash
falkordb-cli interactive
USE mygraph
MATCH (n:Person) RETURN n.name
```

Output formats:

```bash
falkordb-cli --format json query mygraph "MATCH (n) RETURN n"
falkordb-cli --format csv query mygraph "MATCH (n) RETURN n.name, n.age"
```

Common commands:

- `create-index <graph> NODE <Label> <prop>`
- `drop-index <graph> NODE <Label> <prop>`
- `schema <graph>`
- `list` — list graphs
- `slowlog <graph>` — view slow queries

## Developer

This section is for contributors and maintainers.

### Project structure

```
src/
├── main.rs        # Application entry point and main logic
├── cli.rs         # Command-line argument parsing and CLI structure
├── client.rs      # FalkorDB client wrapper and query execution
├── commands.rs    # Command handlers for all CLI operations
├── interactive.rs # Interactive mode implementation
└── tests.rs       # Unit tests for CLI functionality
```

### Build & test

Install dependencies and build:

```bash
cargo build --release
```

Run tests:

```bash
cargo test
```

### Releases

This repository includes a GitHub Actions workflow that builds release binaries for multiple platforms when a Release is published. The workflow packages artifacts, aggregates per-job artifacts, computes checksums and uploads them to the Release that triggered the workflow.

If you'd like a more advanced pipeline (Homebrew formula, automated changelogs, signed artifacts), consider adding a `.goreleaser.yml` and invoking goreleaser from CI.

### Contributing

Contributions are welcome. Please open issues and pull requests. Follow the project's coding style and run tests locally before submitting.

### Limitations

- File input mode (`-f`) is not yet implemented
- Procedure calls with parameters need manual query construction
- Result set iteration is limited due to the current falkordb-rs API design

## License

MIT License - see LICENSE file for details.
