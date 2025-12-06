# Temporary Stack Setup Guide

## Overview

The temporary stack feature allows you to spawn an isolated BotServer environment for testing purposes. This guide explains how to implement and use this feature.

## Architecture

### What is a Temporary Stack?

A temporary stack is a self-contained, isolated instance of the General Bots platform that:
- Runs in a dedicated temporary directory
- Uses isolated database, cache, and storage
- Can be spawned and torn down automatically
- Doesn't interfere with your main development environment
- Perfect for E2E testing and integration tests

### File Structure

```
/tmp/botserver-test-{timestamp}-{random}/
├── postgres/
│   ├── data/               # PostgreSQL data directory
│   ├── postgres.log        # Database logs
│   └── postgresql.conf     # Database config
├── redis/
│   ├── data/               # Redis persistence
│   └── redis.log           # Redis logs
├── minio/
│   ├── data/               # S3-compatible storage
│   └── minio.log           # MinIO logs
├── botserver/
│   ├── config/             # BotServer configuration
│   ├── logs/
│   │   ├── botserver.log   # Main application logs
│   │   ├── api.log         # API logs
│   │   └── debug.log       # Debug logs
│   ├── cache/              # Local cache directory
│   └── state.json          # Stack state and metadata
└── env.stack              # Environment variables for this stack
```

## Implementation: BotServer Changes

### 1. Add CLI Arguments

Update `botserver/src/main.rs`:

```rust
use clap::Parser;
use std::path::PathBuf;
use std::time::Duration;

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Args {
    /// Enable temporary stack mode for testing
    #[arg(long)]
    temp_stack: bool,

    /// Custom temporary stack root directory
    /// If not provided, uses /tmp/botserver-test-{timestamp}-{random}
    #[arg(long)]
    stack_root: Option<PathBuf>,

    /// Timeout in seconds for temporary stack auto-shutdown
    /// Useful for CI/CD pipelines
    #[arg(long)]
    temp_stack_timeout: Option<u64>,

    /// Keep temporary stack directory after shutdown (for debugging)
    #[arg(long)]
    keep_temp_stack: bool,

    // ... existing arguments ...
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    if args.temp_stack {
        return run_temp_stack(args).await;
    }

    // ... normal startup code ...
}
```

### 2. Implement Temporary Stack Manager

Create `botserver/src/temp_stack.rs`:

