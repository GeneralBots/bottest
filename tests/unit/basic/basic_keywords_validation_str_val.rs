//! Unit tests migrated from src/basic/keywords/validation/str_val.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    
    fn test_val_parsing() {
        assert_eq!("123.45".trim().parse::<f64>().unwrap_or(0.0), 123.45);
        assert_eq!("  456  ".trim().parse::<f64>().unwrap_or(0.0), 456.0);
        assert_eq!("abc".trim().parse::<f64>().unwrap_or(0.0), 0.0);
    }

    #[test]

    
    fn test_cint_rounding() {
        assert_eq!(2.4_f64.round() as i64, 2);
        assert_eq!(2.5_f64.round() as i64, 3);
        assert_eq!(2.6_f64.round() as i64, 3);
        assert_eq!((-2.5_f64).round() as i64, -3);
    }