


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    use rhai::{Array, Dynamic};

    #[test]


    fn test_push() {
        let mut arr: Array = vec![Dynamic::from(1), Dynamic::from(2)];
        arr.push(Dynamic::from(3));
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[2].as_int().unwrap_or(0), 3);
    }

    #[test]


    fn test_pop() {
        let mut arr: Array = vec![Dynamic::from(1), Dynamic::from(2), Dynamic::from(3)];
        let popped = arr.pop();
        assert_eq!(arr.len(), 2);
        assert_eq!(popped.and_then(|v| v.as_int().ok()).unwrap_or(0), 3);
    }

    #[test]


    fn test_pop_empty() {
        let mut arr: Array = vec![];
        let popped = arr.pop();
        assert!(popped.is_none());
    }

    #[test]


    fn test_shift() {
        let mut arr: Array = vec![Dynamic::from(1), Dynamic::from(2), Dynamic::from(3)];
        let shifted = arr.remove(0);
        assert_eq!(arr.len(), 2);
        assert_eq!(shifted.as_int().unwrap_or(0), 1);
        assert_eq!(arr[0].as_int().unwrap_or(0), 2);
    }

    #[test]


    fn test_unshift() {
        let mut arr: Array = vec![Dynamic::from(2), Dynamic::from(3)];
        arr.insert(0, Dynamic::from(1));
        assert_eq!(arr.len(), 3);
        assert_eq!(arr[0].as_int().unwrap_or(0), 1);
    }