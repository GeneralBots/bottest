//! Unit tests migrated from src/basic/keywords/arrays/slice.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;

// Original: use super::*; - tests used internal functions from botserver

    fn make_test_array() -> Array {
        vec![
            Dynamic::from(1),
            Dynamic::from(2),
            Dynamic::from(3),
            Dynamic::from(4),
            Dynamic::from(5),
        ]
    }

    #[test]

    
    fn test_slice_from_start() {
        let arr = make_test_array();
        let result = slice_array(&arr, 2, None);
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].as_int().unwrap(), 3);
    }

    #[test]

    
    fn test_slice_with_end() {
        let arr = make_test_array();
        let result = slice_array(&arr, 1, Some(3));
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].as_int().unwrap(), 2);
        assert_eq!(result[1].as_int().unwrap(), 3);
    }

    #[test]

    
    fn test_slice_negative_start() {
        let arr = make_test_array();
        let result = slice_array(&arr, -2, None);
        assert_eq!(result.len(), 2);
        assert_eq!(result[0].as_int().unwrap(), 4);
        assert_eq!(result[1].as_int().unwrap(), 5);
    }

    #[test]

    
    fn test_slice_negative_end() {
        let arr = make_test_array();
        let result = slice_array(&arr, 0, Some(-2));
        assert_eq!(result.len(), 3);
        assert_eq!(result[0].as_int().unwrap(), 1);
        assert_eq!(result[2].as_int().unwrap(), 3);
    }

    #[test]

    
    fn test_slice_out_of_bounds() {
        let arr = make_test_array();
        let result = slice_array(&arr, 10, None);
        assert!(result.is_empty());
    }

    #[test]

    
    fn test_slice_empty_range() {
        let arr = make_test_array();
        let result = slice_array(&arr, 3, Some(2));
        assert!(result.is_empty());
    }