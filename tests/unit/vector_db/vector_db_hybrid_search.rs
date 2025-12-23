//! Unit tests migrated from src/vector-db/hybrid_search.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;

// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_hybrid_config_default() {
        let config = HybridSearchConfig::default();

        assert_eq!(config.dense_weight, 0.7);
        assert_eq!(config.sparse_weight, 0.3);
        assert!(!config.reranker_enabled);
        assert_eq!(config.max_results, 10);
        assert!(config.bm25_enabled);
    }

    #[test]

    
    fn test_hybrid_config_search_modes() {
        let config = HybridSearchConfig::default();
        assert!(config.use_sparse_search());
        assert!(config.use_dense_search());

        let dense_only = HybridSearchConfig {
            bm25_enabled: false,
            ..Default::default()
        };
        assert!(!dense_only.use_sparse_search());
        assert!(dense_only.use_dense_search());

        let sparse_only = HybridSearchConfig {
            dense_weight: 0.0,
            sparse_weight: 1.0,
            ..Default::default()
        };
        assert!(sparse_only.use_sparse_search());
        assert!(!sparse_only.use_dense_search());
    }

    #[test]

    
    fn test_reciprocal_rank_fusion() {
        let config = HybridSearchConfig::default();
        let engine = HybridSearchEngine::new(config, "http://localhost:6333", "test");

        let sparse = vec![
            ("doc1".to_string(), 0.9),
            ("doc2".to_string(), 0.7),
            ("doc3".to_string(), 0.5),
        ];

        let dense = vec![
            ("doc2".to_string(), 0.95),
            ("doc1".to_string(), 0.8),
            ("doc4".to_string(), 0.6),
        ];

        let fused = engine.reciprocal_rank_fusion(&sparse, &dense);

        assert!(!fused.is_empty());
        // doc1 and doc2 appear in both, should rank high
        let top_ids: Vec<&str> = fused.iter().take(2).map(|(id, _)| id.as_str()).collect();
        assert!(top_ids.contains(&"doc1") || top_ids.contains(&"doc2"));
    }

    #[test]

    
    fn test_query_decomposer_simple() {
        let decomposer = QueryDecomposer::new("http://localhost:8081", "none");

        let rt = tokio::runtime::Runtime::new().unwrap();

        let result = rt.block_on(async {
            decomposer
                .decompose("What is machine learning and how does it work?")
                .await
        });

        assert!(result.is_ok());
        let queries = result.unwrap();
        assert!(!queries.is_empty());
    }

    #[test]

    
    fn test_search_result_serialization() {
        let result = SearchResult {
            doc_id: "test123".to_string(),
            content: "Test content".to_string(),
            source: "/path/to/file".to_string(),
            score: 0.85,
            metadata: HashMap::new(),
            search_method: SearchMethod::Hybrid,
        };

        let json = serde_json::to_string(&result);
        assert!(json.is_ok());

        let parsed: Result<SearchResult, _> = serde_json::from_str(&json.unwrap());
        assert!(parsed.is_ok());
        assert_eq!(parsed.unwrap().doc_id, "test123");
    }

    #[cfg(not(feature = "vectordb"))]
    #[test]
    
    fn test_fallback_bm25_index() {
        let mut index = BM25Index::new();

        index.add_document(
            "doc1",
            "machine learning artificial intelligence",
            "source1",
        );
        index.add_document("doc2", "natural language processing NLP", "source2");
        index.add_document("doc3", "computer vision image recognition", "source3");

        let results = index.search("machine learning", 10);

        assert!(!results.is_empty());
        assert_eq!(results[0].0, "doc1");

        let stats = index.stats();
        assert_eq!(stats.doc_count, 3);
        assert!(stats.enabled);
    }

    #[cfg(not(feature = "vectordb"))]
    #[test]
    
    fn test_fallback_bm25_disabled() {
        let mut index = BM25Index::new();
        index.set_enabled(false);

        index.add_document("doc1", "test content", "source1");
        let results = index.search("test", 10);

        assert!(results.is_empty());
        assert!(!index.stats().enabled);
    }