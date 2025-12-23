


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_normalize_phone_us_10_digit() {
        assert_eq!(normalize_phone_number("5551234567"), "+15551234567");
    }

    #[test]


    fn test_normalize_phone_us_11_digit() {
        assert_eq!(normalize_phone_number("15551234567"), "+15551234567");
    }

    #[test]


    fn test_normalize_phone_with_plus() {
        assert_eq!(normalize_phone_number("+15551234567"), "+15551234567");
    }

    #[test]


    fn test_normalize_phone_with_formatting() {
        assert_eq!(normalize_phone_number("+1 (555) 123-4567"), "+15551234567");
    }

    #[test]


    fn test_normalize_phone_international() {
        assert_eq!(normalize_phone_number("+44 7911 123456"), "+447911123456");
    }

    #[test]


    fn test_sms_provider_from_str() {
        assert_eq!(SmsProvider::from("twilio"), SmsProvider::Twilio);
        assert_eq!(SmsProvider::from("aws_sns"), SmsProvider::AwsSns);
        assert_eq!(SmsProvider::from("vonage"), SmsProvider::Vonage);
        assert_eq!(SmsProvider::from("nexmo"), SmsProvider::Vonage);
        assert_eq!(SmsProvider::from("messagebird"), SmsProvider::MessageBird);
        assert_eq!(
            SmsProvider::from("custom"),
            SmsProvider::Custom("custom".to_string())
        );
    }