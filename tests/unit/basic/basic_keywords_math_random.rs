//! Unit tests migrated from src/basic/keywords/math/random.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    
    fn test_mod() {
        assert_eq!(17 % 5, 2);
        assert_eq!(10 % 3, 1);
        assert_eq!(0 % 5, 0);
    }