


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    fn test_contains_string() {
        let arr: Array = vec![
            Dynamic::from("Alice"),
            Dynamic::from("Bob"),
            Dynamic::from("Charlie"),
        ];

        assert!(array_contains(&arr, &Dynamic::from("Bob")));
        assert!(!array_contains(&arr, &Dynamic::from("David")));
    }

    #[test]


    fn test_contains_integer() {
        let arr: Array = vec![
            Dynamic::from(1_i64),
            Dynamic::from(2_i64),
            Dynamic::from(3_i64),
        ];

        assert!(array_contains(&arr, &Dynamic::from(2_i64)));
        assert!(!array_contains(&arr, &Dynamic::from(5_i64)));
    }

    #[test]


    fn test_contains_float() {
        let arr: Array = vec![
            Dynamic::from(1.5_f64),
            Dynamic::from(2.5_f64),
            Dynamic::from(3.5_f64),
        ];

        assert!(array_contains(&arr, &Dynamic::from(2.5_f64)));
        assert!(!array_contains(&arr, &Dynamic::from(4.5_f64)));
    }

    #[test]


    fn test_contains_bool() {
        let arr: Array = vec![Dynamic::from(true), Dynamic::from(false)];

        assert!(array_contains(&arr, &Dynamic::from(true)));
        assert!(array_contains(&arr, &Dynamic::from(false)));
    }

    #[test]


    fn test_contains_empty_array() {
        let arr = Array::new();
        assert!(!array_contains(&arr, &Dynamic::from("anything")));
    }

    #[test]


    fn test_items_equal_integers() {
        assert!(items_equal(&Dynamic::from(5_i64), &Dynamic::from(5_i64)));
        assert!(!items_equal(&Dynamic::from(5_i64), &Dynamic::from(6_i64)));
    }

    #[test]


    fn test_items_equal_strings() {
        assert!(items_equal(
            &Dynamic::from("hello"),
            &Dynamic::from("hello")
        ));
        assert!(!items_equal(
            &Dynamic::from("hello"),
            &Dynamic::from("world")
        ));
    }