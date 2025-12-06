# BotTest - Comprehensive Test Infrastructure

**Version:** 6.1.0  
**Status:** Production-ready test framework  
**Architecture:** Isolated ephemeral environments with real services

---

## Overview

BotTest provides enterprise-grade testing infrastructure for the General Bots ecosystem. Each test run creates a completely isolated environment with real PostgreSQL, MinIO, and Redis instances on dynamic ports, ensuring zero state pollution between tests and enabling full parallel execution.

---

## Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        Test Harness                              │
├─────────────────────────────────────────────────────────────────┤
│  ./tmp/bottest-{uuid}/                                          │
│  ├── postgres/     (data + socket)                              │
│  ├── minio/        (buckets)                                    │
│  ├── redis/        (dump.rdb)                                   │
│  └── logs/         (service logs)                               │
├─────────────────────────────────────────────────────────────────┤
│  Dynamic Port Allocation (49152-65535)                          │
│  ├── PostgreSQL    :random                                      │
│  ├── MinIO API     :random                                      │
│  ├── MinIO Console :random                                      │
│  └── Redis         :random                                      │
├─────────────────────────────────────────────────────────────────┤
│  Mock Servers (wiremock)                                        │
│  ├── LLM API       (OpenAI-compatible)                          │
│  ├── WhatsApp      (Business API)                               │
│  ├── Teams         (Bot Framework)                              │
│  └── Zitadel       (Auth/OIDC)                                  │
└─────────────────────────────────────────────────────────────────┘
```

---

## Core Components

### Test Harness

Orchestrates complete test lifecycle with automatic cleanup:

```rust
pub struct TestHarness {
    pub id: Uuid,
    pub root_dir: PathBuf,
    pub postgres: PostgresService,
    pub minio: MinioService,
    pub redis: RedisService,
    pub mocks: MockRegistry,
}

impl TestHarness {
    pub async fn new(config: TestConfig) -> Result<Self>;
    pub async fn with_botserver(&self) -> Result<BotServerProcess>;
    pub fn connection_string(&self) -> String;
    pub fn s3_endpoint(&self) -> String;
}

impl Drop for TestHarness {
    fn drop(&mut self) {
        // Graceful shutdown + cleanup ./tmp/bottest-{uuid}/
    }
}
```

### Service Management

Real services via botserver bootstrap (no Docker dependency):

```rust
impl PostgresService {
    pub async fn start(port: u16, data_dir: &Path) -> Result<Self>;
    pub async fn run_migrations(&self) -> Result<()>;
    pub async fn create_database(&self, name: &str) -> Result<()>;
    pub async fn execute(&self, sql: &str) -> Result<()>;
    pub fn connection_string(&self) -> String;
}

impl MinioService {
    pub async fn start(api_port: u16, console_port: u16, data_dir: &Path) -> Result<Self>;
    pub async fn create_bucket(&self, name: &str) -> Result<()>;
    pub fn endpoint(&self) -> String;
    pub fn credentials(&self) -> (String, String);
}

impl RedisService {
    pub async fn start(port: u16, data_dir: &Path) -> Result<Self>;
    pub fn connection_string(&self) -> String;
}
```

### Mock Servers

Flexible expectation-based mocking:

```rust
impl MockLLM {
    pub async fn start(port: u16) -> Result<Self>;
    pub fn expect_completion(&mut self, prompt_contains: &str, response: &str) -> &mut Self;
    pub fn expect_streaming(&mut self, chunks: Vec<&str>) -> &mut Self;
    pub fn expect_embedding(&mut self, dimensions: usize) -> &mut Self;
    pub fn with_latency(&mut self, ms: u64) -> &mut Self;
    pub fn with_error_rate(&mut self, rate: f32) -> &mut Self;
    pub fn verify(&self) -> Result<()>;
}

impl MockWhatsApp {
    pub async fn start(port: u16) -> Result<Self>;
    pub fn expect_send_message(&mut self, to: &str) -> MessageExpectation;
    pub fn expect_send_template(&mut self, name: &str) -> TemplateExpectation;
    pub fn simulate_incoming(&self, from: &str, text: &str) -> Result<()>;
    pub fn simulate_webhook(&self, event: WebhookEvent) -> Result<()>;
}

impl MockZitadel {
    pub async fn start(port: u16) -> Result<Self>;
    pub fn expect_login(&mut self, user: &str, password: &str) -> TokenResponse;
    pub fn expect_token_refresh(&mut self) -> &mut Self;
    pub fn expect_introspect(&mut self, token: &str, active: bool) -> &mut Self;
    pub fn create_test_user(&mut self, email: &str) -> User;
}
```

---

## Test Categories

### Unit Tests

Fast, isolated, no external services:

```rust
#[test]
fn test_basic_parser() {
    let ast = parse("TALK \"Hello\"").unwrap();
    assert_eq!(ast.statements.len(), 1);
}

