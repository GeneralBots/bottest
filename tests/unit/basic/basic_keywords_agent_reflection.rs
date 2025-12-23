


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_reflection_type_from_str() {
        assert_eq!(
            ReflectionType::from("conversation_quality"),
            ReflectionType::ConversationQuality
        );
        assert_eq!(
            ReflectionType::from("quality"),
            ReflectionType::ConversationQuality
        );
        assert_eq!(
            ReflectionType::from("tool_usage"),
            ReflectionType::ToolUsage
        );
        assert_eq!(
            ReflectionType::from("performance"),
            ReflectionType::Performance
        );
    }

    #[test]


    fn test_reflection_config_default() {
        let config = ReflectionConfig::default();
        assert!(!config.enabled);
        assert_eq!(config.interval, 10);
        assert!(!config.auto_apply);
    }

    #[test]


    fn test_reflection_result_new() {
        let bot_id = Uuid::new_v4();
        let session_id = Uuid::new_v4();
        let result = ReflectionResult::new(bot_id, session_id, ReflectionType::ConversationQuality);

        assert_eq!(result.bot_id, bot_id);
        assert_eq!(result.session_id, session_id);
        assert_eq!(result.score, 0.0);
        assert!(result.insights.is_empty());
    }

    #[test]


    fn test_reflection_result_from_json() {
        let json_response = r#"{
            "score": 7.5,
            "key_insights": ["Users prefer concise responses", "Technical questions need more detail"],
            "improvements": ["Add more examples", "Improve response time"],
            "positive_patterns": ["Good greeting", "Clear explanations"]
        }"#;

        let result = ReflectionResult::from_llm_response(
            Uuid::new_v4(),
            Uuid::new_v4(),
            ReflectionType::ConversationQuality,
            json_response,
            10,
        );

        assert_eq!(result.score, 7.5);
        assert_eq!(result.insights.len(), 2);
        assert_eq!(result.improvements.len(), 2);
        assert_eq!(result.positive_patterns.len(), 2);
    }

    #[test]


    fn test_reflection_result_needs_improvement() {
        let mut result =
            ReflectionResult::new(Uuid::new_v4(), Uuid::new_v4(), ReflectionType::Performance);

        result.score = 5.0;
        assert!(result.needs_improvement(6.0));

        result.score = 8.0;
        assert!(!result.needs_improvement(6.0));
    }

    #[test]


    fn test_extract_insights_from_text() {
        let text = "Here are some insights:\n\
                    1. Users prefer short responses\n\
                    2. Technical questions need examples\n\
                    - Consider adding more context\n\
                    • Improve response time";

        let insights = extract_insights_from_text(text);
        assert!(!insights.is_empty());
    }

    #[test]


    fn test_reflection_type_prompt_template() {
        let template = ReflectionType::ConversationQuality.prompt_template();
        assert!(template.contains("{conversation}"));
        assert!(template.contains("JSON format"));
    }

    #[test]


    fn test_reflection_result_summary() {
        let mut result =
            ReflectionResult::new(Uuid::new_v4(), Uuid::new_v4(), ReflectionType::Performance);
        result.score = 7.5;
        result.messages_analyzed = 15;
        result.insights = vec!["Insight 1".to_string(), "Insight 2".to_string()];
        result.improvements = vec!["Improvement 1".to_string()];

        let summary = result.summary();
        assert!(summary.contains("7.5"));
        assert!(summary.contains("15"));
        assert!(summary.contains("2"));
        assert!(summary.contains("1"));
    }