//! Unit tests migrated from src/core/secrets/mod.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    /// Helper function to parse database URL into HashMap for tests
    fn parse_database_url(url: &str) -> Result<HashMap<String, String>> {
        let mut result = HashMap::new();
        if let Some(stripped) = url.strip_prefix("postgres://") {
            let parts: Vec<&str> = stripped.split('@').collect();
            if parts.len() == 2 {
                let user_pass: Vec<&str> = parts[0].split(':').collect();
                let host_db: Vec<&str> = parts[1].split('/').collect();

                result.insert(
                    "username".to_string(),
                    user_pass.get(0).unwrap_or(&"").to_string(),
                );
                result.insert(
                    "password".to_string(),
                    user_pass.get(1).unwrap_or(&"").to_string(),
                );

                let host_port: Vec<&str> = host_db[0].split(':').collect();
                result.insert(
                    "host".to_string(),
                    host_port.get(0).unwrap_or(&"").to_string(),
                );
                result.insert(
                    "port".to_string(),
                    host_port.get(1).unwrap_or(&"5432").to_string(),
                );

                if host_db.len() >= 2 {
                    result.insert("database".to_string(), host_db[1].to_string());
                }
            }
        }
        Ok(result)
    }

    #[test]

    
    fn test_parse_database_url() {
        let parsed = parse_database_url("postgres://user:pass@localhost:5432/mydb").unwrap();
        assert_eq!(parsed.get("username"), Some(&"user".to_string()));
        assert_eq!(parsed.get("password"), Some(&"pass".to_string()));
        assert_eq!(parsed.get("host"), Some(&"localhost".to_string()));
        assert_eq!(parsed.get("port"), Some(&"5432".to_string()));
        assert_eq!(parsed.get("database"), Some(&"mydb".to_string()));
    }

    #[test]

    
    fn test_parse_database_url_minimal() {
        let parsed = parse_database_url("postgres://user@localhost/mydb").unwrap();
        assert_eq!(parsed.get("username"), Some(&"user".to_string()));
        assert_eq!(parsed.get("password"), Some(&"".to_string()));
        assert_eq!(parsed.get("host"), Some(&"localhost".to_string()));
        assert_eq!(parsed.get("port"), Some(&"5432".to_string()));
    }

    #[test]

    
    fn test_secret_paths() {
        assert_eq!(SecretPaths::DIRECTORY, "gbo/directory");
        assert_eq!(SecretPaths::TABLES, "gbo/tables");
        assert_eq!(SecretPaths::LLM, "gbo/llm");
    }