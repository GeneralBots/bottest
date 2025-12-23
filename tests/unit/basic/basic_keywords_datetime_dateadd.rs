


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_dateadd_days() {
        assert_eq!(dateadd_impl("2025-01-15", 5, "day"), "2025-01-20");
        assert_eq!(dateadd_impl("2025-01-15", -10, "day"), "2025-01-05");
    }

    #[test]


    fn test_dateadd_months() {
        assert_eq!(dateadd_impl("2025-01-15", 1, "month"), "2025-02-15");
        assert_eq!(dateadd_impl("2025-01-15", -1, "month"), "2024-12-15");
    }

    #[test]


    fn test_dateadd_years() {
        assert_eq!(dateadd_impl("2025-01-15", 1, "year"), "2026-01-15");
    }

    #[test]


    fn test_datediff_days() {
        assert_eq!(datediff_impl("2025-01-01", "2025-01-15", "day"), 14);
        assert_eq!(datediff_impl("2025-01-15", "2025-01-01", "day"), -14);
    }

    #[test]


    fn test_datediff_months() {
        assert_eq!(datediff_impl("2025-01-01", "2025-03-01", "month"), 2);
    }

    #[test]


    fn test_datediff_years() {
        assert_eq!(datediff_impl("2024-01-01", "2025-01-01", "year"), 1);
    }

    #[test]


    fn test_parse_date() {
        assert!(parse_date("2025-01-15").is_some());
        assert!(parse_date("15/01/2025").is_some());
        assert!(parse_date("invalid").is_none());
    }