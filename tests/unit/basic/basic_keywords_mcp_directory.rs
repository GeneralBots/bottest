


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_parse_csv_columns() {
        let loader = McpCsvLoader::new("./work", "test");

        let cols = loader.parse_csv_columns("name,type,command");
        assert_eq!(cols, vec!["name", "type", "command"]);

        let cols = loader.parse_csv_columns(
            "filesystem,stdio,npx,\"-y @modelcontextprotocol/server-filesystem\"",
        );
        assert_eq!(cols.len(), 4);
        assert_eq!(cols[3], "-y @modelcontextprotocol/server-filesystem");
    }

    #[test]


    fn test_parse_args() {
        let loader = McpCsvLoader::new("./work", "test");

        let args = loader.parse_args("-y @modelcontextprotocol/server-filesystem /data");
        assert_eq!(
            args,
            vec!["-y", "@modelcontextprotocol/server-filesystem", "/data"]
        );
    }

    #[test]


    fn test_infer_server_type() {
        let loader = McpCsvLoader::new("./work", "test");

        assert!(matches!(
            loader.infer_server_type("filesystem", "stdio", "npx"),
            McpServerType::Filesystem
        ));
        assert!(matches!(
            loader.infer_server_type("postgres", "stdio", "npx"),
            McpServerType::Database
        ));
        assert!(matches!(
            loader.infer_server_type("myapi", "http", "https://api.example.com"),
            McpServerType::Web
        ));
    }