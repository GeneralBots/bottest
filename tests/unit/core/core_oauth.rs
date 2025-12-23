


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_provider_from_str() {
        assert_eq!(
            OAuthProvider::from_str("google"),
            Some(OAuthProvider::Google)
        );
        assert_eq!(
            OAuthProvider::from_str("DISCORD"),
            Some(OAuthProvider::Discord)
        );
        assert_eq!(
            OAuthProvider::from_str("Twitter"),
            Some(OAuthProvider::Twitter)
        );
        assert_eq!(OAuthProvider::from_str("x"), Some(OAuthProvider::Twitter));
        assert_eq!(OAuthProvider::from_str("invalid"), None);
    }

    #[test]


    fn test_oauth_state_encode_decode() {
        let state = OAuthState::new(OAuthProvider::Google, Some("/dashboard".to_string()));
        let encoded = state.encode();
        let decoded = OAuthState::decode(&encoded).unwrap();

        assert_eq!(decoded.provider, OAuthProvider::Google);
        assert_eq!(decoded.redirect_after, Some("/dashboard".to_string()));
        assert!(!decoded.is_expired());
    }

    #[test]


    fn test_oauth_config_validation() {
        let valid_config = OAuthConfig::new(
            OAuthProvider::Google,
            "client_id".to_string(),
            "client_secret".to_string(),
            "http://localhost/callback".to_string(),
        );
        assert!(valid_config.is_valid());

        let mut invalid_config = valid_config.clone();
        invalid_config.client_id = String::new();
        assert!(!invalid_config.is_valid());
    }