//! Unit tests migrated from src/basic/keywords/math/aggregate.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    use rhai::Dynamic;

    #[test]

    
    fn test_sum() {
        let arr: Vec<Dynamic> = vec![
            Dynamic::from(10_i64),
            Dynamic::from(20_i64),
            Dynamic::from(30_i64),
        ];
        let sum: f64 = arr
            .iter()
            .filter_map(|v| v.as_int().ok().map(|i| i as f64))
            .sum();
        assert_eq!(sum, 60.0);
    }

    #[test]

    
    fn test_avg() {
        let arr: Vec<f64> = vec![10.0, 20.0, 30.0];
        let sum: f64 = arr.iter().sum();
        let avg = sum / arr.len() as f64;
        assert_eq!(avg, 20.0);
    }

    #[test]

    
    fn test_empty_array() {
        let arr: Vec<f64> = vec![];
        let result = if arr.is_empty() { 0.0 } else { arr.iter().sum::<f64>() / arr.len() as f64 };
        assert_eq!(result, 0.0);
    }