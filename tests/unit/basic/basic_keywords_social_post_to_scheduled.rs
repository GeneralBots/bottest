


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_parse_schedule_time() {
        let result = parse_schedule_time("2025-02-01 10:00");
        assert!(result.is_ok());

        let result = parse_schedule_time("2025-02-01T10:00:00");
        assert!(result.is_ok());

        let result = parse_schedule_time("invalid");
        assert!(result.is_err());
    }