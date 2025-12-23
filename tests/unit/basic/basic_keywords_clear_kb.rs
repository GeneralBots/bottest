//! Unit tests migrated from src/basic/keywords/clear_kb.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;
use rhai::Engine;

// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_clear_kb_syntax() {
        let mut engine = Engine::new();

        // Test CLEAR_KB with argument
        assert!(engine
            .register_custom_syntax(&["CLEAR_KB", "$expr$"], true, |_, _| Ok(Dynamic::UNIT))
            .is_ok());

        // Test CLEAR_KB without argument
        assert!(engine
            .register_custom_syntax(&["CLEAR_KB"], true, |_, _| Ok(Dynamic::UNIT))
            .is_ok());
    }