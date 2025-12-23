//! Unit tests migrated from src/basic/keywords/use_website.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;
use rhai::Engine;

// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_url_sanitization() {
        assert_eq!(
            sanitize_url_for_collection("https://docs.example.com/path"),
            "docs_example_com_path"
        );
        assert_eq!(
            sanitize_url_for_collection("http://test.site:8080"),
            "test_site_8080"
        );
    }

    #[test]

    
    fn test_use_website_syntax() {
        let mut engine = Engine::new();

        // Test USE_WEBSITE with argument
        assert!(engine
            .register_custom_syntax(&["USE_WEBSITE", "$expr$"], true, |_, _| Ok(Dynamic::UNIT))
            .is_ok());

        // Test CLEAR_WEBSITES without argument
        assert!(engine
            .register_custom_syntax(&["CLEAR_WEBSITES"], true, |_, _| Ok(Dynamic::UNIT))
            .is_ok());
    }