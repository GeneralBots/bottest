


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    fn test_sort_integers() {
        let arr: Array = vec![
            Dynamic::from(3),
            Dynamic::from(1),
            Dynamic::from(4),
            Dynamic::from(1),
            Dynamic::from(5),
        ];
        let sorted = sort_array(arr, false);
        assert_eq!(sorted[0].as_int().unwrap(), 1);
        assert_eq!(sorted[1].as_int().unwrap(), 1);
        assert_eq!(sorted[2].as_int().unwrap(), 3);
        assert_eq!(sorted[3].as_int().unwrap(), 4);
        assert_eq!(sorted[4].as_int().unwrap(), 5);
    }

    #[test]


    fn test_sort_strings() {
        let arr: Array = vec![
            Dynamic::from("banana"),
            Dynamic::from("apple"),
            Dynamic::from("cherry"),
        ];
        let sorted = sort_array(arr, false);
        assert_eq!(sorted[0].clone().into_string().unwrap(), "apple");
        assert_eq!(sorted[1].clone().into_string().unwrap(), "banana");
        assert_eq!(sorted[2].clone().into_string().unwrap(), "cherry");
    }

    #[test]


    fn test_sort_descending() {
        let arr: Array = vec![Dynamic::from(1), Dynamic::from(3), Dynamic::from(2)];
        let sorted = sort_array(arr, true);
        assert_eq!(sorted[0].as_int().unwrap(), 3);
        assert_eq!(sorted[1].as_int().unwrap(), 2);
        assert_eq!(sorted[2].as_int().unwrap(), 1);
    }

    #[test]


    fn test_compare_dynamic_numbers() {
        let a = Dynamic::from(5);
        let b = Dynamic::from(3);
        assert_eq!(compare_dynamic(&a, &b), std::cmp::Ordering::Greater);
    }

    #[test]


    fn test_compare_dynamic_strings() {
        let a = Dynamic::from("apple");
        let b = Dynamic::from("banana");
        assert_eq!(compare_dynamic(&a, &b), std::cmp::Ordering::Less);
    }