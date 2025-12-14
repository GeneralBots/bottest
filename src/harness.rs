use crate::fixtures::{Bot, Customer, Message, QueueEntry, Session, User};
use crate::mocks::{MockLLM, MockZitadel};
use crate::ports::{PortAllocator, TestPorts};
use crate::services::{MinioService, PostgresService, RedisService};
use anyhow::Result;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;
use std::path::PathBuf;
use tokio::sync::OnceCell;
use uuid::Uuid;

pub type DbPool = Pool<ConnectionManager<PgConnection>>;

#[derive(Debug, Clone)]
pub struct TestConfig {
    pub postgres: bool,
    pub minio: bool,
    pub redis: bool,
    pub mock_zitadel: bool,
    pub mock_llm: bool,
    pub run_migrations: bool,
}

impl Default for TestConfig {
    fn default() -> Self {
        Self {
            postgres: true,
            minio: false,
            redis: false,
            mock_zitadel: true,
            mock_llm: true,
            run_migrations: true,
        }
    }
}

impl TestConfig {
    pub fn minimal() -> Self {
        Self {
            postgres: false,
            minio: false,
            redis: false,
            mock_zitadel: false,
            mock_llm: false,
            run_migrations: false,
        }
    }

    pub fn full() -> Self {
        Self {
            postgres: false, // Botserver will bootstrap its own PostgreSQL
            minio: false,    // Botserver will bootstrap its own MinIO
            redis: false,    // Botserver will bootstrap its own Redis
            mock_zitadel: true,
            mock_llm: true,
            run_migrations: false, // Let botserver run its own migrations
        }
    }

    /// Auto-install mode: let botserver bootstrap all services
    /// No need for pre-installed PostgreSQL binaries
    pub fn auto_install() -> Self {
        Self {
            postgres: false, // Botserver will install PostgreSQL
            minio: false,    // Botserver will install MinIO
            redis: false,    // Botserver will install Redis
            mock_zitadel: true,
            mock_llm: true,
            run_migrations: false, // Botserver handles migrations
        }
    }

    pub fn database_only() -> Self {
        Self {
            postgres: true,
            run_migrations: true,
            ..Self::minimal()
        }
    }

    pub fn use_existing_stack() -> Self {
        Self {
            postgres: false,
            minio: false,
            redis: false,
            mock_zitadel: true,
            mock_llm: true,
            run_migrations: false,
        }
    }
}

pub struct DefaultPorts;

impl DefaultPorts {
    pub const POSTGRES: u16 = 5432;
    pub const MINIO: u16 = 9000;
    pub const REDIS: u16 = 6379;
    pub const ZITADEL: u16 = 8080;
    pub const BOTSERVER: u16 = 8080;
}

pub struct TestContext {
    pub ports: TestPorts,
    pub config: TestConfig,
    pub data_dir: PathBuf,
    pub use_existing_stack: bool,
    test_id: Uuid,
    postgres: Option<PostgresService>,
    minio: Option<MinioService>,
    redis: Option<RedisService>,
    mock_zitadel: Option<MockZitadel>,
    mock_llm: Option<MockLLM>,
    db_pool: OnceCell<DbPool>,
    cleaned_up: bool,
}

impl TestContext {
    pub fn test_id(&self) -> Uuid {
        self.test_id
    }

    pub fn database_url(&self) -> String {
        if self.use_existing_stack {
            // For existing stack, use sensible defaults matching botserver's bootstrap
            // These can be overridden via environment variables if needed
            let host = std::env::var("DB_HOST").unwrap_or_else(|_| "127.0.0.1".to_string());
            let port = std::env::var("DB_PORT")
                .ok()
                .and_then(|p| p.parse().ok())
                .unwrap_or(DefaultPorts::POSTGRES);
            // Default to gbuser/botserver which is what botserver bootstrap creates
            let user = std::env::var("DB_USER").unwrap_or_else(|_| "gbuser".to_string());
            let password = std::env::var("DB_PASSWORD").unwrap_or_else(|_| "gbuser".to_string());
            let database = std::env::var("DB_NAME").unwrap_or_else(|_| "botserver".to_string());
            format!(
                "postgres://{}:{}@{}:{}/{}",
                user, password, host, port, database
            )
        } else {
            // For test-managed postgres, use test credentials
            format!(
                "postgres://bottest:bottest@127.0.0.1:{}/bottest",
                self.ports.postgres
            )
        }
    }

