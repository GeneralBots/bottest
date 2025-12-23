//! Unit tests migrated from src/core/package_manager/cache.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver
    use std::io::Write;
    use tempfile::TempDir;

    fn create_test_config(dir: &Path) -> Result<()> {
        let config = r#"
[cache_settings]
cache_dir = "test-cache"

[components.test]
name = "Test Component"
url = "https://example.com/test.tar.gz"
filename = "test.tar.gz"
sha256 = ""

[models.test_model]
name = "Test Model"
url = "https://example.com/model.gguf"
filename = "model.gguf"
sha256 = ""
"#;
        let config_path = dir.join(CONFIG_FILE);
        fs::write(config_path, config)?;
        Ok(())
    }

    #[test]

    
    fn test_extract_filename() {
        assert_eq!(
            DownloadCache::extract_filename("https://example.com/path/file.tar.gz"),
            "file.tar.gz"
        );
        assert_eq!(
            DownloadCache::extract_filename("https://example.com/file.zip?token=abc"),
            "file.zip"
        );
        assert_eq!(DownloadCache::extract_filename("https://example.com/"), "");
    }

    #[test]

    
    fn test_cache_creation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        assert!(cache.cache_dir().exists());
        assert_eq!(cache.cache_dir().file_name().unwrap(), "test-cache");

        Ok(())
    }

    #[test]

    
    fn test_is_cached() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        // Initially not cached
        assert!(!cache.is_cached("test.tar.gz"));

        // Create a cached file
        let cache_path = cache.get_cache_path("test.tar.gz");
        let mut file = fs::File::create(&cache_path)?;
        file.write_all(b"test content")?;

        // Now it should be cached
        assert!(cache.is_cached("test.tar.gz"));

        // Empty file should not count as cached
        let empty_path = cache.get_cache_path("empty.tar.gz");
        fs::File::create(&empty_path)?;
        assert!(!cache.is_cached("empty.tar.gz"));

        Ok(())
    }

    #[test]

    
    fn test_resolve_url() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        // Test with uncached URL
        let result = cache.resolve_url("https://example.com/newfile.tar.gz");
        assert!(!result.is_cached());
        assert_eq!(result.url(), Some("https://example.com/newfile.tar.gz"));

        // Create cached file
        let cache_path = cache.get_cache_path("newfile.tar.gz");
        let mut file = fs::File::create(&cache_path)?;
        file.write_all(b"cached content")?;

        // Now it should resolve to cached
        let result = cache.resolve_url("https://example.com/newfile.tar.gz");
        assert!(result.is_cached());
        assert!(result.url().is_none());

        Ok(())
    }

    #[test]

    
    fn test_get_component() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        let component = cache.get_component("test");
        assert!(component.is_some());
        assert_eq!(component.unwrap().name, "Test Component");

        let missing = cache.get_component("nonexistent");
        assert!(missing.is_none());

        Ok(())
    }

    #[test]

    
    fn test_list_cached() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        // Create some cached files
        fs::write(cache.get_cache_path("file1.tar.gz"), "content1")?;
        fs::write(cache.get_cache_path("file2.zip"), "content2")?;

        let files = cache.list_cached()?;
        assert_eq!(files.len(), 2);
        assert!(files.contains(&"file1.tar.gz".to_string()));
        assert!(files.contains(&"file2.zip".to_string()));

        Ok(())
    }

    #[test]

    
    fn test_cache_size() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        // Initially empty
        assert_eq!(cache.cache_size()?, 0);

        // Add files
        fs::write(cache.get_cache_path("file1.txt"), "12345")?; // 5 bytes
        fs::write(cache.get_cache_path("file2.txt"), "1234567890")?; // 10 bytes

        assert_eq!(cache.cache_size()?, 15);

        Ok(())
    }

    #[test]

    
    fn test_clear_cache() -> Result<()> {
        let temp_dir = TempDir::new()?;
        create_test_config(temp_dir.path())?;

        let cache = DownloadCache::new(temp_dir.path())?;

        // Create some cached files
        fs::write(cache.get_cache_path("file1.tar.gz"), "content1")?;
        fs::write(cache.get_cache_path("file2.zip"), "content2")?;

        assert_eq!(cache.list_cached()?.len(), 2);

        cache.clear_cache()?;

        assert_eq!(cache.list_cached()?.len(), 0);

        Ok(())
    }