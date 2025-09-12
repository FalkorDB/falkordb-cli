#[cfg(test)]
mod tests {
    use crate::{Cli, cli::Commands};
    use clap::Parser;

    #[test]
    fn test_cli_parsing() {
        // Test basic CLI argument parsing
        let cli = Cli::try_parse_from(&[
            "falkordb-cli",
            "--hostname", "localhost",
            "-p", "6379",
            "-g", "test_graph",
            "query",
            "test_graph",
            "MATCH (n) RETURN n"
        ]).unwrap();

        assert_eq!(cli.hostname, "localhost");
        assert_eq!(cli.port, 6379);
        assert_eq!(cli.graph, Some("test_graph".to_string()));
        
        match cli.command {
            Some(Commands::Query { graph, query, params: _ }) => {
                assert_eq!(graph, "test_graph");
                assert_eq!(query, "MATCH (n) RETURN n");
            }
            _ => panic!("Expected Query command"),
        }
    }

    #[test]
    fn test_interactive_mode_parsing() {
        let cli = Cli::try_parse_from(&[
            "falkordb-cli",
            "interactive"
        ]).unwrap();

        match cli.command {
            Some(Commands::Interactive) => {
                // Success
            }
            _ => panic!("Expected Interactive command"),
        }
    }

    #[test]
    fn test_index_commands() {
        let cli = Cli::try_parse_from(&[
            "falkordb-cli",
            "create-index",
            "mygraph",
            "NODE",
            "Person",
            "name"
        ]).unwrap();

        match cli.command {
            Some(Commands::CreateIndex { graph, entity_type, label, property }) => {
                assert_eq!(graph, "mygraph");
                assert_eq!(entity_type, "NODE");
                assert_eq!(label, "Person");
                assert_eq!(property, "name");
            }
            _ => panic!("Expected CreateIndex command"),
        }
    }

    #[test]
    fn test_default_values() {
        let cli = Cli::try_parse_from(&[
            "falkordb-cli"
        ]).unwrap();

        assert_eq!(cli.hostname, "localhost");
        assert_eq!(cli.port, 6379);
        assert_eq!(cli.database, 0);
        assert_eq!(cli.format, "table");
        assert_eq!(cli.quiet, false);
        assert_eq!(cli.raw, false);
        assert!(cli.auth.is_none());
        assert!(cli.graph.is_none());
        assert!(cli.eval.is_none());
        assert!(cli.file.is_none());
    }

    #[test]
    fn test_output_formats() {
        for format in &["table", "json", "csv"] {
            let cli = Cli::try_parse_from(&[
                "falkordb-cli",
                "--format", format,
                "query",
                "test",
                "MATCH (n) RETURN n"
            ]).unwrap();

            assert_eq!(cli.format, *format);
        }
    }

    #[test]
    fn test_eval_mode() {
        let cli = Cli::try_parse_from(&[
            "falkordb-cli",
            "-g", "test",
            "--eval", "CREATE (n:Test)"
        ]).unwrap();

        assert_eq!(cli.eval, Some("CREATE (n:Test)".to_string()));
        assert_eq!(cli.graph, Some("test".to_string()));
    }
}
