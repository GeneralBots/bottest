//! Unit tests migrated from src/core/shared/state.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[tokio::test]
    async fn test_extensions_insert_and_get() {
        let ext = Extensions::new();
        ext.insert(42i32).await;
        ext.insert("hello".to_string()).await;

        let num = ext.get::<i32>().await;
        assert!(num.is_some());
        assert_eq!(*num.unwrap(), 42);

        let text = ext.get::<String>().await;
        assert!(text.is_some());
        assert_eq!(&*text.unwrap(), "hello");
    }

    #[tokio::test]
    async fn test_extensions_clone_shares_data() {
        let ext1 = Extensions::new();
        ext1.insert(100u64).await;

        let ext2 = ext1.clone();

        let val = ext2.get::<u64>().await;
        assert!(val.is_some());
        assert_eq!(*val.unwrap(), 100);

        ext2.insert(200u32).await;

        let val2 = ext1.get::<u32>().await;
        assert!(val2.is_some());
        assert_eq!(*val2.unwrap(), 200);
    }

    #[tokio::test]
    async fn test_extensions_remove() {
        let ext = Extensions::new();
        ext.insert(42i32).await;

        assert!(ext.contains::<i32>().await);
        assert_eq!(ext.len().await, 1);

        let removed = ext.remove::<i32>().await;
        assert!(removed.is_some());
        assert_eq!(*removed.unwrap(), 42);

        assert!(!ext.contains::<i32>().await);
        assert!(ext.is_empty().await);
    }

    #[tokio::test]
    async fn test_extensions_get_nonexistent() {
        let ext = Extensions::new();
        let val = ext.get::<i32>().await;
        assert!(val.is_none());
    }