


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;



    #[test]


    fn test_mock_channel_adapter_creation() {
        let adapter = MockChannelAdapter::new("test");
        assert_eq!(adapter.name(), "test");
        assert!(adapter.is_configured());
    }

    #[cfg(feature = "llm")]
    #[test]

    fn test_mock_llm_provider_creation() {
        let provider = MockLLMProvider::new();
        assert_eq!(provider.response, "Mock LLM response");

        let custom = MockLLMProvider::with_response("Custom response");
        assert_eq!(custom.response, "Custom response");
    }

    #[test]


    fn test_builder_defaults() {
        let builder = TestAppStateBuilder::new();
        assert_eq!(builder.bucket_name, "test-bucket");
        assert!(builder.database_url.is_none());
        assert!(builder.config.is_none());
    }

    #[cfg(feature = "llm")]
    #[tokio::test]
    async fn test_mock_llm_generate() {
        let provider = MockLLMProvider::with_response("Test output");
        let result = provider
            .generate("test prompt", &serde_json::json!({}), "model", "key")
            .await;
        assert!(result.is_ok());
        assert_eq!(result.unwrap(), "Test output");
    }

    #[tokio::test]
    async fn test_mock_channel_send_message() {
        let adapter = MockChannelAdapter::new("test_channel");
        let response = BotResponse {
            session_id: "sess-1".to_string(),
            user_id: "user-1".to_string(),
            content: "Hello".to_string(),
            channel: "test".to_string(),
            ..Default::default()
        };

        let result = adapter.send_message(response.clone()).await;
        assert!(result.is_ok());

        let messages = adapter.get_sent_messages().await;
        assert_eq!(messages.len(), 1);
        assert_eq!(messages[0].content, "Hello");
    }