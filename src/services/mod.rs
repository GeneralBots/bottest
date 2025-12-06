//! Service management for test infrastructure
//!
//! Provides real service instances (PostgreSQL, MinIO, Redis) for integration testing.
//! Each service runs on a dynamic port to enable parallel test execution.

mod minio;
mod postgres;
mod redis;

pub use minio::MinioService;
pub use postgres::PostgresService;
pub use redis::RedisService;

use anyhow::Result;
use std::path::Path;
use std::time::Duration;
use tokio::time::sleep;

/// Default timeout for service health checks
pub const HEALTH_CHECK_TIMEOUT: Duration = Duration::from_secs(30);

/// Default interval between health check attempts
pub const HEALTH_CHECK_INTERVAL: Duration = Duration::from_millis(100);

/// Wait for a condition to become true with timeout
pub async fn wait_for<F, Fut>(timeout: Duration, interval: Duration, mut check: F) -> Result<()>
where
    F: FnMut() -> Fut,
    Fut: std::future::Future<Output = bool>,
{
    let start = std::time::Instant::now();
    while start.elapsed() < timeout {
        if check().await {
            return Ok(());
        }
        sleep(interval).await;
    }
    anyhow::bail!("Timeout waiting for condition")
}

/// Check if a TCP port is accepting connections
pub async fn check_tcp_port(host: &str, port: u16) -> bool {
    tokio::net::TcpStream::connect((host, port)).await.is_ok()
}

/// Create a directory if it doesn't exist
pub fn ensure_dir(path: &Path) -> Result<()> {
    if !path.exists() {
        std::fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Service trait for common operations
#[async_trait::async_trait]
pub trait Service: Send + Sync {
    /// Start the service
    async fn start(&mut self) -> Result<()>;

    /// Stop the service gracefully
    async fn stop(&mut self) -> Result<()>;

    /// Check if the service is healthy
    async fn health_check(&self) -> Result<bool>;

    /// Get the service connection URL
    fn connection_url(&self) -> String;
}

/// Service status
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ServiceStatus {
    Stopped,
    Starting,
    Running,
    Stopping,
    Failed,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_wait_for_success() {
        let mut counter = 0;
        let result = wait_for(Duration::from_secs(1), Duration::from_millis(10), || {
            counter += 1;
            async move { counter >= 3 }
        })
        .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_wait_for_timeout() {
        let result = wait_for(
            Duration::from_millis(50),
            Duration::from_millis(10),
            || async { false },
        )
        .await;
        assert!(result.is_err());
    }
}
