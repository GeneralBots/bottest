


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use serde_json;



    #[test]


    fn test_collection_stats_serialization() {
        let stats = CollectionStats {
            name: "test_collection".to_string(),
            vectors_count: 1000,
            points_count: 1000,
            segments_count: 2,
            disk_data_size: 1024 * 1024,
            ram_data_size: 512 * 1024,
            indexed_vectors_count: 1000,
            status: "green".to_string(),
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("test_collection"));
        assert!(json.contains("1000"));
    }

    #[test]


    fn test_kb_statistics_serialization() {
        let stats = KBStatistics {
            total_collections: 3,
            total_documents: 5000,
            total_vectors: 5000,
            total_disk_size_mb: 10.5,
            total_ram_size_mb: 5.2,
            documents_added_last_week: 100,
            documents_added_last_month: 500,
            collections: vec![],
        };

        let json = serde_json::to_string(&stats).unwrap();
        assert!(json.contains("5000"));
        assert!(json.contains("10.5"));
    }