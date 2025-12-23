//! Unit tests migrated from src/basic/keywords/knowledge_graph.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;

// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_default_config() {
        let config = KnowledgeGraphConfig::default();
        assert!(config.enabled);
        assert_eq!(config.backend, "postgresql");
        assert!(config.entity_types.contains(&"person".to_string()));
    }

    #[test]

    
    fn test_extraction_prompt() {
        let manager = KnowledgeGraphManager::new(KnowledgeGraphConfig::default());
        let prompt = manager.generate_extraction_prompt("John works at Acme Corp.");
        assert!(prompt.contains("John works at Acme Corp."));
        assert!(prompt.contains("ENTITY TYPES TO EXTRACT"));
    }

    #[test]

    
    fn test_parse_extraction_response() {
        let manager = KnowledgeGraphManager::new(KnowledgeGraphConfig::default());
        let response = r#"{
            "entities": [
                {
                    "name": "John",
                    "canonical_name": "John Smith",
                    "entity_type": "person",
                    "confidence": 0.9,
                    "properties": {}
                }
            ],
            "relationships": [
                {
                    "from_entity": "John",
                    "to_entity": "Acme Corp",
                    "relationship_type": "works_on",
                    "confidence": 0.85,
                    "evidence": "John works at Acme Corp"
                }
            ]
        }"#;

        let result = manager.parse_extraction_response(response, 100, 50);
        assert!(result.is_ok());
        let extraction = result.unwrap();
        assert_eq!(extraction.entities.len(), 1);
        assert_eq!(extraction.relationships.len(), 1);
    }

    #[test]

    
    fn test_entity_to_dynamic() {
        let entity = KgEntity {
            id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            entity_type: "person".to_string(),
            entity_name: "John Smith".to_string(),
            aliases: vec!["John".to_string()],
            properties: serde_json::json!({"department": "Sales"}),
            confidence: 0.95,
            source: EntitySource::Manual,
            created_at: Utc::now(),
            updated_at: Utc::now(),
        };

        let dynamic = entity.to_dynamic();
        assert!(dynamic.is::<Map>());
    }

    #[test]

    
    fn test_is_valid_entity_type() {
        let manager = KnowledgeGraphManager::new(KnowledgeGraphConfig::default());
        assert!(manager.is_valid_entity_type("person"));
        assert!(manager.is_valid_entity_type("PERSON"));
        assert!(manager.is_valid_entity_type("organization"));
        assert!(!manager.is_valid_entity_type("unknown_type"));
    }

    #[test]

    
    fn test_json_to_dynamic() {
        let json = serde_json::json!({
            "name": "test",
            "count": 42,
            "active": true,
            "tags": ["a", "b"]
        });

        let dynamic = json_to_dynamic(&json);
        assert!(dynamic.is::<Map>());
    }