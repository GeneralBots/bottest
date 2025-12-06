//! PostgreSQL service management for test infrastructure
//!
//! Starts and manages a PostgreSQL instance for integration testing.
//! Finds PostgreSQL binaries from system installation or botserver-stack.

use super::{check_tcp_port, ensure_dir, wait_for, HEALTH_CHECK_INTERVAL, HEALTH_CHECK_TIMEOUT};
use anyhow::{Context, Result};
use nix::sys::signal::{kill, Signal};
use nix::unistd::Pid;
use std::path::PathBuf;
use std::process::{Child, Command, Stdio};
use std::time::Duration;
use tokio::time::sleep;

/// PostgreSQL service for test environments
pub struct PostgresService {
    port: u16,
    data_dir: PathBuf,
    bin_dir: PathBuf,
    lib_dir: Option<PathBuf>,
    process: Option<Child>,
    connection_string: String,
    database_name: String,
    username: String,
    password: String,
}

impl PostgresService {
    /// Default database name for tests
    pub const DEFAULT_DATABASE: &'static str = "bottest";

    /// Default username for tests
    pub const DEFAULT_USERNAME: &'static str = "bottest";

    /// Default password for tests
    pub const DEFAULT_PASSWORD: &'static str = "bottest";

    /// Find PostgreSQL binaries - checks botserver-stack first, then system paths
    fn find_postgres_installation() -> Result<(PathBuf, Option<PathBuf>)> {
        // First, check BOTSERVER_STACK_PATH env var
        if let Ok(stack_path) = std::env::var("BOTSERVER_STACK_PATH") {
            let bin_dir = PathBuf::from(&stack_path).join("bin/tables/bin");
            let lib_dir = PathBuf::from(&stack_path).join("bin/tables/lib");
            if bin_dir.join("postgres").exists() || bin_dir.join("initdb").exists() {
                log::info!("Using PostgreSQL from BOTSERVER_STACK_PATH: {:?}", bin_dir);
                return Ok((bin_dir, Some(lib_dir)));
            }
        }

        // Check relative paths from current directory
        let cwd = std::env::current_dir().unwrap_or_default();
        let relative_paths = [
            "../botserver/botserver-stack/bin/tables/bin",
            "botserver/botserver-stack/bin/tables/bin",
            "botserver-stack/bin/tables/bin",
        ];

        for rel_path in &relative_paths {
            let bin_dir = cwd.join(rel_path);
            if bin_dir.join("postgres").exists() || bin_dir.join("initdb").exists() {
                let lib_dir = bin_dir.parent().unwrap().join("lib");
                log::info!("Using PostgreSQL from botserver-stack: {:?}", bin_dir);
                return Ok((
                    bin_dir,
                    if lib_dir.exists() {
                        Some(lib_dir)
                    } else {
                        None
                    },
                ));
            }
        }

        // Check system PostgreSQL paths
        let system_paths = [
            "/usr/lib/postgresql/17/bin",
            "/usr/lib/postgresql/16/bin",
            "/usr/lib/postgresql/15/bin",
            "/usr/lib/postgresql/14/bin",
            "/usr/bin",
            "/usr/local/bin",
            "/opt/homebrew/bin",
            "/opt/homebrew/opt/postgresql@17/bin",
            "/opt/homebrew/opt/postgresql@16/bin",
            "/opt/homebrew/opt/postgresql@15/bin",
        ];

        for path in &system_paths {
            let bin_dir = PathBuf::from(path);
            if bin_dir.join("postgres").exists() || bin_dir.join("initdb").exists() {
                log::info!("Using system PostgreSQL from: {:?}", bin_dir);
                return Ok((bin_dir, None));
            }
        }

        // Last resort: try to find via which
        if let Ok(initdb_path) = which::which("initdb") {
            if let Some(bin_dir) = initdb_path.parent() {
                log::info!("Using PostgreSQL from PATH: {:?}", bin_dir);
                return Ok((bin_dir.to_path_buf(), None));
            }
        }

        anyhow::bail!(
            "PostgreSQL not found. Install PostgreSQL or set BOTSERVER_STACK_PATH env var"
        )
    }

