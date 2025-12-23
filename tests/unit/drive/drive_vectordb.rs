


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_file_document_creation() {
        let file = FileDocument {
            id: "test-123".to_string(),
            file_path: "/test/file.txt".to_string(),
            file_name: "file.txt".to_string(),
            file_type: "text".to_string(),
            file_size: 1024,
            bucket: "test-bucket".to_string(),
            content_text: "Test file content".to_string(),
            content_summary: Some("Summary".to_string()),
            created_at: Utc::now(),
            modified_at: Utc::now(),
            indexed_at: Utc::now(),
            mime_type: Some("text/plain".to_string()),
            tags: vec!["test".to_string()],
        };

        assert_eq!(file.id, "test-123");
        assert_eq!(file.file_name, "file.txt");
    }

    #[test]


    fn test_should_index() {
        assert!(FileContentExtractor::should_index("text/plain", 1024));
        assert!(FileContentExtractor::should_index("text/markdown", 5000));
        assert!(!FileContentExtractor::should_index(
            "text/plain",
            20 * 1024 * 1024
        ));
        assert!(!FileContentExtractor::should_index("video/mp4", 1024));
    }

    #[tokio::test]
    async fn test_user_drive_vectordb_creation() {
        let temp_dir = std::env::temp_dir().join("test_drive_vectordb");
        let db = UserDriveVectorDB::new(Uuid::new_v4(), Uuid::new_v4(), temp_dir);

        assert!(db.collection_name.starts_with("drive_"));
    }