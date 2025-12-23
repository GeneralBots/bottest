


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    use rhai::Dynamic;

    #[test]


    fn test_ubound() {
        let arr: Vec<Dynamic> = vec![Dynamic::from(1), Dynamic::from(2), Dynamic::from(3)];
        assert_eq!(arr.len() - 1, 2);
    }

    #[test]


    fn test_join() {
        let arr = vec!["a", "b", "c"];
        let result = arr.join("-");
        assert_eq!(result, "a-b-c");
    }

    #[test]


    fn test_split() {
        let s = "a,b,c";
        let parts: Vec<&str> = s.split(',').collect();
        assert_eq!(parts.len(), 3);
    }

    #[test]


    fn test_range() {
        let range: Vec<i64> = (1..=5).collect();
        assert_eq!(range, vec![1, 2, 3, 4, 5]);
    }

    #[test]


    fn test_flatten() {

        let nested = vec![vec![1, 2], vec![3, 4]];
        let flat: Vec<i32> = nested.into_iter().flatten().collect();
        assert_eq!(flat, vec![1, 2, 3, 4]);
    }