    /// Start a new PostgreSQL instance on the specified port
    pub async fn start(port: u16, data_dir: &str) -> Result<Self> {
        let (bin_dir, lib_dir) = Self::find_postgres_installation()?;

        let data_path = PathBuf::from(data_dir).join("postgres");
        ensure_dir(&data_path)?;

        let mut service = Self {
            port,
            data_dir: data_path.clone(),
            bin_dir,
            lib_dir,
            process: None,
            connection_string: String::new(),
            database_name: Self::DEFAULT_DATABASE.to_string(),
            username: Self::DEFAULT_USERNAME.to_string(),
            password: Self::DEFAULT_PASSWORD.to_string(),
        };

        service.connection_string = service.build_connection_string();

        // Initialize database cluster if needed
        if !data_path.join("PG_VERSION").exists() {
            service.init_db().await?;
        }

        // Start PostgreSQL
        service.start_server().await?;

        // Wait for it to be ready
        service.wait_ready().await?;

        // Create test database and user
        service.setup_test_database().await?;

        Ok(service)
    }

    /// Get binary path
    fn get_binary(&self, name: &str) -> PathBuf {
        self.bin_dir.join(name)
    }

    /// Build command with LD_LIBRARY_PATH if needed
    fn build_command(&self, binary_name: &str) -> Command {
        let binary = self.get_binary(binary_name);
        let mut cmd = Command::new(&binary);
        if let Some(ref lib_dir) = self.lib_dir {
            cmd.env("LD_LIBRARY_PATH", lib_dir);
        }
        cmd
    }

    /// Initialize the database cluster
    async fn init_db(&self) -> Result<()> {
        log::info!(
            "Initializing PostgreSQL data directory at {:?}",
            self.data_dir
        );

        let output = self
            .build_command("initdb")
            .args([
                "-D",
                self.data_dir.to_str().unwrap(),
                "-U",
                "postgres",
                "-A",
                "trust",
                "-E",
                "UTF8",
                "--no-locale",
            ])
            .output()
            .context("Failed to run initdb")?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("initdb failed: {}", stderr);
        }

        // Configure postgresql.conf for testing
        self.configure_for_testing()?;