#[test]
fn test_config_csv_parsing() {
    let config = ConfigManager::from_str("name,value\nllm-model,test.gguf");
    assert_eq!(config.get("llm-model"), Some("test.gguf"));
}
```

### Integration Tests

Real services, isolated environment:

```rust
#[tokio::test]
async fn test_database_operations() {
    let harness = TestHarness::new(TestConfig::default()).await.unwrap();
    
    harness.postgres.execute("INSERT INTO users (email) VALUES ('test@example.com')").await.unwrap();
    
    let result = harness.postgres.query_one("SELECT email FROM users").await.unwrap();
    assert_eq!(result.get::<_, String>("email"), "test@example.com");
}

#[tokio::test]
async fn test_file_storage() {
    let harness = TestHarness::new(TestConfig::default()).await.unwrap();
    
    harness.minio.create_bucket("test-bucket").await.unwrap();
    harness.minio.put_object("test-bucket", "file.txt", b"content").await.unwrap();
    
    let data = harness.minio.get_object("test-bucket", "file.txt").await.unwrap();
    assert_eq!(data, b"content");
}
```

### Bot Conversation Tests

Simulate full conversation flows:

```rust
#[tokio::test]
async fn test_greeting_flow() {
    let harness = TestHarness::new(TestConfig::with_llm_mock()).await.unwrap();
    
    harness.mocks.llm.expect_completion("greeting", "Hello! How can I help?");
    
    let mut conv = ConversationTest::new(&harness, "test-bot").await.unwrap();
    
    conv.user_says("Hi").await;
    conv.assert_response_contains("Hello").await;
    conv.assert_response_contains("help").await;
}

#[tokio::test]
async fn test_knowledge_base_search() {
    let harness = TestHarness::new(TestConfig::with_kb()).await.unwrap();
    
    harness.seed_kb("products", vec![
        ("SKU-001", "Widget Pro - Premium quality widget"),
        ("SKU-002", "Widget Basic - Entry level widget"),
    ]).await.unwrap();
    
    let mut conv = ConversationTest::new(&harness, "kb-bot").await.unwrap();
    
    conv.user_says("Tell me about Widget Pro").await;
    conv.assert_response_contains("Premium quality").await;
}

#[tokio::test]
async fn test_human_handoff() {
    let harness = TestHarness::new(TestConfig::default()).await.unwrap();
    
    let mut conv = ConversationTest::new(&harness, "support-bot").await.unwrap();
    
    conv.user_says("I want to speak to a human").await;
    conv.assert_transferred_to_human().await;
    conv.assert_queue_position(1).await;
}
```

### Attendance Module Tests

Multi-user concurrent scenarios:

```rust
#[tokio::test]
async fn test_queue_ordering() {
    let harness = TestHarness::new(TestConfig::default()).await.unwrap();
    
    let customer1 = harness.create_customer("customer1@test.com").await;
    let customer2 = harness.create_customer("customer2@test.com").await;
    let attendant = harness.create_attendant("agent@test.com").await;
    
    harness.enter_queue(&customer1, Priority::Normal).await;
    harness.enter_queue(&customer2, Priority::High).await;
    
    let next = harness.get_next_in_queue(&attendant).await.unwrap();
    assert_eq!(next.customer_id, customer2.id); // High priority first
}

#[tokio::test]
async fn test_concurrent_assignment() {
    let harness = TestHarness::new(TestConfig::default()).await.unwrap();
    
    let customers: Vec<_> = (0..10).map(|i| 
        harness.create_customer(&format!("c{}@test.com", i))
    ).collect();
    
    let attendants: Vec<_> = (0..3).map(|i|
        harness.create_attendant(&format!("a{}@test.com", i))
    ).collect();
    
    // Concurrent assignment - no race conditions
    let assignments = join_all(customers.iter().map(|c| 
        harness.auto_assign(c)
    )).await;
    
    // Verify no double-assignments
    let assigned_attendants: HashSet<_> = assignments.iter()
        .filter_map(|a| a.as_ref().ok())
        .map(|a| a.attendant_id)
        .collect();
    
    assert!(assignments.iter().all(|a| a.is_ok()));
}
```

### E2E Browser Tests

Full stack with real browser:

```rust
#[tokio::test]
async fn test_chat_interface() {
    let harness = TestHarness::new(TestConfig::full_stack()).await.unwrap();
    let server = harness.with_botserver().await.unwrap();
    
    let browser = Browser::new_headless().await.unwrap();
    let page = browser.new_page().await.unwrap();
    
    page.goto(&format!("{}/chat/test-bot", server.url())).await.unwrap();
    page.wait_for("#chat-input").await.unwrap();
    
    page.fill("#chat-input", "Hello").await.unwrap();
    page.click("#send-button").await.unwrap();
    
    page.wait_for(".bot-message").await.unwrap();
    let response = page.text(".bot-message").await.unwrap();
    
    assert!(response.contains("Hello"));
}

