//! Unit tests migrated from src/vector-db/bm25_config.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_default_config() {
        let config = Bm25Config::default();
        assert!(config.enabled);
        assert!((config.k1 - 1.2).abs() < f32::EPSILON);
        assert!((config.b - 0.75).abs() < f32::EPSILON);
        assert!(config.stemming);
        assert!(config.stopwords);
    }

    #[test]

    
    fn test_disabled_config() {
        let config = Bm25Config::disabled();
        assert!(!config.enabled);
        assert!(!config.is_enabled());
    }

    #[test]

    
    fn test_with_params() {
        let config = Bm25Config::with_params(1.5, 0.5);
        assert!((config.k1 - 1.5).abs() < f32::EPSILON);
        assert!((config.b - 0.5).abs() < f32::EPSILON);
    }

    #[test]

    
    fn test_validation_negative_k1() {
        let mut config = Bm25Config {
            k1: -1.0,
            ..Default::default()
        };
        config.validate();
        assert!((config.k1 - 1.2).abs() < f32::EPSILON);
    }

    #[test]

    
    fn test_validation_high_k1() {
        let mut config = Bm25Config {
            k1: 15.0,
            ..Default::default()
        };
        config.validate();
        assert!((config.k1 - 10.0).abs() < f32::EPSILON);
    }

    #[test]

    
    fn test_validation_b_range() {
        let mut config = Bm25Config {
            b: -0.5,
            ..Default::default()
        };
        config.validate();
        assert!(config.b.abs() < f32::EPSILON);

        let mut config2 = Bm25Config {
            b: 1.5,
            ..Default::default()
        };
        config2.validate();
        assert!((config2.b - 1.0).abs() < f32::EPSILON);
    }

    #[test]

    
    fn test_has_preprocessing() {
        let config = Bm25Config::default();
        assert!(config.has_preprocessing());

        let no_preprocess = Bm25Config {
            stemming: false,
            stopwords: false,
            ..Default::default()
        };
        assert!(!no_preprocess.has_preprocessing());
    }

    #[test]

    
    fn test_describe() {
        let config = Bm25Config::default();
        let desc = config.describe();
        assert!(desc.contains("k1=1.2"));
        assert!(desc.contains("b=0.75"));

        let disabled = Bm25Config::disabled();
        assert_eq!(disabled.describe(), "BM25(disabled)");
    }

    #[test]

    
    fn test_is_stopword() {
        assert!(is_stopword("the"));
        assert!(is_stopword("THE"));
        assert!(is_stopword("and"));
        assert!(is_stopword("is"));
        assert!(!is_stopword("algorithm"));
        assert!(!is_stopword("rust"));
        assert!(!is_stopword("tantivy"));
    }

    #[test]

    
    fn test_stopwords_list() {
        assert!(!DEFAULT_STOPWORDS.is_empty());
        assert!(DEFAULT_STOPWORDS.len() > 80);
    }