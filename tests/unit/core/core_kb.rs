//! Unit tests migrated from src/core/kb/mod.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_kb_manager_creation() {
        let temp_dir = TempDir::new().unwrap();
        let manager = KnowledgeBaseManager::new(temp_dir.path());

        // Test that manager is created successfully
        assert!(manager.processor.chunk_size() == 1000);
        assert!(manager.processor.chunk_overlap() == 200);
    }

    #[test]

    
    fn test_collection_naming() {
        let bot_name = "testbot";
        let kb_name = "docs";
        let collection_name = format!("{}_{}", bot_name, kb_name);
        assert_eq!(collection_name, "testbot_docs");
    }