


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_parse_duration() {

        assert!(parse_duration("30 days").is_ok());
        assert!(parse_duration("1 hour").is_ok());
        assert!(parse_duration("forever").is_ok());
        assert!(parse_duration("5 minutes").is_ok());
        assert!(parse_duration("invalid").is_err());
    }