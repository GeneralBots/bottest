//! Unit tests migrated from src/basic/keywords/save_from_unstructured.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_clean_value_for_type() {
        assert_eq!(clean_value_for_type(&json!("test"), "text"), json!("test"));
        assert_eq!(clean_value_for_type(&json!("42"), "integer"), json!(42));
        assert_eq!(clean_value_for_type(&json!("3.14"), "numeric"), json!(3.14));
        assert_eq!(clean_value_for_type(&json!("true"), "boolean"), json!(true));
    }

    #[test]

    
    fn test_get_default_schema() {
        let leads_schema = get_default_schema("leads");
        assert!(leads_schema.is_array());

        let tasks_schema = get_default_schema("tasks");
        assert!(tasks_schema.is_array());
    }