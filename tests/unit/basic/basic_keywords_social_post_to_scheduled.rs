//! Unit tests migrated from src/basic/keywords/social/post_to_scheduled.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_parse_schedule_time() {
        let result = parse_schedule_time("2025-02-01 10:00");
        assert!(result.is_ok());

        let result = parse_schedule_time("2025-02-01T10:00:00");
        assert!(result.is_ok());

        let result = parse_schedule_time("invalid");
        assert!(result.is_err());
    }