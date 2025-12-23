


use botserver::basic::keywords::string_functions::{instr_impl, is_numeric_impl};

    #[test]


    fn test_instr_basic() {
        assert_eq!(instr_impl(1, "Hello, World!", "World"), 8);
        assert_eq!(instr_impl(1, "Hello, World!", "o"), 5);
        assert_eq!(instr_impl(1, "Hello, World!", "xyz"), 0);
    }

    #[test]


    fn test_instr_with_start() {
        assert_eq!(instr_impl(1, "one two one", "one"), 1);
        assert_eq!(instr_impl(2, "one two one", "one"), 9);
        assert_eq!(instr_impl(10, "one two one", "one"), 0);
    }

    #[test]


    fn test_instr_edge_cases() {
        assert_eq!(instr_impl(1, "", "test"), 0);
        assert_eq!(instr_impl(1, "test", ""), 0);
        assert_eq!(instr_impl(1, "", ""), 0);
    }

    #[test]


    fn test_is_numeric_integers() {
        assert!(is_numeric_impl("42"));
        assert!(is_numeric_impl("-17"));
        assert!(is_numeric_impl("0"));
        assert!(is_numeric_impl("  42  "));
    }

    #[test]


    fn test_is_numeric_decimals() {
        assert!(is_numeric_impl("3.14"));
        assert!(is_numeric_impl("-0.5"));
        assert!(is_numeric_impl(".25"));
        assert!(is_numeric_impl("0.0"));
    }

    #[test]


    fn test_is_numeric_scientific() {
        assert!(is_numeric_impl("1e10"));
        assert!(is_numeric_impl("2.5E-3"));
        assert!(is_numeric_impl("-1.5e+2"));
    }

    #[test]


    fn test_is_numeric_invalid() {
        assert!(!is_numeric_impl(""));
        assert!(!is_numeric_impl("abc"));
        assert!(!is_numeric_impl("12abc"));
        assert!(!is_numeric_impl("$100"));
        assert!(!is_numeric_impl("1,000"));
    }
