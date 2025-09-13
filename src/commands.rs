use anyhow::Result;
use colored::*;

use crate::cli::Commands;
use crate::client::FalkorCli;

impl FalkorCli {
    pub fn handle_command(&mut self, command: Commands) -> Result<()> {
        match command {
            Commands::Query {
                graph,
                query,
                params: _,
            } => self.execute_query(&graph, &query, false),
            Commands::RoQuery {
                graph,
                query,
                params: _,
            } => self.execute_query(&graph, &query, true),
            Commands::Profile { graph, query } => {
                let mut graph_client = self.client.select_graph(&graph);
                match graph_client.profile(&query).execute() {
                    Ok(plan) => {
                        println!("{}", "Execution Plan:".cyan().bold());
                        println!("{:?}", plan);
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Profile failed: {}", e)),
                }
            }
            Commands::Explain { graph, query } => {
                let mut graph_client = self.client.select_graph(&graph);
                match graph_client.explain(&query).execute() {
                    Ok(plan) => {
                        println!("{}", "Query Explanation:".cyan().bold());
                        println!("{:?}", plan);
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Explain failed: {}", e)),
                }
            }
            Commands::Delete { graph } => {
                let mut graph_client = self.client.select_graph(&graph);
                match graph_client.delete() {
                    Ok(_) => {
                        println!("Graph '{}' deleted successfully", graph);
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Delete failed: {}", e)),
                }
            }
            Commands::List => self.list_graphs(),
            Commands::Schema { graph } => self.show_schema(&graph),
            Commands::Slowlog { graph } => {
                let graph_client = self.client.select_graph(&graph);
                match graph_client.slowlog() {
                    Ok(entries) => {
                        println!("{}", "Slowlog Entries:".cyan().bold());
                        for (i, entry) in entries.iter().enumerate() {
                            println!("{}. {:?}", i + 1, entry);
                        }
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Slowlog failed: {}", e)),
                }
            }
            Commands::SlowlogReset { graph } => {
                let graph_client = self.client.select_graph(&graph);
                match graph_client.slowlog_reset() {
                    Ok(_) => {
                        println!("Slowlog reset successfully");
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Slowlog reset failed: {}", e)),
                }
            }
            Commands::Indices { graph } => {
                let mut graph_client = self.client.select_graph(&graph);
                match graph_client.list_indices() {
                    Ok(indices) => {
                        println!("{}", "Indices:".cyan().bold());
                        for index in &indices.data {
                            println!("  {:?}", index);
                        }
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Indices query failed: {}", e)),
                }
            }
            Commands::CreateIndex {
                graph,
                entity_type,
                label,
                property,
            } => {
                let mut graph_client = self.client.select_graph(&graph);
                let entity = match entity_type.to_uppercase().as_str() {
                    "NODE" => falkordb::EntityType::Node,
                    "EDGE" | "RELATIONSHIP" => falkordb::EntityType::Edge,
                    _ => return Err(anyhow::anyhow!("Invalid entity type. Use NODE or EDGE")),
                };

                // Use Range index as default, the API requires IndexType, EntityType, label, properties, options
                match graph_client.create_index(
                    falkordb::IndexType::Range,
                    entity,
                    &label,
                    &[&property],
                    None,
                ) {
                    Ok(_) => {
                        println!(
                            "Index created successfully on {}:{} for {}",
                            entity_type, label, property
                        );
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Index creation failed: {}", e)),
                }
            }
            Commands::DropIndex {
                graph,
                entity_type,
                label,
                property,
            } => {
                let mut graph_client = self.client.select_graph(&graph);
                let entity = match entity_type.to_uppercase().as_str() {
                    "NODE" => falkordb::EntityType::Node,
                    "EDGE" | "RELATIONSHIP" => falkordb::EntityType::Edge,
                    _ => return Err(anyhow::anyhow!("Invalid entity type. Use NODE or EDGE")),
                };

                // Use Range index as default, the API requires IndexType, EntityType, label, properties
                match graph_client.drop_index(
                    falkordb::IndexType::Range,
                    entity,
                    &label,
                    &[&property],
                ) {
                    Ok(_) => {
                        println!(
                            "Index dropped successfully on {}:{} for {}",
                            entity_type, label, property
                        );
                        Ok(())
                    }
                    Err(e) => Err(anyhow::anyhow!("Index drop failed: {}", e)),
                }
            }
            Commands::Call {
                graph,
                procedure,
                args: _,
            } => {
                // This would need proper parameter parsing for procedure calls
                println!("Procedure calls not fully implemented yet");
                println!("Procedure: {}", procedure);
                println!("Graph: {}", graph);
                println!(
                    "Use: graph.call_procedure(\"{}\").execute() in the Rust API",
                    procedure
                );
                Ok(())
            }
            Commands::Interactive => self.interactive_mode(),
        }
    }
}
