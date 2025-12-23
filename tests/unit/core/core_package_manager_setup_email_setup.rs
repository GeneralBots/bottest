//! Unit tests migrated from src/core/package_manager/setup/email_setup.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_email_setup_creation() {
        let setup = EmailSetup::new(
            "http://localhost:8080".to_string(),
            PathBuf::from("/tmp/email_config.json"),
        );
        assert_eq!(setup.base_url, "http://localhost:8080");
    }

    #[tokio::test]
    async fn test_generate_config() {
        let config_path = std::env::temp_dir().join("email_test_config.toml");
        let data_path = std::env::temp_dir().join("email_data");

        generate_email_config(config_path.clone(), data_path, false)
            .await
            .unwrap();

        assert!(config_path.exists());

        // Cleanup
        let _ = std::fs::remove_file(config_path);
    }