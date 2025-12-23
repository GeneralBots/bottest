//! Unit tests migrated from src/basic/keywords/data_operations.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_sanitize_identifier() {
        assert_eq!(sanitize_identifier("users"), "users");
        assert_eq!(sanitize_identifier("user_name"), "user_name");
        assert_eq!(
            sanitize_identifier("users; DROP TABLE users;"),
            "usersDROPTABLEusers"
        );
    }

    #[test]

    
    fn test_sanitize_sql() {
        assert_eq!(sanitize_sql("hello"), "hello");
        assert_eq!(sanitize_sql("it's"), "it''s");
        assert_eq!(sanitize_sql("O'Brien"), "O''Brien");
    }

    #[test]

    
    fn test_parse_condition() {
        let (field, op, value) = parse_condition_internal("status=active").unwrap();
        assert_eq!(field, "status");
        assert_eq!(op, "=");
        assert_eq!(value, "active");

        let (field, op, value) = parse_condition_internal("age>=18").unwrap();
        assert_eq!(field, "age");
        assert_eq!(op, ">=");
        assert_eq!(value, "18");
    }

    #[test]

    
    fn test_parse_filter_clause() {
        let clause = parse_filter_clause("name=John").unwrap();
        assert!(clause.contains("name"));
        assert!(clause.contains("John"));
    }