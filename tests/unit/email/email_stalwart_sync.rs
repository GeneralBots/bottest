


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use botserver::email::stalwart_client::RuleAction;
use botserver::email::stalwart_client::RuleCondition;



    #[test]


    fn test_new_distribution_list() {
        let list = NewDistributionList {
            bot_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            name: "Test List".to_string(),
            email_alias: "test@example.com".to_string(),
            description: Some("A test list".to_string()),
            members: vec![
                "user1@example.com".to_string(),
                "user2@example.com".to_string(),
            ],
        };

        assert_eq!(list.name, "Test List");
        assert_eq!(list.members.len(), 2);
    }

    #[test]


    fn test_new_auto_responder() {
        let responder = NewAutoResponder {
            bot_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            subject: "Out of Office".to_string(),
            body_html: "<p>I am away</p>".to_string(),
            body_plain: Some("I am away".to_string()),
            start_date: Some(Utc::now()),
            end_date: None,
            only_contacts: false,
        };

        assert_eq!(responder.subject, "Out of Office");
    }

    #[test]


    fn test_new_email_rule() {
        let rule = NewEmailRule {
            bot_id: Uuid::new_v4(),
            user_id: Uuid::new_v4(),
            name: "Move newsletters".to_string(),
            priority: 10,
            conditions: vec![RuleCondition {
                field: "from".to_string(),
                operator: "contains".to_string(),
                value: "newsletter".to_string(),
                header_name: None,
                case_sensitive: false,
            }],
            actions: vec![RuleAction {
                action_type: "move".to_string(),
                value: "Newsletters".to_string(),
            }],
            stop_processing: true,
        };

        assert_eq!(rule.name, "Move newsletters");
        assert_eq!(rule.conditions.len(), 1);
        assert_eq!(rule.actions.len(), 1);
    }

    #[test]


    fn test_distribution_list_dto() {
        let dto = DistributionListDto {
            id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            owner_id: Uuid::new_v4(),
            name: "Sales Team".to_string(),
            email_alias: Some("sales@example.com".to_string()),
            description: Some("Sales distribution list".to_string()),
            members: vec!["alice@example.com".to_string()],
            is_public: false,
            stalwart_principal_id: Some("123".to_string()),
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        assert_eq!(dto.name, "Sales Team");
        assert!(dto.stalwart_principal_id.is_some());
    }