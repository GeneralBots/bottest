


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]



    fn test_max_values() {
        assert_eq!(10_i64.max(5), 10);
        assert_eq!(3.5_f64.max(7.2), 7.2);
    }

    #[test]


    fn test_min_values() {
        assert_eq!(10_i64.min(5), 5);
        assert_eq!(3.5_f64.min(7.2), 3.5);
    }