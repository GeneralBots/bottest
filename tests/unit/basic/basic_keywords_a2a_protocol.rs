


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;



    #[test]


    fn test_a2a_message_creation() {
        let msg = A2AMessage::new(
            "bot_a",
            Some("bot_b"),
            A2AMessageType::Request,
            serde_json::json!({"test": "data"}),
            Uuid::new_v4(),
        );

        assert_eq!(msg.from_agent, "bot_a");
        assert_eq!(msg.to_agent, Some("bot_b".to_string()));
        assert_eq!(msg.message_type, A2AMessageType::Request);
        assert_eq!(msg.hop_count, 0);
    }

    #[test]


    fn test_a2a_message_response() {
        let original = A2AMessage::new(
            "bot_a",
            Some("bot_b"),
            A2AMessageType::Request,
            serde_json::json!({"question": "test"}),
            Uuid::new_v4(),
        );

        let response = original.create_response("bot_b", serde_json::json!({"answer": "result"}));

        assert_eq!(response.from_agent, "bot_b");
        assert_eq!(response.to_agent, Some("bot_a".to_string()));
        assert_eq!(response.message_type, A2AMessageType::Response);
        assert_eq!(response.correlation_id, original.correlation_id);
        assert_eq!(response.hop_count, 1);
    }

    #[test]


    fn test_message_type_display() {
        assert_eq!(A2AMessageType::Request.to_string(), "request");
        assert_eq!(A2AMessageType::Response.to_string(), "response");
        assert_eq!(A2AMessageType::Broadcast.to_string(), "broadcast");
        assert_eq!(A2AMessageType::Delegate.to_string(), "delegate");
    }

    #[test]


    fn test_message_type_from_str() {
        assert_eq!(A2AMessageType::from("request"), A2AMessageType::Request);
        assert_eq!(A2AMessageType::from("RESPONSE"), A2AMessageType::Response);
        assert_eq!(A2AMessageType::from("unknown"), A2AMessageType::Request);
    }

    #[test]


    fn test_a2a_config_default() {
        let config = A2AConfig::default();
        assert!(config.enabled);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.max_hops, 5);
        assert_eq!(config.protocol_version, "1.0");
    }

    #[test]


    fn test_message_not_expired() {
        let msg = A2AMessage::new(
            "bot_a",
            Some("bot_b"),
            A2AMessageType::Request,
            serde_json::json!({}),
            Uuid::new_v4(),
        );

        assert!(!msg.is_expired());
    }

    #[test]


    fn test_max_hops_not_exceeded() {
        let msg = A2AMessage::new(
            "bot_a",
            Some("bot_b"),
            A2AMessageType::Request,
            serde_json::json!({}),
            Uuid::new_v4(),
        );

        assert!(!msg.max_hops_exceeded(5));
    }

    #[test]


    fn test_max_hops_exceeded() {
        let mut msg = A2AMessage::new(
            "bot_a",
            Some("bot_b"),
            A2AMessageType::Request,
            serde_json::json!({}),
            Uuid::new_v4(),
        );
        msg.hop_count = 5;

        assert!(msg.max_hops_exceeded(5));
    }