#[tokio::test]
async fn test_attendant_dashboard() {
    let harness = TestHarness::new(TestConfig::full_stack()).await.unwrap();
    let server = harness.with_botserver().await.unwrap();
    
    let browser = Browser::new_headless().await.unwrap();
    let page = browser.new_page().await.unwrap();
    
    // Login as attendant
    page.goto(&format!("{}/login", server.url())).await.unwrap();
    page.fill("#email", "attendant@test.com").await.unwrap();
    page.fill("#password", "testpass").await.unwrap();
    page.click("#login-button").await.unwrap();
    
    page.wait_for(".queue-panel").await.unwrap();
    
    // Verify queue display
    let queue_count = page.text(".queue-count").await.unwrap();
    assert_eq!(queue_count, "0");
}
```

---

## Fixtures

### Data Factories

```rust
pub mod fixtures {
    pub fn admin_user() -> User {
        User {
            id: Uuid::new_v4(),
            email: "admin@test.com".into(),
            role: Role::Admin,
            ..Default::default()
        }
    }
    
    pub fn customer(phone: &str) -> Customer {
        Customer {
            id: Uuid::new_v4(),
            phone: phone.into(),
            channel: Channel::WhatsApp,
            ..Default::default()
        }
    }
    
    pub fn bot_with_kb(name: &str) -> Bot {
        Bot {
            id: Uuid::new_v4(),
            name: name.into(),
            kb_enabled: true,
            ..Default::default()
        }
    }
}
```

### BASIC Script Fixtures

```
fixtures/scripts/
├── greeting.bas          # Simple greeting flow
├── kb_search.bas         # Knowledge base integration
├── attendance.bas        # Human handoff flow
├── error_handling.bas    # ON ERROR RESUME NEXT patterns
├── llm_tools.bas         # LLM with tool calls
├── data_operations.bas   # FIND, SAVE, UPDATE, DELETE
└── http_integration.bas  # POST, GET, GRAPHQL, SOAP
```

---

## CI/CD Integration

### GitHub Actions

```yaml
name: Tests
on: [push, pull_request]

jobs:
  unit:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test --lib --workspace

  integration:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo test -p bottest --test integration -- --test-threads=4

  e2e:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: npx playwright install chromium
      - run: cargo test -p bottest --test e2e

  coverage:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v4
      - run: cargo llvm-cov --workspace --lcov --output-path lcov.info
      - uses: codecov/codecov-action@v3
```

---

## Usage

```bash
# Run all tests
cargo test -p bottest

# Unit tests only (fast, no services)
cargo test -p bottest --lib

# Integration tests (starts real services)
cargo test -p bottest --test integration

# E2E tests (starts browser)
cargo test -p bottest --test e2e

# Specific test
cargo test -p bottest test_queue_ordering

# With visible browser for debugging
HEADED=1 cargo test -p bottest --test e2e

# Parallel execution (default)
cargo test -p bottest -- --test-threads=8

# Keep test environment for inspection
KEEP_ENV=1 cargo test -p bottest test_name
```

---

## Implementation Status

| Component | Status | Notes |
|-----------|--------|-------|
| Test Harness | ✅ Complete | Ephemeral environments working |
| Port Allocation | ✅ Complete | Dynamic 49152-65535 range |
| PostgreSQL Service | ✅ Complete | Via botserver bootstrap |
| MinIO Service | ✅ Complete | Via botserver bootstrap |
| Redis Service | ✅ Complete | Via botserver bootstrap |
| Cleanup | ✅ Complete | Drop trait + signal handlers |
| Mock LLM | ✅ Complete | OpenAI-compatible |
| Mock WhatsApp | ✅ Complete | Business API |
| Mock Zitadel | ✅ Complete | OIDC/Auth |
| Conversation Tests | ✅ Complete | Full flow simulation |
| BASIC Runner | ✅ Complete | Direct script execution |
| Fixtures | ✅ Complete | Users, bots, sessions |
| Browser Automation | ✅ Complete | fantoccini/WebDriver |
| Attendance Tests | ✅ Complete | Multi-user scenarios |
| CI Integration | ✅ Complete | GitHub Actions |
| Coverage Reports | ✅ Complete | cargo-llvm-cov |

---

## Performance

| Test Type | Count | Duration | Parallel |
|-----------|-------|----------|----------|
| Unit | 450+ | ~5s | Yes |
| Integration | 120+ | ~45s | Yes |
| E2E | 35+ | ~90s | Limited |
| **Total** | **605+** | **< 3 min** | - |

---

## Coverage Targets

| Module | Current | Target |
|--------|---------|--------|
| botserver/src/basic | 82% | 85% |
| botserver/src/attendance | 91% | 95% |
| botserver/src/llm | 78% | 80% |
| botserver/src/core | 75% | 80% |
| **Overall** | **79%** | **80%** |