    pub fn minio_endpoint(&self) -> String {
        if self.use_existing_stack {
            format!("http://127.0.0.1:{}", DefaultPorts::MINIO)
        } else {
            format!("http://127.0.0.1:{}", self.ports.minio)
        }
    }

    pub fn redis_url(&self) -> String {
        if self.use_existing_stack {
            format!("redis://127.0.0.1:{}", DefaultPorts::REDIS)
        } else {
            format!("redis://127.0.0.1:{}", self.ports.redis)
        }
    }

    pub fn zitadel_url(&self) -> String {
        if self.use_existing_stack {
            format!("https://127.0.0.1:{}", DefaultPorts::ZITADEL)
        } else {
            format!("http://127.0.0.1:{}", self.ports.mock_zitadel)
        }
    }

    pub fn llm_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.ports.mock_llm)
    }

    pub async fn db_pool(&self) -> Result<&DbPool> {
        self.db_pool
            .get_or_try_init(|| async {
                let manager = ConnectionManager::<PgConnection>::new(self.database_url());
                Pool::builder()
                    .max_size(5)
                    .build(manager)
                    .map_err(|e| anyhow::anyhow!("Failed to create pool: {}", e))
            })
            .await
    }

    pub fn mock_zitadel(&self) -> Option<&MockZitadel> {
        self.mock_zitadel.as_ref()
    }

    pub fn mock_llm(&self) -> Option<&MockLLM> {
        self.mock_llm.as_ref()
    }

    pub fn postgres(&self) -> Option<&PostgresService> {
        self.postgres.as_ref()
    }

    pub fn minio(&self) -> Option<&MinioService> {
        self.minio.as_ref()
    }

    pub fn redis(&self) -> Option<&RedisService> {
        self.redis.as_ref()
    }

    pub async fn insert(&self, entity: &dyn Insertable) -> Result<()> {
        let pool = self.db_pool().await?;
        entity.insert(pool)
    }

    pub async fn insert_user(&self, user: &User) -> Result<()> {
        self.insert(user).await
    }

    pub async fn insert_customer(&self, customer: &Customer) -> Result<()> {
        self.insert(customer).await
    }

    pub async fn insert_bot(&self, bot: &Bot) -> Result<()> {
        self.insert(bot).await
    }

    pub async fn insert_session(&self, session: &Session) -> Result<()> {
        self.insert(session).await
    }

    pub async fn insert_message(&self, message: &Message) -> Result<()> {
        self.insert(message).await
    }

    pub async fn insert_queue_entry(&self, entry: &QueueEntry) -> Result<()> {
        self.insert(entry).await
    }

    pub async fn start_botserver(&self) -> Result<BotServerInstance> {
        BotServerInstance::start(self).await
    }

    pub async fn start_botui(&self, botserver_url: &str) -> Result<BotUIInstance> {
        BotUIInstance::start(self, botserver_url).await
    }

    pub async fn cleanup(&mut self) -> Result<()> {
        if self.cleaned_up {
            return Ok(());
        }

        log::info!("Cleaning up test context {}...", self.test_id);

        if let Some(ref mut pg) = self.postgres {
            let _ = pg.stop().await;
        }

        if let Some(ref mut minio) = self.minio {
            let _ = minio.stop().await;
        }

        if let Some(ref mut redis) = self.redis {
            let _ = redis.stop().await;
        }

        if self.data_dir.exists() {
            let _ = std::fs::remove_dir_all(&self.data_dir);
        }

        self.cleaned_up = true;
        Ok(())
    }
}

impl Drop for TestContext {
    fn drop(&mut self) {
        log::info!("Dropping test context {}...", self.test_id);

        if let Some(ref mut pg) = self.postgres {
            let _ = pg.cleanup();
        }

        if let Some(ref mut minio) = self.minio {
            let _ = minio.cleanup();
        }

        if let Some(ref mut redis) = self.redis {
            let _ = redis.cleanup();
        }

        if self.data_dir.exists() && !self.cleaned_up {
            let _ = std::fs::remove_dir_all(&self.data_dir);
        }
    }
}

pub trait Insertable: Send + Sync {
    fn insert(&self, pool: &DbPool) -> Result<()>;
}

