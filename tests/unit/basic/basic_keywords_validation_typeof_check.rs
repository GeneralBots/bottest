


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    fn test_get_type_name() {
        assert_eq!(get_type_name(&Dynamic::UNIT), "null");
        assert_eq!(get_type_name(&Dynamic::from(true)), "boolean");
        assert_eq!(get_type_name(&Dynamic::from(42_i64)), "integer");
        assert_eq!(get_type_name(&Dynamic::from(3.14_f64)), "float");
        assert_eq!(get_type_name(&Dynamic::from("hello")), "string");
    }

    #[test]


    fn test_is_numeric() {
        assert!(is_numeric(&Dynamic::from(42_i64)));
        assert!(is_numeric(&Dynamic::from(3.14_f64)));
        assert!(is_numeric(&Dynamic::from("123")));
        assert!(is_numeric(&Dynamic::from("3.14")));
        assert!(!is_numeric(&Dynamic::from("hello")));
        assert!(!is_numeric(&Dynamic::from(true)));
    }