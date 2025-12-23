


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    fn test_switch_match_strings() {
        let a = Dynamic::from("hello");
        let b = Dynamic::from("hello");
        let c = Dynamic::from("world");

        assert!(switch_match_impl(&a, &b));
        assert!(!switch_match_impl(&a, &c));
    }

    #[test]


    fn test_switch_match_integers() {
        let a = Dynamic::from(42_i64);
        let b = Dynamic::from(42_i64);
        let c = Dynamic::from(0_i64);

        assert!(switch_match_impl(&a, &b));
        assert!(!switch_match_impl(&a, &c));
    }

    #[test]


    fn test_switch_match_floats() {
        let a = Dynamic::from(3.14_f64);
        let b = Dynamic::from(3.14_f64);
        let c = Dynamic::from(2.71_f64);

        assert!(switch_match_impl(&a, &b));
        assert!(!switch_match_impl(&a, &c));
    }

    #[test]


    fn test_switch_match_mixed_numeric() {
        let int_val = Dynamic::from(42_i64);
        let float_val = Dynamic::from(42.0_f64);

        assert!(switch_match_impl(&int_val, &float_val));
    }

    #[test]


    fn test_preprocess_simple_switch() {
        let input = r#"
SWITCH role
  CASE "admin"
    x = 1
  CASE "user"
    x = 2
  DEFAULT
    x = 0
END SWITCH
"#;
        let output = preprocess_switch(input);
        assert!(output.contains("__switch_expr_"));
        assert!(output.contains("if"));
        assert!(output.contains("else"));
    }

    #[test]


    fn test_preprocess_multiple_values() {
        let input = r#"
SWITCH day
  CASE "saturday", "sunday"
    weekend = true
  DEFAULT
    weekend = false
END SWITCH
"#;
        let output = preprocess_switch(input);
        assert!(output.contains("||"));
    }