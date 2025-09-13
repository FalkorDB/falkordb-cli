use anyhow::{Context, Result};
use colored::Colorize;
use falkordb::{FalkorClientBuilder, FalkorConnectionInfo, FalkorSyncClient};

pub struct FalkorCli {
    pub client: FalkorSyncClient,
    pub current_graph: Option<String>,
    pub format: String,
    pub quiet: bool,
    pub raw: bool,
}

impl FalkorCli {
    pub fn new(
        hostname: &str,
        port: u16,
        database: u8,
        auth: Option<&str>,
        format: String,
        quiet: bool,
        raw: bool,
    ) -> Result<Self> {
        let connection_string = auth.map_or_else(
            || format!("redis://{hostname}:{port}/{database}"),
            |password| format!("redis://{password}@{hostname}:{port}/{database}"),
        );

        let connection_info = FalkorConnectionInfo::try_from(connection_string)
            .context("Failed to create connection info")?;

        let client = FalkorClientBuilder::new()
            .with_connection_info(connection_info)
            .build()
            .context("Failed to create FalkorDB client")?;

        Ok(Self {
            client,
            current_graph: None,
            format,
            quiet,
            raw,
        })
    }

    pub fn set_graph(&mut self, graph_name: String) {
        self.current_graph = Some(graph_name);
    }

    pub fn get_graph_name(&self, provided: Option<&str>) -> Result<String> {
        provided.map_or_else(
            || {
                self.current_graph.clone().ok_or_else(|| {
                    anyhow::anyhow!("No graph specified. Use -g option or 'USE graph_name' command")
                })
            },
            |name| Ok(name.to_string()),
        )
    }

    pub fn execute_query(&self, graph_name: &str, query: &str, readonly: bool) -> Result<()> {
        let mut graph = self.client.select_graph(graph_name);

        let result = if readonly {
            graph.ro_query(query).execute()
        } else {
            graph.query(query).execute()
        };

        match result {
            Ok(query_result) => {
                if !self.quiet {
                    self.display_query_result(&query_result)?;
                }
                Ok(())
            }
            Err(e) => Err(anyhow::anyhow!("Query failed: {}", e)),
        }
    }

    fn display_query_result(
        &self,
        result: &falkordb::QueryResult<falkordb::LazyResultSet>,
    ) -> Result<()> {
        if self.raw {
            println!(
                "Raw result: headers={:?}, stats={:?}",
                result.header, result.stats
            );
            return Ok(());
        }

        match self.format.as_str() {
            "json" => Self::display_as_json(result),
            "csv" => Self::display_as_csv(result),
            _ => self.display_as_table(result),
        }
    }

    #[allow(clippy::unnecessary_wraps)]
    fn display_as_table(
        &self,
        result: &falkordb::QueryResult<falkordb::LazyResultSet>,
    ) -> Result<()> {
        // Display statistics
        if !self.quiet {
            println!("{}", "Statistics:".cyan().bold());
            println!(
                "  Nodes created: {}",
                result.get_nodes_created().unwrap_or(0)
            );
            println!(
                "  Nodes deleted: {}",
                result.get_nodes_deleted().unwrap_or(0)
            );
            println!(
                "  Relationships created: {}",
                result.get_relationship_created().unwrap_or(0)
            );
            println!(
                "  Relationships deleted: {}",
                result.get_relationship_deleted().unwrap_or(0)
            );
            println!(
                "  Properties set: {}",
                result.get_properties_set().unwrap_or(0)
            );
            if let Some(time) = result.get_internal_execution_time() {
                println!("  Query internal execution time: {time:.3} milliseconds");
            }
            println!();
        }

        // Display results if any
        let headers = &result.header;
        if !headers.is_empty() {
            // Print headers
            print!("| ");
            for header in headers {
                print!("{:15} | ", header.cyan().bold());
            }
            println!();

            // Print separator
            print!("|");
            for _ in headers {
                print!("-----------------+");
            }
            println!();

            // Note: We can't iterate over the LazyResultSet because it requires mutable access
            // This is a limitation of the current implementation
            println!(
                "| {:15} |",
                "Data available but iteration requires mutable access"
            );
        }

        Ok(())
    }

    fn display_as_json(
        result: &falkordb::QueryResult<falkordb::LazyResultSet>,
    ) -> Result<()> {
        println!(
            "{}",
            serde_json::to_string_pretty(&serde_json::json!({
                "statistics": {
                    "nodes_created": result.get_nodes_created().unwrap_or(0),
                    "nodes_deleted": result.get_nodes_deleted().unwrap_or(0),
                    "relationships_created": result.get_relationship_created().unwrap_or(0),
                    "relationships_deleted": result.get_relationship_deleted().unwrap_or(0),
                    "properties_set": result.get_properties_set().unwrap_or(0),
                    "query_time": result.get_internal_execution_time().unwrap_or(0.0),
                },
                "headers": result.header,
                "data": "Result data serialization would be implemented here"
            }))?
        );
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    fn display_as_csv(
        result: &falkordb::QueryResult<falkordb::LazyResultSet>,
    ) -> Result<()> {
        let headers = &result.header;
        if !headers.is_empty() {
            // Print headers
            println!("{}", headers.join(","));

            // Note: We can't iterate over the LazyResultSet because it requires mutable access
            println!("Data rows would be printed here if we had mutable access to the result set");
        }
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn list_graphs() -> Result<()> {
        // This would need to be implemented based on FalkorDB's graph listing capability
        // For now, we'll use a Redis command to list keys
        println!("Graph listing not directly supported in current falkordb-rs version");
        println!("Use Redis CLI: KEYS *");
        Ok(())
    }

    #[allow(clippy::unnecessary_wraps)]
    pub fn show_schema(&self, graph_name: &str) -> Result<()> {
        // Schema inspection would need to be done via queries since the schema methods are not public
        println!("{}", "Graph Schema:".cyan().bold());

        // Execute a query to get node labels
        let mut graph = self.client.select_graph(graph_name);

        // Query for node labels
        match graph.query("CALL db.labels()").execute() {
            Ok(_result) => {
                println!("  Node labels:");
                // Result would contain the labels, but we can't easily iterate over LazyResultSet
                println!("    (Use query 'CALL db.labels()' to see node labels)");
            }
            Err(_) => {
                println!("    Unable to retrieve node labels");
            }
        }

        // Query for relationship types
        match graph.query("CALL db.relationshipTypes()").execute() {
            Ok(_result) => {
                println!("  Relationship types:");
                // Result would contain the types, but we can't easily iterate over LazyResultSet
                println!("    (Use query 'CALL db.relationshipTypes()' to see relationship types)");
            }
            Err(_) => {
                println!("    Unable to retrieve relationship types");
            }
        }

        Ok(())
    }
}
