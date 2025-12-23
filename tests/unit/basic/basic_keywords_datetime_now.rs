//! Unit tests migrated from src/basic/keywords/datetime/now.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_create_datetime_map() {
        let now = Local::now();
        let map = create_datetime_map(now);

        assert!(map.contains_key("year"));
        assert!(map.contains_key("month"));
        assert!(map.contains_key("day"));
        assert!(map.contains_key("hour"));
        assert!(map.contains_key("minute"));
        assert!(map.contains_key("second"));
        assert!(map.contains_key("weekday"));
        assert!(map.contains_key("timestamp"));
        assert!(map.contains_key("formatted"));
        assert!(map.contains_key("is_weekend"));
        assert!(map.contains_key("quarter"));
    }

    #[test]

    
    fn test_year_extraction() {
        let now = Local::now();
        let map = create_datetime_map(now);

        let year = map.get("year").unwrap().as_int().unwrap();
        assert!(year >= 2024);
    }

    #[test]

    
    fn test_month_range() {
        let now = Local::now();
        let map = create_datetime_map(now);

        let month = map.get("month").unwrap().as_int().unwrap();
        assert!(month >= 1 && month <= 12);
    }

    #[test]

    
    fn test_hour12_range() {
        let now = Local::now();
        let map = create_datetime_map(now);

        let hour12 = map.get("hour12").unwrap().as_int().unwrap();
        assert!(hour12 >= 1 && hour12 <= 12);
    }

    #[test]

    
    fn test_quarter_calculation() {
        let now = Local::now();
        let map = create_datetime_map(now);

        let quarter = map.get("quarter").unwrap().as_int().unwrap();
        assert!(quarter >= 1 && quarter <= 4);
    }