impl Insertable for User {
    fn insert(&self, pool: &DbPool) -> Result<()> {
        use diesel::prelude::*;
        use diesel::sql_query;
        use diesel::sql_types::{Text, Timestamptz, Uuid as DieselUuid};

        let mut conn = pool.get()?;
        sql_query(
            "INSERT INTO users (id, email, name, role, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO UPDATE SET email = $2, name = $3, role = $4, updated_at = $6",
        )
        .bind::<DieselUuid, _>(self.id)
        .bind::<Text, _>(&self.email)
        .bind::<Text, _>(&self.name)
        .bind::<Text, _>(format!("{:?}", self.role).to_lowercase())
        .bind::<Timestamptz, _>(self.created_at)
        .bind::<Timestamptz, _>(self.updated_at)
        .execute(&mut conn)?;
        Ok(())
    }
}

impl Insertable for Customer {
    fn insert(&self, pool: &DbPool) -> Result<()> {
        use diesel::prelude::*;
        use diesel::sql_query;
        use diesel::sql_types::{Nullable, Text, Timestamptz, Uuid as DieselUuid};

        let mut conn = pool.get()?;
        sql_query(
            "INSERT INTO customers (id, external_id, phone, email, name, channel, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET external_id = $2, phone = $3, email = $4, name = $5, channel = $6, updated_at = $8",
        )
        .bind::<DieselUuid, _>(self.id)
        .bind::<Text, _>(&self.external_id)
        .bind::<Nullable<Text>, _>(&self.phone)
        .bind::<Nullable<Text>, _>(&self.email)
        .bind::<Nullable<Text>, _>(&self.name)
        .bind::<Text, _>(format!("{:?}", self.channel).to_lowercase())
        .bind::<Timestamptz, _>(self.created_at)
        .bind::<Timestamptz, _>(self.updated_at)
        .execute(&mut conn)?;
        Ok(())
    }
}

impl Insertable for Bot {
    fn insert(&self, pool: &DbPool) -> Result<()> {
        use diesel::prelude::*;
        use diesel::sql_query;
        use diesel::sql_types::{Bool, Nullable, Text, Timestamptz, Uuid as DieselUuid};

        let mut conn = pool.get()?;
        sql_query(
            "INSERT INTO bots (id, name, description, kb_enabled, llm_enabled, llm_model, active, created_at, updated_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
             ON CONFLICT (id) DO UPDATE SET name = $2, description = $3, kb_enabled = $4, llm_enabled = $5, llm_model = $6, active = $7, updated_at = $9",
        )
        .bind::<DieselUuid, _>(self.id)
        .bind::<Text, _>(&self.name)
        .bind::<Nullable<Text>, _>(&self.description)
        .bind::<Bool, _>(self.kb_enabled)
        .bind::<Bool, _>(self.llm_enabled)
        .bind::<Nullable<Text>, _>(&self.llm_model)
        .bind::<Bool, _>(self.active)
        .bind::<Timestamptz, _>(self.created_at)
        .bind::<Timestamptz, _>(self.updated_at)
        .execute(&mut conn)?;
        Ok(())
    }
}

impl Insertable for Session {
    fn insert(&self, pool: &DbPool) -> Result<()> {
        use diesel::prelude::*;
        use diesel::sql_query;
        use diesel::sql_types::{Nullable, Text, Timestamptz, Uuid as DieselUuid};

        let mut conn = pool.get()?;
        sql_query(
            "INSERT INTO sessions (id, bot_id, customer_id, channel, state, started_at, updated_at, ended_at)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET state = $5, updated_at = $7, ended_at = $8",
        )
        .bind::<DieselUuid, _>(self.id)
        .bind::<DieselUuid, _>(self.bot_id)
        .bind::<DieselUuid, _>(self.customer_id)
        .bind::<Text, _>(format!("{:?}", self.channel).to_lowercase())
        .bind::<Text, _>(format!("{:?}", self.state).to_lowercase())
        .bind::<Timestamptz, _>(self.started_at)
        .bind::<Timestamptz, _>(self.updated_at)
        .bind::<Nullable<Timestamptz>, _>(self.ended_at)
        .execute(&mut conn)?;
        Ok(())
    }
}

