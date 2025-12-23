//! Unit tests migrated from src/email/vectordb.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_email_document_creation() {
        let email = EmailDocument {
            id: "test-123".to_string(),
            account_id: "account-456".to_string(),
            from_email: "sender@example.com".to_string(),
            from_name: "Test Sender".to_string(),
            to_email: "receiver@example.com".to_string(),
            subject: "Test Subject".to_string(),
            body_text: "Test email body".to_string(),
            date: Utc::now(),
            folder: "INBOX".to_string(),
            has_attachments: false,
            thread_id: None,
        };

        assert_eq!(email.id, "test-123");
        assert_eq!(email.subject, "Test Subject");
    }

    #[tokio::test]
    async fn test_user_email_vectordb_creation() {
        let temp_dir = std::env::temp_dir().join("test_vectordb");
        let db = UserEmailVectorDB::new(Uuid::new_v4(), Uuid::new_v4(), temp_dir);

        assert!(db.collection_name.starts_with("emails_"));
    }