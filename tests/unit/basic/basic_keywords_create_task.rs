//! Unit tests migrated from src/basic/keywords/create_task.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_parse_due_date() {
        assert!(parse_due_date("tomorrow").is_ok());
        assert!(parse_due_date("+3 days").is_ok());
        assert!(parse_due_date("2024-12-31").is_ok());
        assert!(parse_due_date("null").unwrap().is_none());
    }

    #[test]

    
    fn test_determine_priority() {
        let tomorrow = Some(Utc::now() + Duration::days(1));
        assert_eq!(determine_priority(tomorrow), "high");

        let next_week = Some(Utc::now() + Duration::days(7));
        assert_eq!(determine_priority(next_week), "medium");

        let next_month = Some(Utc::now() + Duration::days(30));
        assert_eq!(determine_priority(next_month), "low");
    }