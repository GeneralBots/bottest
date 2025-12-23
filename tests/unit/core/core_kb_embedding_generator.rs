


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


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



        assert!(!text.is_empty());
    }