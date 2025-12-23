


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_default_config() {
        let config = SseConfig::default();
        assert!(config.enabled);
        assert_eq!(config.heartbeat_seconds, 30);
        assert_eq!(config.max_connections, 1000);
    }

    #[test]


    fn test_can_accept_connection() {
        let config = SseConfig::default();
        assert!(config.can_accept_connection(0));
        assert!(config.can_accept_connection(999));
        assert!(!config.can_accept_connection(1000));
        assert!(!config.can_accept_connection(1001));
    }

    #[test]


    fn test_can_accept_connection_disabled() {
        let config = SseConfig {
            enabled: false,
            ..Default::default()
        };
        assert!(!config.can_accept_connection(0));
    }

    #[test]


    fn test_heartbeat_duration() {
        let config = SseConfig {
            heartbeat_seconds: 45,
            ..Default::default()
        };
        assert_eq!(
            config.heartbeat_duration(),
            std::time::Duration::from_secs(45)
        );
    }