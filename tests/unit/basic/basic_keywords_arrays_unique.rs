//! Unit tests migrated from src/basic/keywords/arrays/unique.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;

// Original: use super::*; - tests used internal functions from botserver
    use rhai::Dynamic;

    #[test]

    
    fn test_unique_integers() {
        let mut arr = Array::new();
        arr.push(Dynamic::from(1_i64));
        arr.push(Dynamic::from(2_i64));
        arr.push(Dynamic::from(2_i64));
        arr.push(Dynamic::from(3_i64));
        arr.push(Dynamic::from(3_i64));
        arr.push(Dynamic::from(3_i64));
        arr.push(Dynamic::from(4_i64));

        let result = unique_array(arr);
        assert_eq!(result.len(), 4);
    }

    #[test]

    
    fn test_unique_strings() {
        let mut arr = Array::new();
        arr.push(Dynamic::from("Alice"));
        arr.push(Dynamic::from("Bob"));
        arr.push(Dynamic::from("Alice"));
        arr.push(Dynamic::from("Charlie"));

        let result = unique_array(arr);
        assert_eq!(result.len(), 3);
    }

    #[test]

    
    fn test_unique_preserves_order() {
        let mut arr = Array::new();
        arr.push(Dynamic::from("C"));
        arr.push(Dynamic::from("A"));
        arr.push(Dynamic::from("B"));
        arr.push(Dynamic::from("A"));
        arr.push(Dynamic::from("C"));

        let result = unique_array(arr);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].to_string(), "C");
        assert_eq!(result[1].to_string(), "A");
        assert_eq!(result[2].to_string(), "B");
    }

    #[test]

    
    fn test_unique_empty_array() {
        let arr = Array::new();
        let result = unique_array(arr);
        assert!(result.is_empty());
    }

    #[test]

    
    fn test_unique_single_element() {
        let mut arr = Array::new();
        arr.push(Dynamic::from(42_i64));

        let result = unique_array(arr);
        assert_eq!(result.len(), 1);
    }

    #[test]

    
    fn test_unique_all_same() {
        let mut arr = Array::new();
        arr.push(Dynamic::from(1_i64));
        arr.push(Dynamic::from(1_i64));
        arr.push(Dynamic::from(1_i64));

        let result = unique_array(arr);
        assert_eq!(result.len(), 1);
    }

    #[test]

    
    fn test_unique_mixed_types() {
        let mut arr = Array::new();
        arr.push(Dynamic::from(1_i64));
        arr.push(Dynamic::from("1"));
        arr.push(Dynamic::from(1_i64));

        let result = unique_array(arr);
        // "1" (int) and "1" (string) may have same string representation
        // so behavior depends on Dynamic::to_string() implementation
        assert!(result.len() >= 1 && result.len() <= 2);
    }