```rust
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Child};
use chrono::Local;
use uuid::Uuid;
use anyhow::{anyhow, Context};
use log::{info, debug};
use tokio::time::{sleep, Duration};

pub struct TemporaryStack {
    pub root_dir: PathBuf,
    pub postgres_dir: PathBuf,
    pub redis_dir: PathBuf,
    pub minio_dir: PathBuf,
    pub botserver_dir: PathBuf,
    pub postgres_process: Option<Child>,
    pub redis_process: Option<Child>,
    pub minio_process: Option<Child>,
    pub botserver_process: Option<Child>,
    pub keep_on_shutdown: bool,
    pub auto_shutdown_duration: Option<Duration>,
}

impl TemporaryStack {
    /// Create and initialize a new temporary stack
    pub async fn new(
        custom_root: Option<PathBuf>,
        keep_on_shutdown: bool,
        auto_shutdown: Option<u64>,
    ) -> anyhow::Result<Self> {
        // Generate unique directory name
        let timestamp = Local::now().format("%Y%m%d-%H%M%S");
        let unique_id = Uuid::new_v4().to_string()[..8].to_string();
        let dir_name = format!("botserver-test-{}-{}", timestamp, unique_id);

        let root_dir = match custom_root {
            Some(p) => p.join(&dir_name),
            None => std::env::temp_dir().join(dir_name),
        };

        info!("Creating temporary stack at: {}", root_dir.display());

        // Create directory structure
        fs::create_dir_all(&root_dir)
            .context("Failed to create temp stack root directory")?;

        let postgres_dir = root_dir.join("postgres");
        let redis_dir = root_dir.join("redis");
        let minio_dir = root_dir.join("minio");
        let botserver_dir = root_dir.join("botserver");

        fs::create_dir_all(&postgres_dir)?;
        fs::create_dir_all(&redis_dir)?;
        fs::create_dir_all(&minio_dir)?;
        fs::create_dir_all(&botserver_dir)?;

        let auto_shutdown_duration = auto_shutdown.map(Duration::from_secs);

        Ok(Self {
            root_dir,
            postgres_dir,
            redis_dir,
            minio_dir,
            botserver_dir,
            postgres_process: None,
            redis_process: None,
            minio_process: None,
            botserver_process: None,
            keep_on_shutdown,
            auto_shutdown_duration,
        })
    }

    /// Start all services in the temporary stack
    pub async fn start_services(&mut self) -> anyhow::Result<()> {
        info!("Starting temporary stack services");

        // Start PostgreSQL
        self.start_postgres().await?;
        sleep(Duration::from_secs(2)).await;

        // Start Redis
        self.start_redis().await?;
        sleep(Duration::from_secs(1)).await;

        // Start MinIO
        self.start_minio().await?;
        sleep(Duration::from_secs(1)).await;

        info!("All temporary stack services started");
        Ok(())
    }

    /// Start PostgreSQL
    async fn start_postgres(&mut self) -> anyhow::Result<()> {
        info!("Starting PostgreSQL");

        let data_dir = self.postgres_dir.join("data");
        fs::create_dir_all(&data_dir)?;

        // Initialize PostgreSQL cluster if needed
        let initdb_output = Command::new("initdb")
            .arg("-D")
            .arg(&data_dir)
            .output();

        if initdb_output.is_ok() {
            debug!("Initialized PostgreSQL cluster");
        }

        let process = Command::new("postgres")
            .arg("-D")
            .arg(&data_dir)
            .arg("-p")
            .arg("5433") // Use different port than default
            .spawn()
            .context("Failed to start PostgreSQL")?;

        self.postgres_process = Some(process);
        info!("PostgreSQL started on port 5433");
        Ok(())
    }

    /// Start Redis
    async fn start_redis(&mut self) -> anyhow::Result<()> {
        info!("Starting Redis");

        let data_dir = self.redis_dir.join("data");
        fs::create_dir_all(&data_dir)?;

        let process = Command::new("redis-server")
            .arg("--port")
            .arg("6380") // Use different port than default
            .arg("--dir")
            .arg(&data_dir)
            .spawn()
            .context("Failed to start Redis")?;

        self.redis_process = Some(process);
        info!("Redis started on port 6380");
        Ok(())
    }

    /// Start MinIO
    async fn start_minio(&mut self) -> anyhow::Result<()> {
        info!("Starting MinIO");

        let data_dir = self.minio_dir.join("data");
        fs::create_dir_all(&data_dir)?;

        let process = Command::new("minio")
            .arg("server")
            .arg(&data_dir)
            .arg("--address")
            .arg("127.0.0.1:9001") // Use different port than default
            .spawn()
            .context("Failed to start MinIO")?;

        self.minio_process = Some(process);
        info!("MinIO started on port 9001");
        Ok(())
    }

    /// Write environment configuration for this stack
    pub fn write_env_config(&self) -> anyhow::Result<()> {
        let env_content = format!(
            r#"# Temporary Stack Configuration
# Generated at: {}

# Stack Identity
BOTSERVER_STACK_ID={}
BOTSERVER_TEMP_STACK_DIR={}
BOTSERVER_KEEP_ON_SHUTDOWN={}

# Database
DATABASE_URL=postgres://botuser:botpass@127.0.0.1:5433/botserver
DB_HOST=127.0.0.1
DB_PORT=5433
DB_NAME=botserver
DB_USER=botuser
DB_PASSWORD=botpass

# Cache
REDIS_URL=redis://127.0.0.1:6380
REDIS_HOST=127.0.0.1
REDIS_PORT=6380

# Storage
MINIO_URL=http://127.0.0.1:9001
MINIO_ACCESS_KEY=minioadmin
MINIO_SECRET_KEY=minioadmin
MINIO_BUCKET=botserver

# API
API_HOST=127.0.0.1
API_PORT=8000
API_URL=http://127.0.0.1:8000

# Logging
LOG_LEVEL=debug
LOG_FILE={}/botserver.log
"#,
            chrono::Local::now(),
            Uuid::new_v4(),
            self.root_dir.display(),
            self.keep_on_shutdown,
            self.botserver_dir.display(),
        );

        let env_file = self.root_dir.join("env.stack");
        fs::write(&env_file, env_content)
            .context("Failed to write environment configuration")?;

        info!("Environment configuration written to: {}", env_file.display());
        Ok(())
    }

    /// Wait for all services to be ready
    pub async fn wait_ready(&self, timeout: Duration) -> anyhow::Result<()> {
        let start = std::time::Instant::now();

        // Check PostgreSQL
        loop {
            if start.elapsed() > timeout {
                return Err(anyhow!("Timeout waiting for PostgreSQL"));
            }
            match Command::new("pg_isready")
                .arg("-h")
                .arg("127.0.0.1")
                .arg("-p")
                .arg("5433")
                .output()
            {
                Ok(output) if output.status.success() => break,
                _ => sleep(Duration::from_millis(100)).await,
            }
        }
        info!("PostgreSQL is ready");

        // Check Redis
        loop {
            if start.elapsed() > timeout {
                return Err(anyhow!("Timeout waiting for Redis"));
            }
            match Command::new("redis-cli")
                .arg("-p")
                .arg("6380")
                .arg("ping")
                .output()
            {
                Ok(output) if output.status.success() => break,
                _ => sleep(Duration::from_millis(100)).await,
            }
        }
        info!("Redis is ready");

        Ok(())
    }

    /// Gracefully shutdown all services
    pub async fn shutdown(&mut self) -> anyhow::Result<()> {
        info!("Shutting down temporary stack");

        // Stop BotServer
        if let Some(mut proc) = self.botserver_process.take() {
            let _ = proc.kill();
        }

        // Stop services
        if let Some(mut proc) = self.minio_process.take() {
            let _ = proc.kill();
        }
        if let Some(mut proc) = self.redis_process.take() {
            let _ = proc.kill();
        }
        if let Some(mut proc) = self.postgres_process.take() {
            let _ = proc.kill();
        }

        sleep(Duration::from_millis(500)).await;

        // Cleanup directory if not keeping
        if !self.keep_on_shutdown {
            if let Err(e) = fs::remove_dir_all(&self.root_dir) {
                log::warn!("Failed to cleanup temp stack directory: {}", e);
            } else {
                info!("Temporary stack cleaned up: {}", self.root_dir.display());
            }
        } else {
            info!("Keeping temporary stack at: {}", self.root_dir.display());
        }

        Ok(())
    }
}

impl Drop for TemporaryStack {
    fn drop(&mut self) {
        if let Err(e) = tokio::runtime::Handle::current().block_on(self.shutdown()) {
            log::error!("Error during temporary stack cleanup: {}", e);
        }
    }
}
```

