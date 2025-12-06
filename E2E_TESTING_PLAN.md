# E2E Testing Plan: Temporary Stack Architecture

## Overview

This document outlines the architecture for comprehensive E2E testing in the General Bots platform using a temporary, isolated stack that can be spawned for testing and automatically cleaned up.

## Problem Statement

Current challenges:
- E2E tests require a pre-configured environment
- Testing can interfere with the main development stack
- No easy way to test the complete flow: platform loading → botserver startup → login → chat → logout
- Integration tests are difficult to automate and reproduce

## Proposed Solution

### 1. Temporary Stack Option in BotServer

Add a new CLI flag `--temp-stack` to BotServer that:

```bash
cargo run -- --temp-stack
# or with custom timeout
cargo run -- --temp-stack --temp-stack-timeout 300
```

**What it does:**
- Creates a temporary directory: `/tmp/botserver-test-{timestamp}-{random}/`
- Sets up all required services (PostgreSQL, MinIO, Redis, etc.) in this directory
- Configures BotServer to use this isolated environment
- Provides environment variables for test harness to connect
- Automatically cleans up on shutdown (SIGTERM/SIGINT)
- Optional timeout that auto-shuts down after N seconds (useful for CI/CD)

### 2. E2E Test Flow

The complete user journey test will validate:

```
1. Platform Loading
   └─ Health check endpoint responds
   └─ UI assets served correctly
   └─ Database migrations completed

2. BotServer Initialization
   └─ Service discovery working
   └─ Configuration loaded
   └─ Dependencies connected

3. Authentication (Login)
   └─ Navigate to login page
   └─ Enter valid credentials
   └─ Session created
   └─ Redirected to dashboard

4. Chat Interaction
   └─ Open chat window
   └─ Send message
   └─ Receive AI response
   └─ Message history persisted

5. Logout
   └─ Click logout button
   └─ Session invalidated
   └─ Redirected to login
   └─ Cannot access protected routes
```

### 3. Test Architecture

#### Test Harness Enhancement

```rust
pub struct TemporaryStack {
    pub temp_dir: PathBuf,
    pub botserver_process: Child,
    pub botserver_url: String,
    pub services: ServiceManager,
}

impl TemporaryStack {
    pub async fn spawn() -> anyhow::Result<Self>;
    pub async fn wait_ready(&self) -> anyhow::Result<()>;
    pub async fn shutdown(mut self) -> anyhow::Result<()>;
}
```

#### E2E Test Structure

```rust
#[tokio::test]
async fn test_complete_user_journey() {
    // 1. Spawn temporary isolated stack
    let stack = TemporaryStack::spawn().await.expect("Failed to spawn stack");
    stack.wait_ready().await.expect("Stack failed to become ready");

    // 2. Setup browser
    let browser = Browser::new(browser_config()).await.expect("Browser failed");

    // 3. Test complete flow
    test_platform_loading(&browser, &stack).await.expect("Platform load failed");
    test_botserver_running(&stack).await.expect("BotServer not running");
    test_login_flow(&browser, &stack).await.expect("Login failed");
    test_chat_interaction(&browser, &stack).await.expect("Chat failed");
    test_logout_flow(&browser, &stack).await.expect("Logout failed");

    // 4. Cleanup (automatic on drop)
    drop(stack);
}
```

### 4. Implementation Phases

#### Phase 1: BotServer Temp Stack Support
- [ ] Add `--temp-stack` CLI argument
- [ ] Create `TempStackConfig` struct
- [ ] Implement temporary directory setup
- [ ] Update service initialization to support temp paths
- [ ] Add cleanup on shutdown

#### Phase 2: Test Harness Integration
- [ ] Create `TemporaryStack` struct in test framework
- [ ] Implement stack spawning logic
- [ ] Add readiness checks
- [ ] Implement graceful shutdown

#### Phase 3: Complete E2E Test Suite
- [ ] Platform loading test
- [ ] BotServer initialization test
- [ ] Complete login → chat → logout flow
- [ ] Error handling and edge cases

#### Phase 4: CI/CD Integration
- [ ] Docker compose for CI environment
- [ ] GitHub Actions workflow
- [ ] Artifact collection on failure
- [ ] Performance benchmarks

## Technical Details

### Environment Variables

When `--temp-stack` is enabled, BotServer outputs:

```bash
export BOTSERVER_TEMP_STACK_DIR="/tmp/botserver-test-2024-01-15-abc123/"
export BOTSERVER_URL="http://localhost:8000"
export DB_HOST="127.0.0.1"
export DB_PORT="5432"
export DB_NAME="botserver_test_abc123"
export REDIS_URL="redis://127.0.0.1:6379"
export MINIO_URL="http://127.0.0.1:9000"
```

### Cleanup Strategy

- **Graceful**: On SIGTERM/SIGINT, wait for in-flight requests then cleanup
- **Timeout**: Auto-shutdown after `--temp-stack-timeout` seconds
- **Forceful**: If timeout reached, force kill processes and cleanup
- **Persistent on Error**: Keep temp dir if error occurs (for debugging)

### Service Isolation

Each temporary stack includes:

```
/tmp/botserver-test-{id}/
├── postgres/
│   └── data/
├── redis/
│   └── data/
├── minio/
│   └── data/
├── botserver/
│   ├── logs/
│   ├── config/
│   └── cache/
└── state.json
```

## Benefits

1. **Isolation**: Each test gets a completely clean environment
2. **Reproducibility**: Same setup every time
3. **Automation**: Can run in CI/CD without manual setup
4. **Debugging**: Failed tests leave artifacts for investigation
5. **Performance**: Multiple tests can run in parallel with different ports
6. **Safety**: No risk of interfering with development environment

## Limitations

- **LXC Containers**: Cannot test containerization (as mentioned)
- **Network**: Tests run on localhost only
- **Performance**: Startup time ~10-30 seconds per test
- **Parallelization**: Need port management for parallel execution

## Usage Examples

### Run Single E2E Test
```bash
cargo test --test e2e_complete_flow -- --nocapture
```

### Run with Headed Browser (for debugging)
```bash
HEADED=1 cargo test --test e2e_complete_flow
```

### Keep Temp Stack on Failure
```bash
KEEP_TEMP_STACK_ON_ERROR=1 cargo test --test e2e_complete_flow
```

### Run All E2E Tests
```bash
cargo test --lib e2e:: -- --nocapture
```

## Monitoring & Logging

- BotServer logs: `/tmp/botserver-test-{id}/botserver.log`
- Database logs: `/tmp/botserver-test-{id}/postgres.log`
- Test output: stdout/stderr from test harness
- Performance metrics: Collected during each phase

## Success Criteria

✓ Platform fully loads without errors
✓ BotServer starts and services become ready within 30 seconds
✓ User can login with test credentials
✓ Chat messages are sent and responses received
✓ User can logout and session is invalidated
✓ All cleanup happens automatically
✓ Test runs consistently multiple times
✓ CI/CD integration works smoothly

## Next Steps

1. Implement `--temp-stack` flag in BotServer
2. Update config loading to support temp paths
3. Create `TemporaryStack` test utility
4. Write comprehensive E2E test suite
5. Integrate into CI/CD pipeline
6. Document for team