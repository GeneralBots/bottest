


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_with_params() {
        let url = ApiUrls::with_params(ApiUrls::USER_BY_ID, &[("id", "123")]);
        assert_eq!(url, "/api/users/123");
    }

    #[test]


    fn test_with_query() {
        let url = ApiUrls::with_query(ApiUrls::USERS, &[("page", "1"), ("limit", "10")]);
        assert_eq!(url, "/api/users?page=1&limit=10");
    }

    #[test]


    fn test_multiple_params() {
        let url = ApiUrls::with_params(
            ApiUrls::EMAIL_CLICK,
            &[("campaign_id", "camp123"), ("email", "user@example.com")],
        );
        assert_eq!(url, "/api/email/click/camp123/user@example.com");
    }