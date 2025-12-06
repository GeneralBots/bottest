//! MinIO service management for test infrastructure
//!
//! Starts and manages a MinIO instance for S3-compatible storage testing.
//! Provides bucket creation, object operations, and credential management.

use super::{check_tcp_port, ensure_dir, wait_for, HEALTH_CHECK_INTERVAL, HEALTH_CHECK_TIMEOUT};
use anyhow::{Context, Result};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::collections::HashMap;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

/// MinIO service for S3-compatible storage in test environments
pub struct MinioService {
    api_port: u16,
    console_port: u16,
    data_dir: PathBuf,
    process: Option<Child>,
    access_key: String,
    secret_key: String,
}

impl MinioService {
    /// Default access key for tests
    pub const DEFAULT_ACCESS_KEY: &'static str = "minioadmin";

    /// Default secret key for tests
    pub const DEFAULT_SECRET_KEY: &'static str = "minioadmin";

    /// Start a new MinIO instance on the specified port
    pub async fn start(api_port: u16, data_dir: &str) -> Result<Self> {
        let data_path = PathBuf::from(data_dir).join("minio");
        ensure_dir(&data_path)?;

        // Allocate a console port (api_port + 1000 or find available)
        let console_port = api_port + 1000;

        let mut service = Self {
            api_port,
            console_port,
            data_dir: data_path,
            process: None,
            access_key: Self::DEFAULT_ACCESS_KEY.to_string(),
            secret_key: Self::DEFAULT_SECRET_KEY.to_string(),
        };

        service.start_server().await?;
        service.wait_ready().await?;

        Ok(service)
    }

    /// Start MinIO with custom credentials
    pub async fn start_with_credentials(
        api_port: u16,
        data_dir: &str,
        access_key: &str,
        secret_key: &str,
    ) -> Result<Self> {
        let data_path = PathBuf::from(data_dir).join("minio");
        ensure_dir(&data_path)?;

        let console_port = api_port + 1000;

        let mut service = Self {
            api_port,
            console_port,
            data_dir: data_path,
            process: None,
            access_key: access_key.to_string(),
            secret_key: secret_key.to_string(),
        };

        service.start_server().await?;
        service.wait_ready().await?;

        Ok(service)
    }

    /// Start the MinIO server process
    async fn start_server(&mut self) -> Result<()> {
        log::info!(
            "Starting MinIO on port {} (console: {})",
            self.api_port,
            self.console_port
        );

        let minio = Self::find_binary()?;

        let child = Command::new(&minio)
            .args([
                "server",
                self.data_dir.to_str().unwrap(),
                "--address",
                &format!("127.0.0.1:{}", self.api_port),
                "--console-address",
                &format!("127.0.0.1:{}", self.console_port),
            ])
            .env("MINIO_ROOT_USER", &self.access_key)
            .env("MINIO_ROOT_PASSWORD", &self.secret_key)
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .context("Failed to start MinIO")?;

        self.process = Some(child);
        Ok(())
    }

    /// Wait for MinIO to be ready
    async fn wait_ready(&self) -> Result<()> {
        log::info!("Waiting for MinIO to be ready...");

        wait_for(HEALTH_CHECK_TIMEOUT, HEALTH_CHECK_INTERVAL, || async {
            check_tcp_port("127.0.0.1", self.api_port).await
        })
        .await
        .context("MinIO failed to start in time")?;

        // Additional health check via HTTP
        let health_url = format!("http://127.0.0.1:{}/minio/health/live", self.api_port);
        for _ in 0..30 {
            if let Ok(resp) = reqwest::get(&health_url).await {
                if resp.status().is_success() {
                    return Ok(());
                }
            }
            sleep(Duration::from_millis(100)).await;
        }

        // Even if health check fails, TCP is up so proceed
        Ok(())
    }