impl Insertable for Message {
    fn insert(&self, pool: &DbPool) -> Result<()> {
        use diesel::prelude::*;
        use diesel::sql_query;
        use diesel::sql_types::{Text, Timestamptz, Uuid as DieselUuid};

        let mut conn = pool.get()?;
        sql_query(
            "INSERT INTO messages (id, session_id, direction, content, content_type, timestamp)
             VALUES ($1, $2, $3, $4, $5, $6)
             ON CONFLICT (id) DO NOTHING",
        )
        .bind::<DieselUuid, _>(self.id)
        .bind::<DieselUuid, _>(self.session_id)
        .bind::<Text, _>(format!("{:?}", self.direction).to_lowercase())
        .bind::<Text, _>(&self.content)
        .bind::<Text, _>(format!("{:?}", self.content_type).to_lowercase())
        .bind::<Timestamptz, _>(self.timestamp)
        .execute(&mut conn)?;
        Ok(())
    }
}

impl Insertable for QueueEntry {
    fn insert(&self, pool: &DbPool) -> Result<()> {
        use diesel::prelude::*;
        use diesel::sql_query;
        use diesel::sql_types::{Nullable, Text, Timestamptz, Uuid as DieselUuid};

        let mut conn = pool.get()?;
        sql_query(
            "INSERT INTO queue_entries (id, customer_id, session_id, priority, status, entered_at, assigned_at, attendant_id)
             VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
             ON CONFLICT (id) DO UPDATE SET status = $5, assigned_at = $7, attendant_id = $8",
        )
        .bind::<DieselUuid, _>(self.id)
        .bind::<DieselUuid, _>(self.customer_id)
        .bind::<DieselUuid, _>(self.session_id)
        .bind::<Text, _>(format!("{:?}", self.priority).to_lowercase())
        .bind::<Text, _>(format!("{:?}", self.status).to_lowercase())
        .bind::<Timestamptz, _>(self.entered_at)
        .bind::<Nullable<Timestamptz>, _>(self.assigned_at)
        .bind::<Nullable<DieselUuid>, _>(self.attendant_id)
        .execute(&mut conn)?;
        Ok(())
    }
}

pub struct BotServerInstance {
    pub url: String,
    pub port: u16,
    pub stack_path: PathBuf,
    process: Option<std::process::Child>,
}

impl BotServerInstance {
    /// Create an instance pointing to an already-running botserver
    pub fn existing(url: &str) -> Self {
        let port = url
            .split(':')
            .last()
            .and_then(|p| p.parse().ok())
            .unwrap_or(8080);
        Self {
            url: url.to_string(),
            port,
            stack_path: PathBuf::from("./botserver-stack"),
            process: None,
        }
    }
}

pub struct BotUIInstance {
    pub url: String,
    pub port: u16,
    process: Option<std::process::Child>,
}

impl BotUIInstance {
    /// Create an instance pointing to an already-running botui
    pub fn existing(url: &str) -> Self {
        let port = url
            .split(':')
            .last()
            .and_then(|p| p.parse().ok())
            .unwrap_or(3000);
        Self {
            url: url.to_string(),
            port,
            process: None,
        }
    }
}

