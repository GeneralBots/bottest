//! Unit tests migrated from src/basic/keywords/use_account.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_parse_account_path() {
        let result = parse_account_path("account://user@gmail.com/Documents/file.pdf");
        assert!(result.is_some());
        let (email, path) = result.unwrap();
        assert_eq!(email, "user@gmail.com");
        assert_eq!(path, "Documents/file.pdf");
    }

    #[test]

    
    fn test_parse_account_path_invalid() {
        assert!(parse_account_path("local/file.pdf").is_none());
        assert!(parse_account_path("/absolute/path").is_none());
    }

    #[test]

    
    fn test_is_account_path() {
        assert!(is_account_path("account://user@gmail.com/file.pdf"));
        assert!(!is_account_path("local/file.pdf"));
        assert!(!is_account_path("file.pdf"));
    }