    /// Create a new bucket
    pub async fn create_bucket(&self, name: &str) -> Result<()> {
        log::info!("Creating bucket '{}'", name);

        // Try using mc (MinIO client) if available
        if let Ok(mc) = Self::find_mc_binary() {
            // Configure mc alias
            let alias_name = format!("test{}", self.api_port);
            let _ = Command::new(&mc)
                .args([
                    "alias",
                    "set",
                    &alias_name,
                    &self.endpoint(),
                    &self.access_key,
                    &self.secret_key,
                ])
                .output();

            let output = Command::new(&mc)
                .args(["mb", "--ignore-existing", &format!("{}/{}", alias_name, name)])
                .output()?;

            if !output.status.success() {
                let stderr = String::from_utf8_lossy(&output.stderr);
                if !stderr.contains("already") {
                    anyhow::bail!("Failed to create bucket: {}", stderr);
                }
            }

            return Ok(());
        }

        // Fallback: use HTTP PUT request
        let url = format!("{}/{}", self.endpoint(), name);
        let client = reqwest::Client::new();
        let resp = client
            .put(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await?;

        if !resp.status().is_success() && resp.status().as_u16() != 409 {
            anyhow::bail!("Failed to create bucket: {}", resp.status());
        }

        Ok(())
    }

    /// Put an object into a bucket
    pub async fn put_object(&self, bucket: &str, key: &str, data: &[u8]) -> Result<()> {
        log::debug!("Putting object '{}/{}' ({} bytes)", bucket, key, data.len());

        let url = format!("{}/{}/{}", self.endpoint(), bucket, key);
        let client = reqwest::Client::new();
        let resp = client
            .put(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .body(data.to_vec())
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Failed to put object: {}", resp.status());
        }

        Ok(())
    }

    /// Get an object from a bucket
    pub async fn get_object(&self, bucket: &str, key: &str) -> Result<Vec<u8>> {
        log::debug!("Getting object '{}/{}'", bucket, key);

        let url = format!("{}/{}/{}", self.endpoint(), bucket, key);
        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Failed to get object: {}", resp.status());
        }

        Ok(resp.bytes().await?.to_vec())
    }

    /// Delete an object from a bucket
    pub async fn delete_object(&self, bucket: &str, key: &str) -> Result<()> {
        log::debug!("Deleting object '{}/{}'", bucket, key);

        let url = format!("{}/{}/{}", self.endpoint(), bucket, key);
        let client = reqwest::Client::new();
        let resp = client
            .delete(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await?;

        if !resp.status().is_success() && resp.status().as_u16() != 404 {
            anyhow::bail!("Failed to delete object: {}", resp.status());
        }

        Ok(())
    }

    /// List objects in a bucket
    pub async fn list_objects(&self, bucket: &str, prefix: Option<&str>) -> Result<Vec<String>> {
        log::debug!("Listing objects in bucket '{}'", bucket);

        let mut url = format!("{}/{}", self.endpoint(), bucket);
        if let Some(p) = prefix {
            url = format!("{}?prefix={}", url, p);
        }

        let client = reqwest::Client::new();
        let resp = client
            .get(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await?;

        if !resp.status().is_success() {
            anyhow::bail!("Failed to list objects: {}", resp.status());
        }

        // Parse XML response (simplified)
        let body = resp.text().await?;
        let mut objects = Vec::new();

        // Simple XML parsing for <Key> elements
        for line in body.lines() {
            if let Some(start) = line.find("<Key>") {
                if let Some(end) = line.find("</Key>") {
                    let key = &line[start + 5..end];
                    objects.push(key.to_string());
                }
            }
        }

        Ok(objects)
    }

    /// Check if a bucket exists
    pub async fn bucket_exists(&self, name: &str) -> Result<bool> {
        let url = format!("{}/{}", self.endpoint(), name);
        let client = reqwest::Client::new();
        let resp = client
            .head(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await?;

        Ok(resp.status().is_success())
    }

    /// Delete a bucket
    pub async fn delete_bucket(&self, name: &str) -> Result<()> {
        log::info!("Deleting bucket '{}'", name);

        let url = format!("{}/{}", self.endpoint(), name);
        let client = reqwest::Client::new();
        let resp = client
            .delete(&url)
            .basic_auth(&self.access_key, Some(&self.secret_key))
            .send()
            .await?;

        if !resp.status().is_success() && resp.status().as_u16() != 404 {
            anyhow::bail!("Failed to delete bucket: {}", resp.status());
        }

        Ok(())
    }

    /// Get the S3 endpoint URL
    pub fn endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.api_port)
    }

    /// Get the console URL
    pub fn console_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.console_port)
    }

    /// Get the API port
    pub fn api_port(&self) -> u16 {
        self.api_port
    }

    /// Get the console port
    pub fn console_port(&self) -> u16 {
        self.console_port
    }

    /// Get credentials as (access_key, secret_key)
    pub fn credentials(&self) -> (String, String) {
        (self.access_key.clone(), self.secret_key.clone())
    }

    /// Get S3-compatible configuration for AWS SDK
    pub fn s3_config(&self) -> HashMap<String, String> {
        let mut config = HashMap::new();
        config.insert("endpoint_url".to_string(), self.endpoint());
        config.insert("access_key_id".to_string(), self.access_key.clone());
        config.insert("secret_access_key".to_string(), self.secret_key.clone());
        config.insert("region".to_string(), "us-east-1".to_string());
        config.insert("force_path_style".to_string(), "true".to_string());
        config
    }

    /// Find the MinIO binary
    fn find_binary() -> Result<PathBuf> {
        let common_paths = [
            "/usr/local/bin/minio",
            "/usr/bin/minio",
            "/opt/minio/minio",
            "/opt/homebrew/bin/minio",
        ];

        for path in common_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return Ok(p);
            }
        }

        which::which("minio").context("minio binary not found in PATH or common locations")
    }

    /// Find the MinIO client (mc) binary
    fn find_mc_binary() -> Result<PathBuf> {
        let common_paths = [
            "/usr/local/bin/mc",
            "/usr/bin/mc",
            "/opt/homebrew/bin/mc",
        ];

        for path in common_paths {
            let p = PathBuf::from(path);
            if p.exists() {
                return Ok(p);
            }
        }

        which::which("mc").context("mc binary not found")
    }

    /// Stop the MinIO server
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.process {
            log::info!("Stopping MinIO...");

            let pid = Pid::from_raw(child.id() as i32);
            let _ = kill(pid, Signal::SIGTERM);

            for _ in 0..50 {
                match child.try_wait() {
                    Ok(Some(_)) => {
                        self.process = None;
                        return Ok(());
                    }
                    Ok(None) => sleep(Duration::from_millis(100)).await,
                    Err(_) => break,
                }
            }

            let _ = kill(pid, Signal::SIGKILL);
            let _ = child.wait();
            self.process = None;
        }

        Ok(())
    }

    /// Clean up data directory
    pub fn cleanup(&self) -> Result<()> {
        if self.data_dir.exists() {
            std::fs::remove_dir_all(&self.data_dir)?;
        }
        Ok(())
    }
}

