


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_mtls_config_default() {
        let config = MtlsConfig::default();
        assert!(!config.enabled);
        assert!(config.ca_cert.is_none());
        assert!(config.client_cert.is_none());
        assert!(config.client_key.is_none());
    }

    #[test]


    fn test_mtls_config_new() {
        let config = MtlsConfig::new(
            Some("ca_cert".to_string()),
            Some("client_cert".to_string()),
            Some("client_key".to_string()),
        );
        assert!(config.enabled);
        assert!(config.is_configured());
    }

    #[test]


    fn test_mtls_config_partial() {
        let config = MtlsConfig::new(Some("ca_cert".to_string()), None, None);
        assert!(!config.enabled);
        assert!(!config.is_configured());
    }

    #[test]


    fn test_mtls_manager_validation() {
        let config = MtlsConfig {
            enabled: true,
            ca_cert: Some(
                "-----BEGIN CERTIFICATE-----\ntest\n-----END CERTIFICATE-----".to_string(),
            ),
            client_cert: Some(
                "-----BEGIN CERTIFICATE-----\ntest\n-----END CERTIFICATE-----".to_string(),
            ),
            client_key: Some(
                "-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----".to_string(),
            ),
        };
        let manager = MtlsManager::new(config);
        assert!(manager.validate().is_ok());
    }

    #[test]


    fn test_mtls_manager_invalid_cert() {
        let config = MtlsConfig {
            enabled: true,
            ca_cert: Some("invalid".to_string()),
            client_cert: Some(
                "-----BEGIN CERTIFICATE-----\ntest\n-----END CERTIFICATE-----".to_string(),
            ),
            client_key: Some(
                "-----BEGIN PRIVATE KEY-----\ntest\n-----END PRIVATE KEY-----".to_string(),
            ),
        };
        let manager = MtlsManager::new(config);
        assert!(manager.validate().is_err());
    }