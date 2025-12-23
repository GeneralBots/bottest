//! Unit tests migrated from src/basic/keywords/transfer_to_human.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_priority_to_int() {
        assert_eq!(priority_to_int(Some("urgent")), 3);
        assert_eq!(priority_to_int(Some("high")), 2);
        assert_eq!(priority_to_int(Some("normal")), 1);
        assert_eq!(priority_to_int(Some("low")), 0);
        assert_eq!(priority_to_int(None), 1);
    }

    #[test]

    
    fn test_find_attendant_by_name() {
        let attendants = vec![
            Attendant {
                id: "att-001".to_string(),
                name: "John Smith".to_string(),
                channel: "all".to_string(),
                preferences: "sales".to_string(),
                department: Some("commercial".to_string()),
                aliases: vec!["johnny".to_string(), "js".to_string()],
                status: AttendantStatus::Online,
            },
            Attendant {
                id: "att-002".to_string(),
                name: "Jane Doe".to_string(),
                channel: "web".to_string(),
                preferences: "support".to_string(),
                department: Some("customer-service".to_string()),
                aliases: vec![],
                status: AttendantStatus::Online,
            },
        ];

        // Find by exact name
        let found = find_attendant(&attendants, Some("John Smith"), None);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "att-001");

        // Find by partial name
        let found = find_attendant(&attendants, Some("john"), None);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "att-001");

        // Find by alias
        let found = find_attendant(&attendants, Some("johnny"), None);
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "att-001");

        // Find by department
        let found = find_attendant(&attendants, None, Some("customer-service"));
        assert!(found.is_some());
        assert_eq!(found.unwrap().id, "att-002");
    }

    #[test]

    
    fn test_transfer_result_to_dynamic() {
        let result = TransferResult {
            success: true,
            status: TransferStatus::Assigned,
            queue_position: Some(1),
            assigned_to: Some("att-001".to_string()),
            assigned_to_name: Some("John Smith".to_string()),
            estimated_wait_seconds: Some(30),
            message: "Connected to John".to_string(),
        };

        let dynamic = result.to_dynamic();
        let map = dynamic.try_cast::<Map>().unwrap();

        assert_eq!(
            map.get("success")
                .unwrap()
                .clone()
                .try_cast::<bool>()
                .unwrap(),
            true
        );
        assert_eq!(
            map.get("assigned_to_name")
                .unwrap()
                .clone()
                .try_cast::<String>()
                .unwrap(),
            "John Smith"
        );
    }