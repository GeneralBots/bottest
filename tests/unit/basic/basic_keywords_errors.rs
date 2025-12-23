//! Unit tests migrated from src/basic/keywords/errors/mod.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    
    fn test_error_map() {
        use rhai::{Dynamic, Map};

        let mut map = Map::new();
        map.insert("error".into(), Dynamic::from(true));
        map.insert("message".into(), Dynamic::from("test error"));

        assert!(map.contains_key("error"));
        assert_eq!(map.get("error").unwrap().as_bool().unwrap_or(false), true);
    }