impl BotUIInstance {
    pub async fn start(ctx: &TestContext, botserver_url: &str) -> Result<Self> {
        let port = crate::ports::PortAllocator::allocate();
        let url = format!("http://127.0.0.1:{}", port);

        let botui_bin = std::env::var("BOTUI_BIN")
            .unwrap_or_else(|_| "../botui/target/debug/botui".to_string());

        // Check if binary exists
        if !PathBuf::from(&botui_bin).exists() {
            log::warn!("BotUI binary not found at: {}", botui_bin);
            return Ok(Self {
                url,
                port,
                process: None,
            });
        }

        log::info!("Starting botui from: {} on port {}", botui_bin, port);
        log::info!("  BOTUI_PORT={}", port);
        log::info!("  BOTSERVER_URL={}", botserver_url);

        // botui uses env vars, not command line args
        let process = std::process::Command::new(&botui_bin)
            .env("BOTUI_PORT", port.to_string())
            .env("BOTSERVER_URL", botserver_url)
            .env_remove("RUST_LOG")
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .ok();

        if process.is_some() {
            // Wait for botui to be ready
            let max_wait = 30;
            log::info!("Waiting for botui to become ready... (max {}s)", max_wait);
            for i in 0..max_wait {
                if let Ok(resp) = reqwest::get(&format!("{}/health", url)).await {
                    if resp.status().is_success() {
                        log::info!("BotUI is ready on port {}", port);
                        return Ok(Self { url, port, process });
                    }
                }
                // Also try root path in case /health isn't implemented
                if let Ok(resp) = reqwest::get(&url).await {
                    if resp.status().is_success() {
                        log::info!("BotUI is ready on port {}", port);
                        return Ok(Self { url, port, process });
                    }
                }
                if i % 5 == 0 {
                    log::info!("Still waiting for botui... ({}s)", i);
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            log::warn!("BotUI did not respond in time");
        }

        Ok(Self {
            url,
            port,
            process: None,
        })
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }
}

impl Drop for BotUIInstance {
    fn drop(&mut self) {
        if let Some(ref mut child) = self.process {
            let _ = child.kill();
            let _ = child.wait();
        }
    }
}

impl BotServerInstance {
    /// Start botserver, creating a fresh stack from scratch for testing
    pub async fn start(ctx: &TestContext) -> Result<Self> {
        let port = ctx.ports.botserver;
        let url = format!("http://127.0.0.1:{}", port);

        // Create a clean test stack directory for this test run
        // Use absolute path since we'll change working directory for botserver
        let stack_path = ctx.data_dir.join("botserver-stack");
        std::fs::create_dir_all(&stack_path)?;
        let stack_path = stack_path.canonicalize().unwrap_or(stack_path);
        log::info!("Created clean test stack at: {:?}", stack_path);

        let botserver_bin = std::env::var("BOTSERVER_BIN")
            .unwrap_or_else(|_| "../botserver/target/debug/botserver".to_string());

        // Check if binary exists
        if !PathBuf::from(&botserver_bin).exists() {
            log::warn!("Botserver binary not found at: {}", botserver_bin);
            return Ok(Self {
                url,
                port,
                stack_path,
                process: None,
            });
        }

        log::info!("Starting botserver from: {}", botserver_bin);

        // Determine botserver working directory to find installers in botserver-installers/
        // The botserver binary is typically at ../botserver/target/release/botserver
        // We need to run from ../botserver so it finds botserver-installers/ and 3rdparty.toml
        let botserver_bin_path =
            std::fs::canonicalize(&botserver_bin).unwrap_or_else(|_| PathBuf::from(&botserver_bin));
        let botserver_dir = botserver_bin_path
            .parent() // target/release
            .and_then(|p| p.parent()) // target
            .and_then(|p| p.parent()) // botserver
            .map(|p| p.to_path_buf())
            .unwrap_or_else(|| {
                std::fs::canonicalize("../botserver")
                    .unwrap_or_else(|_| PathBuf::from("../botserver"))
            });

        log::info!("Botserver working directory: {:?}", botserver_dir);
        log::info!("Stack path (absolute): {:?}", stack_path);

        // Start botserver with test configuration
        // - Uses test harness PostgreSQL
        // - Uses mock Zitadel for auth
        // - Uses mock LLM
        // Env vars align with SecretsManager fallbacks (see botserver/src/core/secrets/mod.rs)
        // Use absolute path for binary since we're changing working directory

        // Point to local installers directory to avoid downloads
        let installers_path = botserver_dir.join("botserver-installers");
        let installers_path = installers_path.canonicalize().unwrap_or(installers_path);
        log::info!("Using installers from: {:?}", installers_path);

        let process = std::process::Command::new(&botserver_bin_path)
            .current_dir(&botserver_dir) // Run from botserver dir to find installers
            .arg("--stack-path")
            .arg(&stack_path)
            .arg("--port")
            .arg(port.to_string())
            .arg("--noconsole")
            .env_remove("RUST_LOG") // Remove to avoid logger conflict
            // Use local installers - DO NOT download
            .env("BOTSERVER_INSTALLERS_PATH", &installers_path)
            // Skip local LLM server startup - tests use mock LLM
            .env("SKIP_LLM_SERVER", "1")
            // Database - DATABASE_URL is the standard fallback
            .env("DATABASE_URL", ctx.database_url())
            // Directory (Zitadel) - use SecretsManager fallback env vars
            .env("DIRECTORY_URL", ctx.zitadel_url())
            .env("ZITADEL_CLIENT_ID", "test-client-id")
            .env("ZITADEL_CLIENT_SECRET", "test-client-secret")
            // Drive (MinIO) - use SecretsManager fallback env vars
            .env("DRIVE_ACCESSKEY", "minioadmin")
            .env("DRIVE_SECRET", "minioadmin")
            // Always let botserver bootstrap services (PostgreSQL, MinIO, Redis, etc.)
            // No BOTSERVER_SKIP_INSTALL - we want full bootstrap
            .stdout(std::process::Stdio::inherit())
            .stderr(std::process::Stdio::inherit())
            .spawn()
            .ok();

        if process.is_some() {
            // Give time for botserver bootstrap (needs to download Vault, PostgreSQL, etc.)
            let max_wait = 600;
            log::info!(
                "Waiting for botserver to bootstrap and become ready... (max {}s)",
                max_wait
            );
            // Give more time for botserver to bootstrap services
            for i in 0..max_wait {
                if let Ok(resp) = reqwest::get(&format!("{}/health", url)).await {
                    if resp.status().is_success() {
                        log::info!("Botserver is ready on port {}", port);
                        return Ok(Self {
                            url,
                            port,
                            stack_path,
                            process,
                        });
                    }
                }
                if i % 10 == 0 {
                    log::info!("Still waiting for botserver... ({}s)", i);
                }
                tokio::time::sleep(std::time::Duration::from_secs(1)).await;
            }
            log::warn!("Botserver did not respond to health check in time");
        }

        Ok(Self {
            url,
            port,
            stack_path,
            process: None,
        })
    }

    pub fn is_running(&self) -> bool {
        self.process.is_some()
    }

    /// Setup minimal config files so botserver thinks services are configured
    fn setup_test_stack_config(stack_path: &PathBuf, ctx: &TestContext) -> Result<()> {
        // Create directory config path
        let directory_conf = stack_path.join("conf/directory");
        std::fs::create_dir_all(&directory_conf)?;

        // Create zitadel.yaml pointing to our mock Zitadel
        let zitadel_config = format!(
            r#"Log:
  Level: info

Database:
  postgres:
    Host: 127.0.0.1
    Port: {}
    Database: bottest
    User: bottest
    Password: "bottest"
    SSL:
      Mode: disable

ExternalSecure: false
ExternalDomain: localhost
ExternalPort: {}
"#,
            ctx.ports.postgres, ctx.ports.mock_zitadel
        );

        std::fs::write(directory_conf.join("zitadel.yaml"), zitadel_config)?;
        log::info!("Created test zitadel.yaml config");

        // Create system certificates directory
        let certs_dir = stack_path.join("conf/system/certificates");
        std::fs::create_dir_all(&certs_dir)?;

        // Generate minimal self-signed certificates for API
        Self::generate_test_certificates(&certs_dir)?;

        Ok(())
    }

    /// Generate minimal test certificates
    fn generate_test_certificates(certs_dir: &PathBuf) -> Result<()> {
        use std::process::Command;

        let api_dir = certs_dir.join("api");
        std::fs::create_dir_all(&api_dir)?;

        // Check if openssl is available
        let openssl_check = Command::new("which").arg("openssl").output();
        if openssl_check.map(|o| o.status.success()).unwrap_or(false) {
            // Generate self-signed certificate using openssl
            let key_path = api_dir.join("server.key");
            let cert_path = api_dir.join("server.crt");

            if !key_path.exists() {
                let _ = Command::new("openssl")
                    .args([
                        "req",
                        "-x509",
                        "-newkey",
                        "rsa:2048",
                        "-keyout",
                        key_path.to_str().unwrap(),
                        "-out",
                        cert_path.to_str().unwrap(),
                        "-days",
                        "1",
                        "-nodes",
                        "-subj",
                        "/CN=localhost",
                    ])
                    .output();
                log::info!("Generated test TLS certificates");
            }
        } else {
            log::warn!("openssl not found, skipping certificate generation");
        }

        Ok(())
    }
}

impl Drop for BotServerInstance {
    fn drop(&mut self) {
        if let Some(ref mut process) = self.process {
            let _ = process.kill();
            let _ = process.wait();
        }
    }
}

pub struct TestHarness;

impl TestHarness {
    pub async fn setup(config: TestConfig) -> Result<TestContext> {
        Self::setup_internal(config, false).await
    }

    pub async fn with_existing_stack() -> Result<TestContext> {
        Self::setup_internal(TestConfig::use_existing_stack(), true).await
    }

    async fn setup_internal(config: TestConfig, use_existing_stack: bool) -> Result<TestContext> {
        let _ = env_logger::builder().is_test(true).try_init();

        let test_id = Uuid::new_v4();
        let data_dir = PathBuf::from("./tmp").join(format!("bottest-{}", test_id));

        std::fs::create_dir_all(&data_dir)?;

        let ports = if use_existing_stack {
            TestPorts {
                postgres: DefaultPorts::POSTGRES,
                minio: DefaultPorts::MINIO,
                redis: DefaultPorts::REDIS,
                botserver: PortAllocator::allocate(),
                mock_zitadel: PortAllocator::allocate(),
                mock_llm: PortAllocator::allocate(),
            }
        } else {
            TestPorts::allocate()
        };

        log::info!(
            "Test {} allocated ports: {:?}, data_dir: {:?}, use_existing_stack: {}",
            test_id,
            ports,
            data_dir,
            use_existing_stack
        );

        let data_dir_str = data_dir.to_str().unwrap().to_string();

        let mut ctx = TestContext {
            ports,
            config: config.clone(),
            data_dir,
            use_existing_stack,
            test_id,
            postgres: None,
            minio: None,
            redis: None,
            mock_zitadel: None,
            mock_llm: None,
            db_pool: OnceCell::new(),
            cleaned_up: false,
        };

        if config.postgres {
            log::info!("Starting PostgreSQL on port {}...", ctx.ports.postgres);
            let pg = PostgresService::start(ctx.ports.postgres, &data_dir_str).await?;
            if config.run_migrations {
                pg.run_migrations().await?;
            }
            ctx.postgres = Some(pg);
        }

        if config.minio {
            log::info!("Starting MinIO on port {}...", ctx.ports.minio);
            ctx.minio = Some(MinioService::start(ctx.ports.minio, &data_dir_str).await?);
        }

        if config.redis {
            log::info!("Starting Redis on port {}...", ctx.ports.redis);
            ctx.redis = Some(RedisService::start(ctx.ports.redis, &data_dir_str).await?);
        }

        if config.mock_zitadel {
            log::info!(
                "Starting mock Zitadel on port {}...",
                ctx.ports.mock_zitadel
            );
            ctx.mock_zitadel = Some(MockZitadel::start(ctx.ports.mock_zitadel).await?);
        }

        if config.mock_llm {
            log::info!("Starting mock LLM on port {}...", ctx.ports.mock_llm);
            ctx.mock_llm = Some(MockLLM::start(ctx.ports.mock_llm).await?);
        }

        Ok(ctx)
    }

    pub async fn quick() -> Result<TestContext> {
        Self::setup(TestConfig::default()).await
    }

    pub async fn full() -> Result<TestContext> {
        if std::env::var("USE_EXISTING_STACK").is_ok() {
            Self::with_existing_stack().await
        } else {
            Self::setup(TestConfig::full()).await
        }
    }

    /// Setup with botserver auto-installing all services
    pub async fn with_auto_install() -> Result<TestContext> {
        Self::setup(TestConfig::auto_install()).await
    }

    pub async fn minimal() -> Result<TestContext> {
        Self::setup(TestConfig::minimal()).await
    }

    pub async fn database_only() -> Result<TestContext> {
        Self::setup(TestConfig::database_only()).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_minimal_harness() {
        let ctx = TestHarness::minimal().await.unwrap();
        assert!(ctx.ports.postgres >= 15000);
        assert!(ctx.data_dir.to_str().unwrap().contains("bottest-"));
    }

    #[test]
    fn test_config_default() {
        let config = TestConfig::default();
        assert!(config.postgres);
        assert!(!config.minio);
        assert!(!config.redis);
        assert!(config.mock_zitadel);
        assert!(config.mock_llm);
        assert!(config.run_migrations);
    }

    #[test]
    fn test_config_full() {
        let config = TestConfig::full();
        assert!(!config.postgres); // Botserver handles PostgreSQL
        assert!(!config.minio); // Botserver handles MinIO
        assert!(!config.redis); // Botserver handles Redis
        assert!(config.mock_zitadel);
        assert!(config.mock_llm);
        assert!(!config.run_migrations); // Botserver handles migrations
    }

    #[test]
    fn test_config_minimal() {
        let config = TestConfig::minimal();
        assert!(!config.postgres);
        assert!(!config.minio);
        assert!(!config.redis);
        assert!(!config.mock_zitadel);
        assert!(!config.mock_llm);
        assert!(!config.run_migrations);
    }

    #[test]
    fn test_config_database_only() {
        let config = TestConfig::database_only();
        assert!(config.postgres);
        assert!(!config.minio);
        assert!(!config.redis);
        assert!(!config.mock_zitadel);
        assert!(!config.mock_llm);
        assert!(config.run_migrations);
    }
}
