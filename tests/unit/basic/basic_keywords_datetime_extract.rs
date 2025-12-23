


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_parse_date() {
        let date = parse_date("2025-01-22");
        assert!(date.is_some());
        let d = date.unwrap();
        assert_eq!(d.year(), 2025);
        assert_eq!(d.month(), 1);
        assert_eq!(d.day(), 22);
    }

    #[test]


    fn test_parse_datetime() {
        let dt = parse_datetime("2025-01-22 14:30:45");
        assert!(dt.is_some());
        let d = dt.unwrap();
        assert_eq!(d.hour(), 14);
        assert_eq!(d.minute(), 30);
        assert_eq!(d.second(), 45);
    }

    #[test]


    fn test_invalid_date() {
        let date = parse_date("invalid");
        assert!(date.is_none());
    }