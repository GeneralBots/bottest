//! Unit tests migrated from src/core/bot/manager.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_sanitize_bot_name() {
        let manager = BotManager::new("", "", "", "", PathBuf::new());

        assert_eq!(manager.sanitize_bot_name("My Bot"), "mybot");
        assert_eq!(manager.sanitize_bot_name("test-bot"), "test-bot");
        assert_eq!(manager.sanitize_bot_name("Bot 123"), "bot123");
        assert_eq!(manager.sanitize_bot_name("--invalid--"), "invalid");
        assert_eq!(manager.sanitize_bot_name("my_bot_name"), "my_bot_name");
    }

    #[test]

    
    fn test_bot_config_default() {
        let settings = BotSettings::default();
        assert!(settings.knowledge_bases.is_empty());
        assert!(settings.channels.is_empty());
    }

    #[test]

    
    fn test_bot_status_display() {
        assert_eq!(format!("{}", BotStatus::Active), "Active");
        assert_eq!(format!("{}", BotStatus::Creating), "Creating");
    }

    #[test]

    
    fn test_bot_route_from_config() {
        let config = BotConfig {
            id: Uuid::new_v4(),
            name: "testbot".to_string(),
            display_name: "Test Bot".to_string(),
            org_id: Uuid::new_v4(),
            org_slug: "myorg".to_string(),
            template: None,
            status: BotStatus::Active,
            bucket: "myorg_testbot".to_string(),
            custom_ui: Some("custom".to_string()),
            settings: BotSettings::default(),
            access: BotAccess::default(),
            created_at: Utc::now(),
            updated_at: Utc::now(),
            created_by: Uuid::new_v4(),
        };

        let route = BotRoute::from(&config);
        assert_eq!(route.name, "testbot");
        assert_eq!(route.org_slug, "myorg");
        assert_eq!(route.bucket, "myorg_testbot");
        assert_eq!(route.custom_ui, Some("custom".to_string()));
    }