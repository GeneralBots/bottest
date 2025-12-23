//! Unit tests migrated from src/email/stalwart_client.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use botserver::email::stalwart_client::AccountUpdate;
use botserver::email::stalwart_client::AutoResponderConfig;
use botserver::email::stalwart_client::DeliveryStatus;
use botserver::email::stalwart_client::EmailRule;
use chrono::NaiveDate;
use botserver::email::stalwart_client::RuleAction;
use botserver::email::stalwart_client::RuleCondition;
use botserver::email::stalwart_client::StalwartClient;
use serde_json;

// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_generate_vacation_sieve_basic() {
        let client = StalwartClient::new("http://localhost", "test");
        let config = AutoResponderConfig {
            enabled: true,
            subject: "Out of Office".to_string(),
            body_plain: "I am away.".to_string(),
            body_html: None,
            start_date: None,
            end_date: None,
            only_contacts: false,
            vacation_days: 1,
        };

        let sieve = client.generate_vacation_sieve(&config);
        assert!(sieve.contains("require"));
        assert!(sieve.contains("vacation"));
        assert!(sieve.contains("Out of Office"));
        assert!(sieve.contains("I am away."));
    }

    #[test]

    
    fn test_generate_vacation_sieve_with_dates() {
        let client = StalwartClient::new("http://localhost", "test");
        let config = AutoResponderConfig {
            enabled: true,
            subject: "Vacation".to_string(),
            body_plain: "On vacation.".to_string(),
            body_html: None,
            start_date: Some(NaiveDate::from_ymd_opt(2024, 12, 20).expect("valid date")),
            end_date: Some(NaiveDate::from_ymd_opt(2024, 12, 31).expect("valid date")),
            only_contacts: false,
            vacation_days: 7,
        };

        let sieve = client.generate_vacation_sieve(&config);
        assert!(sieve.contains("2024-12-20"));
        assert!(sieve.contains("2024-12-31"));
        assert!(sieve.contains(":days 7"));
    }

    #[test]

    
    fn test_generate_filter_sieve_move_rule() {
        let client = StalwartClient::new("http://localhost", "test");
        let rule = EmailRule {
            id: "rule1".to_string(),
            name: "Move newsletters".to_string(),
            priority: 0,
            enabled: true,
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

        let sieve = client.generate_filter_sieve(&rule);
        assert!(sieve.contains("fileinto"));
        assert!(sieve.contains("Newsletters"));
        assert!(sieve.contains("From"));
        assert!(sieve.contains("newsletter"));
        assert!(sieve.contains("stop"));
    }

    #[test]

    
    fn test_generate_filter_sieve_disabled() {
        let client = StalwartClient::new("http://localhost", "test");
        let rule = EmailRule {
            id: "rule2".to_string(),
            name: "Disabled rule".to_string(),
            priority: 0,
            enabled: false,
            conditions: vec![],
            actions: vec![],
            stop_processing: false,
        };

        let sieve = client.generate_filter_sieve(&rule);
        assert!(sieve.contains("DISABLED"));
    }

    #[test]
    fn test_account_update_builders() {
        let set = AccountUpdate::set("description", "New description");
        assert_eq!(set.action, "set");
        assert_eq!(set.field, "description");

        let add = AccountUpdate::add_item("members", "user@example.com");
        assert_eq!(add.action, "addItem");

        let remove = AccountUpdate::remove_item("members", "old@example.com");
        assert_eq!(remove.action, "removeItem");

        let clear = AccountUpdate::clear("members");
        assert_eq!(clear.action, "clear");
    }

    #[test]
    fn test_delivery_status_deserialize() {
        let json = r#""pending""#;
        let status: DeliveryStatus = serde_json::from_str(json).expect("deserialize");
        assert_eq!(status, DeliveryStatus::Pending);

        let json = r#""unknown_status""#;
        let status: DeliveryStatus = serde_json::from_str(json).expect("deserialize");
        assert_eq!(status, DeliveryStatus::Unknown);
    }
