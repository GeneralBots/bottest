//! Unit tests migrated from src/security/ca.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver
    use tempfile::TempDir;

    #[test]

    
    fn test_ca_config_default() {
        let config = CaConfig::default();
        assert_eq!(config.validity_days, 365);
        assert_eq!(config.key_size, 4096);
        assert!(!config.external_ca_enabled);
    }

    #[test]

    
    fn test_ca_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let mut config = CaConfig::default();
        config.ca_cert_path = temp_dir.path().join("ca.crt");
        config.ca_key_path = temp_dir.path().join("ca.key");

        let manager = CaManager::new(config);
        assert!(manager.is_ok());
    }