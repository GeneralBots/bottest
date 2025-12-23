


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_parse_time_string() {
        let result = parse_time_string("2024-01-15 14:30");
        assert!(result.is_ok());

        let result = parse_time_string("tomorrow at 3pm");
        assert!(result.is_ok());

        let result = parse_time_string("in 2 hours");
        assert!(result.is_ok());
    }

    #[test]


    fn test_parse_date_string() {
        let result = parse_date_string("today");
        assert!(result.is_ok());

        let result = parse_date_string("2024-01-15");
        assert!(result.is_ok());

        let result = parse_date_string("tomorrow");
        assert!(result.is_ok());
    }

    #[test]


    fn test_extract_hour() {
        assert_eq!(extract_hour_from_string("3pm"), Some(15));
        assert_eq!(extract_hour_from_string("3 PM"), Some(15));
        assert_eq!(extract_hour_from_string("10am"), Some(10));
        assert_eq!(extract_hour_from_string("12am"), Some(0));
        assert_eq!(extract_hour_from_string("12pm"), Some(12));
    }