### 3. Integration in Main

Add to `botserver/src/main.rs`:

```rust
mod temp_stack;
use temp_stack::TemporaryStack;

async fn run_temp_stack(args: Args) -> anyhow::Result<()> {
    // Setup logging
    env_logger::Builder::from_default_env()
        .filter_level(log::LevelFilter::Info)
        .try_init()?;

    info!("Starting BotServer in temporary stack mode");

    // Create temporary stack
    let mut temp_stack = TemporaryStack::new(
        args.stack_root,
        args.keep_temp_stack,
        args.temp_stack_timeout,
    ).await?;

    // Start services
    temp_stack.start_services().await?;
    temp_stack.write_env_config()?;

    // Wait for services to be ready
    temp_stack.wait_ready(Duration::from_secs(30)).await?;

    info!("Temporary stack ready!");
    info!("Stack directory: {}", temp_stack.root_dir.display());
    info!("Environment config: {}/env.stack", temp_stack.root_dir.display());

    // Setup auto-shutdown timer if specified
    if let Some(timeout) = temp_stack.auto_shutdown_duration {
        tokio::spawn(async move {
            sleep(timeout).await;
            info!("Auto-shutdown timeout reached, shutting down");
            std::process::exit(0);
        });
    }

    // Continue with normal BotServer startup using the temp stack config
    run_botserver_with_stack(temp_stack).await
}
```

