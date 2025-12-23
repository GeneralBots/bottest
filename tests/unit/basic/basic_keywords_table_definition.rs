


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_parse_table_definition() {
        let source = r#"
TABLE Contacts ON maria
    Id number key
    Nome string(150)
    Email string(255)
    Telefone string(20)
END TABLE
"#;

        let tables = parse_table_definition(source).unwrap();
        assert_eq!(tables.len(), 1);

        let table = &tables[0];
        assert_eq!(table.name, "Contacts");
        assert_eq!(table.connection_name, "maria");
        assert_eq!(table.fields.len(), 4);

        assert_eq!(table.fields[0].name, "Id");
        assert_eq!(table.fields[0].field_type, "number");
        assert!(table.fields[0].is_key);

        assert_eq!(table.fields[1].name, "Nome");
        assert_eq!(table.fields[1].field_type, "string");
        assert_eq!(table.fields[1].length, Some(150));
    }

    #[test]


    fn test_parse_field_with_precision() {
        let field = parse_field_definition("Preco double(10,2)", 0).unwrap();
        assert_eq!(field.name, "Preco");
        assert_eq!(field.field_type, "double");
        assert_eq!(field.length, Some(10));
        assert_eq!(field.precision, Some(2));
    }

    #[test]


    fn test_generate_create_table_sql() {
        let table = TableDefinition {
            name: "TestTable".to_string(),
            connection_name: "default".to_string(),
            fields: vec![
                FieldDefinition {
                    name: "id".to_string(),
                    field_type: "number".to_string(),
                    length: None,
                    precision: None,
                    is_key: true,
                    is_nullable: false,
                    default_value: None,
                    reference_table: None,
                    field_order: 0,
                },
                FieldDefinition {
                    name: "name".to_string(),
                    field_type: "string".to_string(),
                    length: Some(100),
                    precision: None,
                    is_key: false,
                    is_nullable: true,
                    default_value: None,
                    reference_table: None,
                    field_order: 1,
                },
            ],
        };

        let sql = generate_create_table_sql(&table, "postgres");
        assert!(sql.contains("CREATE TABLE IF NOT EXISTS TestTable"));
        assert!(sql.contains("id INTEGER NOT NULL"));
        assert!(sql.contains("name VARCHAR(100)"));
        assert!(sql.contains("PRIMARY KEY (id)"));
    }

    #[test]


    fn test_map_types() {
        let field = FieldDefinition {
            name: "test".to_string(),
            field_type: "string".to_string(),
            length: Some(50),
            precision: None,
            is_key: false,
            is_nullable: true,
            default_value: None,
            reference_table: None,
            field_order: 0,
        };
        assert_eq!(map_type_to_sql(&field, "postgres"), "VARCHAR(50)");

        let date_field = FieldDefinition {
            name: "created".to_string(),
            field_type: "datetime".to_string(),
            length: None,
            precision: None,
            is_key: false,
            is_nullable: true,
            default_value: None,
            reference_table: None,
            field_order: 0,
        };
        assert_eq!(map_type_to_sql(&date_field, "mysql"), "DATETIME");
        assert_eq!(map_type_to_sql(&date_field, "postgres"), "TIMESTAMP");
    }

    #[test]


    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("valid_name"), "valid_name");
        assert_eq!(sanitize_identifier("DROP TABLE; --"), "DROPTABLE");
        assert_eq!(sanitize_identifier("name123"), "name123");
    }

    #[test]


    fn test_build_connection_string() {
        let conn = ExternalConnection {
            name: "test".to_string(),
            driver: "mysql".to_string(),
            server: "localhost".to_string(),
            port: Some(3306),
            database: "testdb".to_string(),
            username: "user".to_string(),
            password: "pass".to_string(),
        };

        let conn_str = build_connection_string(&conn);
        assert_eq!(conn_str, "mysql://user:pass@localhost:3306/testdb");
    }