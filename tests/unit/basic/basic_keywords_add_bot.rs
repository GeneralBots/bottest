//! Unit tests migrated from src/basic/keywords/add_bot.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_trigger_from_keywords() {
        let trigger = BotTrigger::from_keywords(vec!["finance".to_string(), "money".to_string()]);
        assert_eq!(trigger.trigger_type, TriggerType::Keyword);
        assert_eq!(trigger.keywords.unwrap().len(), 2);
    }

    #[test]

    
    fn test_match_bot_triggers() {
        let bots = vec![
            SessionBot {
                id: Uuid::new_v4(),
                session_id: Uuid::new_v4(),
                bot_id: Uuid::new_v4(),
                bot_name: "finance-bot".to_string(),
                trigger: BotTrigger::from_keywords(vec!["money".to_string(), "budget".to_string()]),
                priority: 1,
                is_active: true,
            },
            SessionBot {
                id: Uuid::new_v4(),
                session_id: Uuid::new_v4(),
                bot_id: Uuid::new_v4(),
                bot_name: "hr-bot".to_string(),
                trigger: BotTrigger::from_keywords(vec![
                    "vacation".to_string(),
                    "employee".to_string(),
                ]),
                priority: 0,
                is_active: true,
            },
        ];

        let matches = match_bot_triggers("How much money do I have?", &bots);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].bot_name, "finance-bot");

        let matches = match_bot_triggers("I need to request vacation", &bots);
        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].bot_name, "hr-bot");

        let matches = match_bot_triggers("Hello world", &bots);
        assert!(matches.is_empty());
    }

    #[test]

    
    fn test_match_tool_triggers() {
        let bots = vec![SessionBot {
            id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            bot_name: "data-bot".to_string(),
            trigger: BotTrigger::from_tools(vec!["AGGREGATE".to_string(), "CHART".to_string()]),
            priority: 1,
            is_active: true,
        }];

        let matches = match_tool_triggers("aggregate", &bots);
        assert_eq!(matches.len(), 1);

        let matches = match_tool_triggers("SEND", &bots);
        assert!(matches.is_empty());
    }