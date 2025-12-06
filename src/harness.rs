use crate::fixtures::{Bot, Customer, Message, QueueEntry, Session, User};
use crate::mocks::{MockLLM, MockZitadel};
use crate::ports::TestPorts;
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
            postgres: true,
            minio: true,
            redis: true,
            mock_zitadel: true,
            mock_llm: true,
            run_migrations: true,
        }
    }

    pub fn database_only() -> Self {
        Self {
            postgres: true,
            run_migrations: true,
            ..Self::minimal()
        }
    }
}

pub struct TestContext {
    pub ports: TestPorts,
    pub config: TestConfig,
    pub data_dir: PathBuf,
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
        format!(
            "postgres://bottest:bottest@127.0.0.1:{}/bottest",
            self.ports.postgres
        )
    }

    pub fn minio_endpoint(&self) -> String {
        format!("http://127.0.0.1:{}", self.ports.minio)
    }

    pub fn redis_url(&self) -> String {
        format!("redis://127.0.0.1:{}", self.ports.redis)
    }

    pub fn zitadel_url(&self) -> String {
        format!("http://127.0.0.1:{}", self.ports.mock_zitadel)
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
    process: Option<std::process::Child>,
}

impl BotServerInstance {
    pub async fn start(ctx: &TestContext) -> Result<Self> {
        let port = ctx.ports.botserver;
        let url = format!("http://127.0.0.1:{}", port);

        let botserver_bin =
            std::env::var("BOTSERVER_BIN").unwrap_or_else(|_| "botserver".to_string());

        let process = std::process::Command::new(&botserver_bin)
            .arg("--port")
            .arg(port.to_string())
            .arg("--database-url")
            .arg(ctx.database_url())
            .env("ZITADEL_URL", ctx.zitadel_url())
            .env("LLM_URL", ctx.llm_url())
            .env("MINIO_ENDPOINT", ctx.minio_endpoint())
            .env("REDIS_URL", ctx.redis_url())
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .spawn()
            .ok();

        if process.is_some() {
            for _ in 0..50 {
                if let Ok(resp) = reqwest::get(&format!("{}/health", url)).await {
                    if resp.status().is_success() {
                        return Ok(Self { url, port, process });
                    }
                }
                tokio::time::sleep(std::time::Duration::from_millis(100)).await;
            }
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
        let _ = env_logger::builder().is_test(true).try_init();

        let test_id = Uuid::new_v4();
        let data_dir = PathBuf::from("./tmp").join(format!("bottest-{}", test_id));

        std::fs::create_dir_all(&data_dir)?;

        let ports = TestPorts::allocate();
        log::info!(
            "Test {} allocated ports: {:?}, data_dir: {:?}",
            test_id,
            ports,
            data_dir
        );

        let data_dir_str = data_dir.to_str().unwrap().to_string();

        let mut ctx = TestContext {
            ports,
            config: config.clone(),
            data_dir,
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
        Self::setup(TestConfig::full()).await
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
        assert!(config.postgres);
        assert!(config.minio);
        assert!(config.redis);
        assert!(config.mock_zitadel);
        assert!(config.mock_llm);
        assert!(config.run_migrations);
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
