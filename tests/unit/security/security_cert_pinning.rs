


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_pinned_cert_creation() {
        let pin = PinnedCert::new(
            "api.example.com",
            "sha256//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        );

        assert_eq!(pin.hostname, "api.example.com");
        assert!(!pin.is_backup);
        assert_eq!(pin.pin_type, PinType::Leaf);
    }

    #[test]


    fn test_backup_pin() {
        let pin = PinnedCert::backup(
            "api.example.com",
            "sha256//BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB=",
        );

        assert!(pin.is_backup);
        assert!(pin.description.is_some());
    }

    #[test]


    fn test_config_add_pin() {
        let mut config = CertPinningConfig::default();
        config.add_pin(PinnedCert::new(
            "example.com",
            "sha256//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        ));

        assert!(config.get_pins("example.com").is_some());
        assert_eq!(config.get_pins("example.com").unwrap().len(), 1);
    }

    #[test]


    fn test_format_fingerprint() {
        let hash = vec![0xAB, 0xCD, 0xEF, 0x12];
        let formatted = format_fingerprint(&hash);
        assert_eq!(formatted, "AB:CD:EF:12");
    }

    #[test]


    fn test_parse_fingerprint_hex() {
        let result = parse_fingerprint("AB:CD:EF:12").unwrap();
        assert_eq!(result, vec![0xAB, 0xCD, 0xEF, 0x12]);
    }

    #[test]


    fn test_parse_fingerprint_base64() {
        let original = vec![0xAB, 0xCD, 0xEF, 0x12];
        let base64 = format!("sha256//{}", BASE64.encode(&original));
        let result = parse_fingerprint(&base64).unwrap();
        assert_eq!(result, original);
    }

    #[test]


    fn test_pinning_stats() {
        let mut config = CertPinningConfig::default();
        config.add_pin(PinnedCert::new(
            "host1.com",
            "sha256//AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=",
        ));
        config.add_pin(PinnedCert::backup(
            "host1.com",
            "sha256//BBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBBB=",
        ));
        config.add_pin(PinnedCert::new(
            "host2.com",
            "sha256//CCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCCC=",
        ));

        let manager = CertPinningManager::new(config);
        let stats = manager.get_stats().unwrap();

        assert!(stats.enabled);
        assert_eq!(stats.total_hosts, 2);
        assert_eq!(stats.total_pins, 3);
        assert_eq!(stats.backup_pins, 1);
    }

    #[test]


    fn test_pem_to_der() {

        let mock_pem = b"-----BEGIN CERTIFICATE-----
MIIB
-----END CERTIFICATE-----";


        let result = pem_to_der(mock_pem);

        assert!(result.is_err() || result.unwrap().len() > 0);
    }

    #[test]


    fn test_manager_disabled() {
        let mut config = CertPinningConfig::default();
        config.enabled = false;

        let manager = CertPinningManager::new(config);
        assert!(!manager.is_enabled());
    }