impl Drop for MinioService {
    fn drop(&mut self) {
        if let Some(ref mut child) = self.process {
            let pid = Pid::from_raw(child.id() as i32);
            let _ = kill(pid, Signal::SIGTERM);

            std::thread::sleep(Duration::from_millis(500));

            let _ = kill(pid, Signal::SIGKILL);
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_endpoint_format() {
        let service = MinioService {
            api_port: 9000,
            console_port: 10000,
            data_dir: PathBuf::from("/tmp/test"),
            process: None,
            access_key: "test".to_string(),
            secret_key: "secret".to_string(),
        };

        assert_eq!(service.endpoint(), "http://127.0.0.1:9000");
        assert_eq!(service.console_url(), "http://127.0.0.1:10000");
    }

    #[test]
    fn test_credentials() {
        let service = MinioService {
            api_port: 9000,
            console_port: 10000,
            data_dir: PathBuf::from("/tmp/test"),
            process: None,
            access_key: "mykey".to_string(),
            secret_key: "mysecret".to_string(),
        };

        let (key, secret) = service.credentials();
        assert_eq!(key, "mykey");
        assert_eq!(secret, "mysecret");
    }

    #[test]
    fn test_s3_config() {
        let service = MinioService {
            api_port: 9000,
            console_port: 10000,
            data_dir: PathBuf::from("/tmp/test"),
            process: None,
            access_key: "access".to_string(),
            secret_key: "secret".to_string(),
        };

        let config = service.s3_config();
        assert_eq!(config.get("endpoint_url"), Some(&"http://127.0.0.1:9000".to_string()));
        assert_eq!(config.get("access_key_id"), Some(&"access".to_string()));
        assert_eq!(config.get("force_path_style"), Some(&"true".to_string()));
    }
}
