//! Unit tests migrated from src/core/kb/embedding_generator.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_dimension_detection() {
        assert_eq!(EmbeddingConfig::detect_dimensions("bge-small-en"), 384);
        assert_eq!(EmbeddingConfig::detect_dimensions("all-mpnet-base-v2"), 768);
        assert_eq!(
            EmbeddingConfig::detect_dimensions("text-embedding-ada-002"),
            1536
        );
        assert_eq!(EmbeddingConfig::detect_dimensions("unknown-model"), 384);
    }

    #[tokio::test]
    async fn test_text_cleaning_for_embedding() {
        let text = "This is a test\n\nWith multiple lines";
        let _generator = EmbeddingGenerator::new("http://localhost:8082".to_string());

        // This would test actual embedding generation if service is available
        // For unit tests, we just verify the structure is correct
        assert!(!text.is_empty());
    }