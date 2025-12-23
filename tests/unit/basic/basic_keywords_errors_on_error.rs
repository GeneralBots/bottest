//! Unit tests migrated from src/basic/keywords/errors/on_error.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_error_resume_next_flag() {
        // Initially should be false
        assert!(!is_error_resume_next_active());

        // Enable it
        set_error_resume_next(true);
        assert!(is_error_resume_next_active());

        // Disable it
        set_error_resume_next(false);
        assert!(!is_error_resume_next_active());
    }

    #[test]

    
    fn test_error_storage() {
        clear_last_error();
        assert!(get_last_error().is_none());
        assert_eq!(get_error_number(), 0);

        set_last_error("Test error", 42);
        assert_eq!(get_last_error(), Some("Test error".to_string()));
        assert_eq!(get_error_number(), 42);

        clear_last_error();
        assert!(get_last_error().is_none());
        assert_eq!(get_error_number(), 0);
    }

    #[test]

    
    fn test_handle_error_without_resume_next() {
        set_error_resume_next(false);
        clear_last_error();

        let result: Result<String, Box<dyn std::error::Error + Send + Sync>> =
            Err("Test error".into());
        let handled = handle_error(result);

        // Should return error when ON ERROR RESUME NEXT is not active
        assert!(handled.is_err());
    }

    #[test]

    
    fn test_handle_error_with_resume_next() {
        set_error_resume_next(true);
        clear_last_error();

        let result: Result<String, Box<dyn std::error::Error + Send + Sync>> =
            Err("Test error".into());
        let handled = handle_error(result);

        // Should return Ok(UNIT) when ON ERROR RESUME NEXT is active
        assert!(handled.is_ok());
        assert_eq!(get_last_error(), Some("Test error".to_string()));

        // Cleanup
        set_error_resume_next(false);
    }