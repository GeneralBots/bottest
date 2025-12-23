//! Unit tests migrated from src/basic/keywords/validation/isempty.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;

// Original: use super::*; - tests used internal functions from botserver
    use rhai::{Array, Map};

    #[test]

    
    fn test_empty_string() {
        let value = Dynamic::from("");
        assert!(check_empty(&value));
    }

    #[test]

    
    fn test_non_empty_string() {
        let value = Dynamic::from("hello");
        assert!(!check_empty(&value));
    }

    #[test]

    
    fn test_empty_array() {
        let value = Dynamic::from(Array::new());
        assert!(check_empty(&value));
    }

    #[test]

    
    fn test_non_empty_array() {
        let mut arr = Array::new();
        arr.push(Dynamic::from(1));
        let value = Dynamic::from(arr);
        assert!(!check_empty(&value));
    }

    #[test]

    
    fn test_empty_map() {
        let value = Dynamic::from(Map::new());
        assert!(check_empty(&value));
    }

    #[test]

    
    fn test_unit() {
        let value = Dynamic::UNIT;
        assert!(check_empty(&value));
    }

    #[test]

    
    fn test_number_not_empty() {
        let value = Dynamic::from(0);
        assert!(!check_empty(&value));
    }

    #[test]

    
    fn test_bool_not_empty() {
        let value = Dynamic::from(false);
        assert!(!check_empty(&value));
    }