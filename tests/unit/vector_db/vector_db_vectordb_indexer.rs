


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_indexing_stats_creation() {
        let stats = IndexingStats {
            emails_indexed: 10,
            files_indexed: 5,
            emails_pending: 2,
            files_pending: 3,
            last_run: Some(Utc::now()),
            errors: 0,
        };

        assert_eq!(stats.emails_indexed, 10);
        assert_eq!(stats.files_indexed, 5);
    }