## Using Temporary Stack in Tests

### In Test Harness

```rust
// bottest/src/harness.rs

pub struct TemporaryStackHandle {
    stack: TemporaryStack,
}

impl TestHarness {
    pub async fn with_temp_stack() -> anyhow::Result<Self> {
        let mut stack = TemporaryStack::new(None, false, None).await?;
        stack.start_services().await?;
        stack.wait_ready(Duration::from_secs(30)).await?;

        // Load environment from stack config
        let env_file = stack.root_dir.join("env.stack");
        load_env_file(&env_file)?;

        // Create harness with temp stack
        let mut harness = Self::new();
        harness.temp_stack = Some(stack);
        Ok(harness)
    }
}
```

### In E2E Tests

```rust
#[tokio::test]
async fn test_with_temp_stack() {
    // Spawn temporary stack
    let mut harness = TestHarness::with_temp_stack()
        .await
        .expect("Failed to create temp stack");

    // Tests run in isolation
    // Stack automatically cleaned up on drop
}
```

## Running Tests with Temporary Stack

```bash
# Run E2E test with automatic temporary stack
cargo test --test e2e test_complete_platform_flow -- --nocapture

# Keep temporary stack for debugging on failure
KEEP_TEMP_STACK_ON_ERROR=1 cargo test --test e2e -- --nocapture

# Use custom temporary directory
cargo test --test e2e -- --nocapture \
    --temp-stack-root /var/tmp/bottest

# Run with browser UI visible
HEADED=1 cargo test --test e2e -- --nocapture

# Run with auto-shutdown after 5 minutes (300 seconds)
cargo test --test e2e -- --nocapture \
    --temp-stack-timeout 300
```

## Environment Variables

Control temporary stack behavior with environment variables:

| Variable | Default | Description |
|----------|---------|-------------|
| `SKIP_E2E_TESTS` | unset | Skip E2E tests if set |
| `HEADED` | unset | Show browser UI instead of headless |
| `KEEP_TEMP_STACK_ON_ERROR` | unset | Keep temp directory if test fails |
| `WEBDRIVER_URL` | `http://localhost:4444` | WebDriver endpoint for browser automation |
| `LOG_LEVEL` | `info` | Logging level: debug, info, warn, error |
| `TEMP_STACK_TIMEOUT` | unset | Auto-shutdown timeout in seconds |

## Troubleshooting

### PostgreSQL fails to start

```bash
# Make sure PostgreSQL binaries are installed
which postgres initdb pg_isready

# Check if port 5433 is available
lsof -i :5433

# Initialize manually
initdb -D /tmp/botserver-test-*/postgres/data
```

### Redis fails to start

```bash
# Verify Redis is installed
which redis-server redis-cli

# Check if port 6380 is available
lsof -i :6380
```

### Cleanup issues

```bash
# Manually cleanup stale directories
rm -rf /tmp/botserver-test-*

# Keep temporary stack for debugging
KEEP_TEMP_STACK_ON_ERROR=1 cargo test --test e2e
```

### Check stack logs

```bash
# View BotServer logs
tail -f /tmp/botserver-test-{id}/botserver/logs/botserver.log

# View database logs
tail -f /tmp/botserver-test-{id}/postgres.log

# View all logs
ls -la /tmp/botserver-test-{id}/*/
```

## Benefits Summary

✓ **Isolation** - Each test has its own environment
✓ **Automation** - No manual setup required
✓ **Reproducibility** - Same setup every time
✓ **Safety** - Won't interfere with main development
✓ **Cleanup** - Automatic resource management
✓ **Debugging** - Can preserve stacks for investigation
✓ **CI/CD Ready** - Perfect for automated testing pipelines
✓ **Scalability** - Run multiple tests in parallel with port management