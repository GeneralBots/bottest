//! Unit tests migrated from src/basic/keywords/episodic_memory.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;

// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_default_config() {
        let config = EpisodicMemoryConfig::default();
        assert!(config.enabled);
        assert_eq!(config.threshold, 4);
        assert_eq!(config.history, 2);
        assert_eq!(config.max_episodes, 100);
    }

    #[test]

    
    fn test_should_summarize() {
        let manager = EpisodicMemoryManager::new(EpisodicMemoryConfig {
            enabled: true,
            threshold: 4,
            history: 2,
            auto_summarize: true,
            ..Default::default()
        });

        assert!(!manager.should_summarize(2));
        assert!(manager.should_summarize(4));
        assert!(manager.should_summarize(10));
    }

    #[test]

    
    fn test_extract_json() {
        // Test with code block
        let response = "Here's the summary:\n```json\n{\"summary\": \"test\"}\n```\n";
        assert!(extract_json(response).is_ok());

        // Test with raw JSON
        let response = "The result is {\"summary\": \"test\"}";
        assert!(extract_json(response).is_ok());
    }

    #[test]

    
    fn test_generate_summary_prompt() {
        let manager = EpisodicMemoryManager::new(EpisodicMemoryConfig::default());
        let messages = vec![ConversationMessage {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "Hello".to_string(),
            timestamp: Utc::now(),
        }];

        let prompt = manager.generate_summary_prompt(&messages);
        assert!(prompt.contains("CONVERSATION:"));
        assert!(prompt.contains("Hello"));
    }

    #[test]

    
    fn test_parse_summary_response() {
        let manager = EpisodicMemoryManager::new(EpisodicMemoryConfig::default());
        let response = r#"{
            "summary": "User asked about billing",
            "key_topics": ["billing", "payment"],
            "decisions": [],
            "action_items": [],
            "sentiment": {"score": 0.5, "label": "positive", "confidence": 0.8},
            "resolution": "resolved"
        }"#;

        let messages = vec![ConversationMessage {
            id: Uuid::new_v4(),
            role: "user".to_string(),
            content: "What's my balance?".to_string(),
            timestamp: Utc::now(),
        }];

        let episode = manager.parse_summary_response(
            response,
            &messages,
            Uuid::new_v4(),
            Uuid::new_v4(),
            Uuid::new_v4(),
        );

        assert!(episode.is_ok());
        let ep = episode.unwrap();
        assert_eq!(ep.summary, "User asked about billing");
        assert_eq!(ep.key_topics, vec!["billing", "payment"]);
        assert_eq!(ep.resolution, ResolutionStatus::Resolved);
    }

    #[test]

    
    fn test_episode_to_dynamic() {
        let episode = Episode {
            id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            summary: "Test summary".to_string(),
            key_topics: vec!["topic1".to_string()],
            decisions: vec![],
            action_items: vec![],
            sentiment: Sentiment::default(),
            resolution: ResolutionStatus::Resolved,
            message_count: 5,
            message_ids: vec![],
            created_at: Utc::now(),
            conversation_start: Utc::now(),
            conversation_end: Utc::now(),
            metadata: serde_json::json!({}),
        };

        let dynamic = episode.to_dynamic();
        assert!(dynamic.is::<Map>());
    }