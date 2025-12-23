


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]



    fn test_isnull_unit() {
        use rhai::Dynamic;
        let value = Dynamic::UNIT;
        assert!(value.is_unit());
    }

    #[test]


    fn test_isnull_not_unit() {
        use rhai::Dynamic;
        let value = Dynamic::from("test");
        assert!(!value.is_unit());
    }

    #[test]


    fn test_isnull_number() {
        use rhai::Dynamic;
        let value = Dynamic::from(42);
        assert!(!value.is_unit());
    }