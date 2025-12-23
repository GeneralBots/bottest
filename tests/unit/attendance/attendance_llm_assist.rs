


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_config_defaults() {
        let config = LlmAssistConfig::default();
        assert!(!config.tips_enabled);
        assert!(!config.polish_enabled);
        assert!(!config.any_enabled());
    }

    #[test]


    fn test_fallback_tips_urgent() {
        let tips = generate_fallback_tips("This is URGENT! I need help immediately!");
        assert!(!tips.is_empty());
        assert!(tips.iter().any(|t| matches!(t.tip_type, TipType::Warning)));
    }

    #[test]


    fn test_fallback_tips_question() {
        let tips = generate_fallback_tips("How do I reset my password?");
        assert!(!tips.is_empty());
        assert!(tips.iter().any(|t| matches!(t.tip_type, TipType::Intent)));
    }

    #[test]


    fn test_sentiment_positive() {
        let sentiment = analyze_sentiment_keywords("Thank you so much! This is great!");
        assert_eq!(sentiment.overall, "positive");
        assert!(sentiment.score > 0.0);
        assert_eq!(sentiment.escalation_risk, "low");
    }

    #[test]


    fn test_sentiment_negative() {
        let sentiment =
            analyze_sentiment_keywords("This is terrible! I'm very frustrated with this problem.");
        assert_eq!(sentiment.overall, "negative");
        assert!(sentiment.score < 0.0);
        assert!(sentiment.escalation_risk == "medium" || sentiment.escalation_risk == "high");
    }

    #[test]


    fn test_sentiment_urgent() {
        let sentiment = analyze_sentiment_keywords("I need help ASAP! This is urgent!");
        assert!(sentiment.urgency == "high" || sentiment.urgency == "urgent");
    }

    #[test]


    fn test_extract_json() {
        let response = "Here is the result: {\"key\": \"value\"} and some more text.";
        let json = extract_json(response);
        assert_eq!(json, "{\"key\": \"value\"}");
    }

    #[test]


    fn test_fallback_replies() {
        let replies = generate_fallback_replies();
        assert_eq!(replies.len(), 3);
        assert!(replies.iter().any(|r| r.category == "greeting"));
        assert!(replies.iter().any(|r| r.category == "follow_up"));
    }

    #[test]


    fn test_help_text() {
        let help = get_help_text();
        assert!(help.contains("/queue"));
        assert!(help.contains("/tips"));
        assert!(help.contains("/polish"));
    }