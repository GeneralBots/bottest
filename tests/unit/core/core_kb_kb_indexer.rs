


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_collection_name_generation() {
        let bot_name = "mybot";
        let kb_name = "docs";
        let collection_name = format!("{}_{}", bot_name, kb_name);
        assert_eq!(collection_name, "mybot_docs");
    }

    #[test]


    fn test_qdrant_point_creation() {
        let chunk = TextChunk {
            content: "Test content".to_string(),
            metadata: super::super::document_processor::ChunkMetadata {
                document_path: "test.txt".to_string(),
                document_title: Some("Test".to_string()),
                chunk_index: 0,
                total_chunks: 1,
                start_char: 0,
                end_char: 12,
                page_number: None,
            },
        };

        let embedding = Embedding {
            vector: vec![0.1, 0.2, 0.3],
            dimensions: 3,
            model: "test".to_string(),
            tokens_used: None,
        };

        let indexer = KbIndexer::new(EmbeddingConfig::default(), QdrantConfig::default());

        let points = indexer
            .create_qdrant_points("test.txt", vec![(chunk, embedding)])
            .unwrap();

        assert_eq!(points.len(), 1);
        assert_eq!(points[0].vector.len(), 3);
        assert!(points[0].payload.contains_key("content"));
    }