        Ok(())
    }

    /// Configure PostgreSQL for fast testing (reduced durability)
    fn configure_for_testing(&self) -> Result<()> {
        let config_path = self.data_dir.join("postgresql.conf");

        // Use absolute path for unix_socket_directories
        let abs_data_dir = self
            .data_dir
            .canonicalize()
            .unwrap_or_else(|_| self.data_dir.clone());

        let config = format!(
            r#"
# Test configuration - optimized for speed, not durability
listen_addresses = '127.0.0.1'
port = {}
max_connections = 50
shared_buffers = 128MB
work_mem = 16MB
maintenance_work_mem = 64MB
wal_level = minimal
fsync = off
synchronous_commit = off
full_page_writes = off
checkpoint_timeout = 30min
max_wal_senders = 0
logging_collector = off
log_statement = 'none'
log_duration = off
unix_socket_directories = '{}'
"#,
            self.port,
            abs_data_dir.to_str().unwrap()
        );

        std::fs::write(&config_path, config)?;
        Ok(())
    }

    /// Start the PostgreSQL server process
    async fn start_server(&mut self) -> Result<()> {
        log::info!("Starting PostgreSQL on port {}", self.port);

        // Create log file for debugging
        let log_path = self.data_dir.join("postgres.log");
        let log_file = std::fs::File::create(&log_path)
            .context(format!("Failed to create log file {:?}", log_path))?;
        let stderr_file = log_file.try_clone()?;

        log::debug!("PostgreSQL log file: {:?}", log_path);

        let mut cmd = self.build_command("postgres");
        let child = cmd
            .args(["-D", self.data_dir.to_str().unwrap()])
            .stdout(Stdio::from(log_file))
            .stderr(Stdio::from(stderr_file))
            .spawn()
            .context("Failed to start PostgreSQL")?;

        self.process = Some(child);
        Ok(())
    }

    /// Wait for PostgreSQL to be ready to accept connections
    async fn wait_ready(&self) -> Result<()> {
        log::info!("Waiting for PostgreSQL to be ready...");

        let result = wait_for(HEALTH_CHECK_TIMEOUT, HEALTH_CHECK_INTERVAL, || async {
            check_tcp_port("127.0.0.1", self.port).await
        })
        .await;

        if result.is_err() {
            // Read log file to show error
            let log_path = self.data_dir.join("postgres.log");
            if log_path.exists() {
                if let Ok(log_content) = std::fs::read_to_string(&log_path) {
                    log::error!("PostgreSQL log:\n{}", log_content);
                }
            }
            return Err(result.unwrap_err()).context("PostgreSQL failed to start in time");
        }

        // Additional wait for pg_isready
        for _ in 0..30 {
            let status = self
                .build_command("pg_isready")
                .args(["-h", "127.0.0.1", "-p", &self.port.to_string()])
                .status();

            if status.map(|s| s.success()).unwrap_or(false) {
                return Ok(());
            }
            sleep(Duration::from_millis(100)).await;
        }

        Ok(())
    }

    /// Create the test database and user
    async fn setup_test_database(&self) -> Result<()> {
        log::info!("Setting up test database '{}'", self.database_name);

        // Create user
        let _ = self
            .build_command("psql")
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &self.port.to_string(),
                "-U",
                "postgres",
                "-c",
                &format!(
                    "CREATE USER {} WITH PASSWORD '{}' SUPERUSER",
                    self.username, self.password
                ),
            ])
            .output();

        // Create database
        let _ = self
            .build_command("psql")
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &self.port.to_string(),
                "-U",
                "postgres",
                "-c",
                &format!(
                    "CREATE DATABASE {} OWNER {}",
                    self.database_name, self.username
                ),
            ])
            .output();

        Ok(())
    }

    /// Run database migrations
    pub async fn run_migrations(&self) -> Result<()> {
        log::info!("Running database migrations...");

        // Try to run migrations using diesel CLI if available
        if let Ok(diesel) = which::which("diesel") {
            let status = Command::new(diesel)
                .args([
                    "migration",
                    "run",
                    "--database-url",
                    &self.connection_string,
                ])
                .status();

            if status.map(|s| s.success()).unwrap_or(false) {
                return Ok(());
            }
        }

        // Fallback: run migrations programmatically via botlib if available
        log::warn!("diesel CLI not available, skipping migrations");
        Ok(())
    }

    /// Create a new database with the given name
    pub async fn create_database(&self, name: &str) -> Result<()> {
        let output = self
            .build_command("psql")
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &self.port.to_string(),
                "-U",
                &self.username,
                "-c",
                &format!("CREATE DATABASE {}", name),
            ])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            if !stderr.contains("already exists") {
                anyhow::bail!("Failed to create database: {}", stderr);
            }
        }

        Ok(())
    }

    /// Execute raw SQL
    pub async fn execute(&self, sql: &str) -> Result<()> {
        let output = self
            .build_command("psql")
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &self.port.to_string(),
                "-U",
                &self.username,
                "-d",
                &self.database_name,
                "-c",
                sql,
            ])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("SQL execution failed: {}", stderr);
        }

        Ok(())
    }

    /// Execute SQL and return results as JSON
    pub async fn query(&self, sql: &str) -> Result<String> {
        let output = self
            .build_command("psql")
            .args([
                "-h",
                "127.0.0.1",
                "-p",
                &self.port.to_string(),
                "-U",
                &self.username,
                "-d",
                &self.database_name,
                "-t", // tuples only
                "-A", // unaligned
                "-c",
                sql,
            ])
            .output()?;

        if !output.status.success() {
            let stderr = String::from_utf8_lossy(&output.stderr);
            anyhow::bail!("SQL query failed: {}", stderr);
        }

        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    /// Get the connection string
    pub fn connection_string(&self) -> String {
        self.connection_string.clone()
    }

    /// Get the port
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Build the connection string
    fn build_connection_string(&self) -> String {
        format!(
            "postgres://{}:{}@127.0.0.1:{}/{}",
            self.username, self.password, self.port, self.database_name
        )
    }

    /// Stop the PostgreSQL server
    pub async fn stop(&mut self) -> Result<()> {
        if let Some(ref mut child) = self.process {
            log::info!("Stopping PostgreSQL...");

            // Try graceful shutdown first
            let pid = Pid::from_raw(child.id() as i32);
            let _ = kill(pid, Signal::SIGTERM);

            // Wait for process to exit
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

            // Force kill if still running
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

impl Drop for PostgresService {
    fn drop(&mut self) {
        if let Some(ref mut child) = self.process {
            let pid = Pid::from_raw(child.id() as i32);
            let _ = kill(pid, Signal::SIGTERM);

            // Give it a moment to shut down gracefully
            std::thread::sleep(Duration::from_millis(500));

            // Force kill if needed
            let _ = kill(pid, Signal::SIGKILL);
            let _ = child.wait();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connection_string_format() {
        let service = PostgresService {
            port: 5432,
            data_dir: PathBuf::from("/tmp/test"),
            bin_dir: PathBuf::from("/usr/bin"),
            lib_dir: None,
            process: None,
            connection_string: String::new(),
            database_name: "testdb".to_string(),
            username: "testuser".to_string(),
            password: "testpass".to_string(),
        };

        let conn_str = service.build_connection_string();
        assert_eq!(
            conn_str,
            "postgres://testuser:testpass@127.0.0.1:5432/testdb"
        );
    }
}
