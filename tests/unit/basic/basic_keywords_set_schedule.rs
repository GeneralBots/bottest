


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_every_minute() {
        assert_eq!(parse_natural_schedule("every minute").unwrap(), "* * * * *");
    }

    #[test]


    fn test_every_n_minutes() {
        assert_eq!(
            parse_natural_schedule("every 5 minutes").unwrap(),
            "*/5 * * * *"
        );
        assert_eq!(
            parse_natural_schedule("every 15 minutes").unwrap(),
            "*/15 * * * *"
        );
        assert_eq!(
            parse_natural_schedule("every 30 minutes").unwrap(),
            "*/30 * * * *"
        );
    }

    #[test]


    fn test_every_hour() {
        assert_eq!(parse_natural_schedule("every hour").unwrap(), "0 * * * *");
        assert_eq!(parse_natural_schedule("hourly").unwrap(), "0 * * * *");
    }

    #[test]


    fn test_every_n_hours() {
        assert_eq!(
            parse_natural_schedule("every 2 hours").unwrap(),
            "0 */2 * * *"
        );
        assert_eq!(
            parse_natural_schedule("every 6 hours").unwrap(),
            "0 */6 * * *"
        );
    }

    #[test]


    fn test_every_day() {
        assert_eq!(parse_natural_schedule("every day").unwrap(), "0 0 * * *");
        assert_eq!(parse_natural_schedule("daily").unwrap(), "0 0 * * *");
    }

    #[test]


    fn test_every_week() {
        assert_eq!(parse_natural_schedule("every week").unwrap(), "0 0 * * 0");
        assert_eq!(parse_natural_schedule("weekly").unwrap(), "0 0 * * 0");
    }

    #[test]


    fn test_every_month() {
        assert_eq!(parse_natural_schedule("every month").unwrap(), "0 0 1 * *");
        assert_eq!(parse_natural_schedule("monthly").unwrap(), "0 0 1 * *");
    }

    #[test]


    fn test_at_time() {
        assert_eq!(parse_natural_schedule("at 9am").unwrap(), "0 9 * * *");
        assert_eq!(parse_natural_schedule("at 9:30am").unwrap(), "30 9 * * *");
        assert_eq!(parse_natural_schedule("at 2pm").unwrap(), "0 14 * * *");
        assert_eq!(parse_natural_schedule("at 14:00").unwrap(), "0 14 * * *");
        assert_eq!(parse_natural_schedule("at midnight").unwrap(), "0 0 * * *");
        assert_eq!(parse_natural_schedule("at noon").unwrap(), "0 12 * * *");
    }

    #[test]


    fn test_day_of_week() {
        assert_eq!(parse_natural_schedule("every monday").unwrap(), "0 0 * * 1");
        assert_eq!(parse_natural_schedule("every friday").unwrap(), "0 0 * * 5");
        assert_eq!(parse_natural_schedule("every sunday").unwrap(), "0 0 * * 0");
    }

    #[test]


    fn test_day_with_time() {
        assert_eq!(
            parse_natural_schedule("every monday at 9am").unwrap(),
            "0 9 * * 1"
        );
        assert_eq!(
            parse_natural_schedule("every friday at 5pm").unwrap(),
            "0 17 * * 5"
        );
    }

    #[test]


    fn test_weekdays() {
        assert_eq!(parse_natural_schedule("weekdays").unwrap(), "0 0 * * 1-5");
        assert_eq!(
            parse_natural_schedule("every weekday").unwrap(),
            "0 0 * * 1-5"
        );
        assert_eq!(
            parse_natural_schedule("weekdays at 8am").unwrap(),
            "0 8 * * 1-5"
        );
    }

    #[test]


    fn test_weekends() {
        assert_eq!(parse_natural_schedule("weekends").unwrap(), "0 0 * * 0,6");
        assert_eq!(
            parse_natural_schedule("every weekend").unwrap(),
            "0 0 * * 0,6"
        );
    }

    #[test]


    fn test_combined() {
        assert_eq!(
            parse_natural_schedule("every day at 9am").unwrap(),
            "0 9 * * *"
        );
        assert_eq!(
            parse_natural_schedule("every day at 6:30pm").unwrap(),
            "30 18 * * *"
        );
    }

    #[test]


    fn test_hour_range() {
        assert_eq!(
            parse_natural_schedule("every hour from 9 to 17").unwrap(),
            "0 9-17 * * *"
        );
    }

    #[test]


    fn test_business_hours() {
        assert_eq!(
            parse_natural_schedule("business hours").unwrap(),
            "0 9-17 * * 1-5"
        );
        assert_eq!(
            parse_natural_schedule("every 30 minutes during business hours").unwrap(),
            "*/30 9-17 * * 1-5"
        );
        assert_eq!(
            parse_natural_schedule("every hour during business hours").unwrap(),
            "0 9-17 * * 1-5"
        );
    }

    #[test]


    fn test_raw_cron_passthrough() {
        assert_eq!(parse_natural_schedule("0 * * * *").unwrap(), "0 * * * *");
        assert_eq!(
            parse_natural_schedule("*/5 * * * *").unwrap(),
            "*/5 * * * *"
        );
        assert_eq!(
            parse_natural_schedule("0 9-17 * * 1-5").unwrap(),
            "0 9-17 * * 1-5"
        );
    }

    #[test]


    fn test_invalid_input() {
        assert!(parse_natural_schedule("potato salad").is_err());
        assert!(parse_natural_schedule("every 100 minutes").is_err());
    }