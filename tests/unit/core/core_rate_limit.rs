


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[tokio::test]
    async fn test_keyed_rate_limiter() {
        let limiter = KeyedRateLimiter::new(2, 2);


        assert!(limiter.check("test_ip").await);
        assert!(limiter.check("test_ip").await);


        assert!(!limiter.check("test_ip").await);


        assert!(limiter.check("other_ip").await);
    }

    #[test]


    fn test_rate_limit_config_default() {
        let config = RateLimitConfig::default();
        assert_eq!(config.api_rps, 100);
        assert_eq!(config.auth_rps, 10);
        assert_eq!(config.llm_rps, 5);
        assert!(config.enabled);
    }

    #[test]


    fn test_get_limiter_type() {
        assert!(matches!(get_limiter_type("/api/users"), LimiterType::Api));
        assert!(matches!(get_limiter_type("/auth/login"), LimiterType::Auth));
        assert!(matches!(
            get_limiter_type("/api/llm/chat"),
            LimiterType::Llm
        ));
        assert!(matches!(
            get_limiter_type("/api/chat/send"),
            LimiterType::Llm
        ));
    }