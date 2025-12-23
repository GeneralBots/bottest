


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_tls_config_default() {
        let config = TlsConfig::default();
        assert!(config.enabled);
        assert_eq!(config.min_tls_version, Some("1.3".to_string()));
        assert!(!config.require_client_cert);
    }

    #[test]


    fn test_service_tls_config() {
        let config = ServiceTlsConfig::new("test-service", 8443).with_mtls();

        assert_eq!(config.service_name, "test-service");
        assert_eq!(config.port, 8443);
        assert!(config.tls_config.require_client_cert);
    }

    #[test]


    fn test_tls_registry() {
        let mut registry = TlsRegistry::new();
        registry.register_defaults();

        assert!(!registry.services().is_empty());


        let service_names: Vec<&str> = registry
            .services()
            .iter()
            .map(|s| s.service_name.as_str())
            .collect();

        assert!(service_names.contains(&"api"));
        assert!(service_names.contains(&"llm"));
        assert!(service_names.contains(&"embedding"));
    }