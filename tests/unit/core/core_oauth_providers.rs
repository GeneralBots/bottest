//! Unit tests migrated from src/core/oauth/providers.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_build_auth_url() {
        let config = OAuthConfig::new(
            OAuthProvider::Google,
            "test_client_id".to_string(),
            "test_secret".to_string(),
            "http://localhost:8300/callback".to_string(),
        );

        let url = OAuthProvider::Google.build_auth_url(&config, "test_state");

        assert!(url.starts_with("https://accounts.google.com/o/oauth2/v2/auth?"));
        assert!(url.contains("client_id=test_client_id"));
        assert!(url.contains("state=test_state"));
        assert!(url.contains("response_type=code"));
    }

    #[test]

    
    fn test_load_oauth_config() {
        let mut bot_config = HashMap::new();
        bot_config.insert("oauth-google-enabled".to_string(), "true".to_string());
        bot_config.insert(
            "oauth-google-client-id".to_string(),
            "my_client_id".to_string(),
        );
        bot_config.insert(
            "oauth-google-client-secret".to_string(),
            "my_secret".to_string(),
        );

        let config = load_oauth_config(OAuthProvider::Google, &bot_config, "http://localhost:8300");

        assert!(config.is_some());
        let config = config.unwrap();
        assert_eq!(config.client_id, "my_client_id");
        assert!(config.redirect_uri.contains("/auth/oauth/google/callback"));
    }

    #[test]

    
    fn test_disabled_provider() {
        let mut bot_config = HashMap::new();
        bot_config.insert("oauth-google-enabled".to_string(), "false".to_string());
        bot_config.insert(
            "oauth-google-client-id".to_string(),
            "my_client_id".to_string(),
        );
        bot_config.insert(
            "oauth-google-client-secret".to_string(),
            "my_secret".to_string(),
        );

        let config = load_oauth_config(OAuthProvider::Google, &bot_config, "http://localhost:8300");

        assert!(config.is_none());
    }