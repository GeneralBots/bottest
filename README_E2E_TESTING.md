# E2E Testing for General Bots Platform

## Quick Start

Run the complete platform flow test (loads UI → starts BotServer → login → chat → logout):

```bash
cd gb/bottest

# Without browser (HTTP-only tests)
cargo test --test e2e test_platform_loading_http_only -- --nocapture
cargo test --test e2e test_botserver_startup -- --nocapture

# With browser (requires WebDriver)
# 1. Start WebDriver first:
chromedriver --port=4444

# 2. In another terminal:
cargo test --test e2e test_complete_platform_flow_login_chat_logout -- --nocapture
```

## What Gets Tested

The complete platform flow test validates:

1. **Platform Loading** ✓
   - UI assets are served
   - API endpoints respond
   - Database migrations completed

2. **BotServer Initialization** ✓
   - Service is running
   - Health checks pass
   - Configuration loaded

3. **User Authentication** ✓
   - Login page loads
   - Credentials accepted
   - Session created
   - Redirected to dashboard/chat

4. **Chat Interaction** ✓
   - Chat interface loads
   - Messages can be sent
   - Bot responses received
   - Message history persists

5. **Logout Flow** ✓
   - Logout button works
   - Session invalidated
   - Redirect to login page
   - Protected routes blocked

## Test Files

| File | Purpose |
|------|---------|
| `tests/e2e/platform_flow.rs` | ⭐ Complete user journey test |
| `tests/e2e/auth_flow.rs` | Authentication scenarios |
| `tests/e2e/chat.rs` | Chat message flows |
| `tests/e2e/dashboard.rs` | Dashboard functionality |
| `tests/e2e/mod.rs` | Test context and setup |

## Running Specific Tests

```bash
# Platform flow (complete journey)
cargo test --test e2e test_complete_platform_flow_login_chat_logout -- --nocapture

# Platform loading only (no browser needed)
cargo test --test e2e test_platform_loading_http_only -- --nocapture

# BotServer startup
cargo test --test e2e test_botserver_startup -- --nocapture

# Simpler login + chat
cargo test --test e2e test_login_and_chat_flow -- --nocapture

# Platform responsiveness
cargo test --test e2e test_platform_responsiveness -- --nocapture

# All E2E tests
cargo test --test e2e -- --nocapture
```

## Environment Variables

```bash
# Show browser window (for debugging)
HEADED=1 cargo test --test e2e -- --nocapture

# Custom WebDriver URL
WEBDRIVER_URL=http://localhost:4445 cargo test --test e2e -- --nocapture

# Skip E2E tests
SKIP_E2E_TESTS=1 cargo test

# Verbose logging
RUST_LOG=debug cargo test --test e2e -- --nocapture

# Run single-threaded (clearer output)
cargo test --test e2e -- --nocapture --test-threads=1
```

## Prerequisites

### For HTTP-only Tests
- Rust toolchain
- BotServer compiled

### For Browser Tests (Full E2E)
- Chrome/Chromium installed
- WebDriver (chromedriver) running on port 4444
- All HTTP test prerequisites

### Setup WebDriver

**Option 1: Local Installation**
```bash
# Download chromedriver from https://chromedriver.chromium.org/
# Place in PATH, then:
chromedriver --port=4444
```

**Option 2: Docker**
```bash
docker run -d -p 4444:4444 selenium/standalone-chrome
```

**Option 3: Docker Compose**
```bash
# Use provided docker-compose.yml if available
docker-compose up -d webdriver
```

## Architecture: Temporary Stack (Future)

The E2E tests are designed to work with **temporary, isolated stacks**:

```bash
# When implemented, this will spawn a temporary environment:
botserver --temp-stack

# This creates: /tmp/botserver-test-{timestamp}-{random}/
# With: PostgreSQL, Redis, MinIO, Mock LLM, Mock Auth
# Automatic cleanup after tests
```

**Benefits:**
- ✓ Isolation - Each test runs in separate environment
- ✓ Reproducibility - Same setup every time
- ✓ Automation - No manual setup required
- ✓ Cleanup - Automatic resource management
- ✓ Debugging - Optionally preserve stack on failure

See [TEMP_STACK_SETUP.md](TEMP_STACK_SETUP.md) for implementation details.

## Common Issues

### WebDriver Not Available
```bash
# Solution: Start WebDriver first
chromedriver --port=4444
# or
docker run -d -p 4444:4444 selenium/standalone-chrome
```

### Tests Hang or Timeout
```bash
# Run with timeout and single thread
timeout 120s cargo test --test e2e test_name -- --nocapture --test-threads=1

# With verbose logging
RUST_LOG=debug timeout 120s cargo test --test e2e test_name -- --nocapture --test-threads=1
```

### Port Already in Use
```bash
# Kill existing processes
pkill -f chromedriver
pkill -f "botserver"
pkill -f "postgres"
pkill -f "redis-server"
```

### Browser Connection Issues
```bash
# Use different WebDriver port
WEBDRIVER_URL=http://localhost:4445 cargo test --test e2e -- --nocapture
```

## Test Structure

Each test follows this pattern:

```rust
#[tokio::test]
async fn test_example() {
    // 1. Setup context
    let ctx = E2ETestContext::setup_with_browser().await?;
    
    // 2. Get browser
    let browser = ctx.browser.as_ref().unwrap();
    
    // 3. Run test steps
    browser.navigate(&ctx.base_url()).await?;
    
    // 4. Verify results
    assert!(some_condition);
    
    // 5. Cleanup (automatic)
    ctx.close().await;
}
```

## Debugging

### View Test Output
```bash
# Show all output
cargo test --test e2e test_name -- --nocapture

# Show with timestamps
RUST_LOG=debug cargo test --test e2e test_name -- --nocapture

# Save to file
cargo test --test e2e test_name -- --nocapture 2>&1 | tee test_output.log
```

### See Browser in Action
```bash
# Run with visible browser
HEADED=1 cargo test --test e2e test_name -- --nocapture --test-threads=1

# This shows what the test is doing in real-time
```

### Check Server Logs
```bash
# BotServer logs while tests run
tail -f /tmp/bottest-*/botserver.log

# In another terminal:
cargo test --test e2e test_name -- --nocapture
```

## Performance

- **Platform loading test**: ~2-3 seconds
- **BotServer startup test**: ~5-10 seconds  
- **Complete flow with browser**: ~30-45 seconds
- **Full E2E test suite**: ~2-3 minutes

## Integration with CI/CD

Example GitHub Actions workflow:

```yaml
name: E2E Tests
on: [push, pull_request]

jobs:
  e2e:
    runs-on: ubuntu-latest
    services:
      chromedriver:
        image: selenium/standalone-chrome
    steps:
      - uses: actions/checkout@v3
      - uses: actions-rs/toolchain@v1
      - run: cd gb/bottest && cargo test --test e2e -- --nocapture
```

## What's Tested in Each Scenario

### `test_complete_platform_flow_login_chat_logout`
✓ Platform health check
✓ API responsiveness  
✓ Login with credentials
✓ Dashboard/chat visibility
✓ Send message to bot
✓ Receive bot response
✓ Message appears in history
✓ Logout button works
✓ Session invalidated
✓ Protected routes blocked

### `test_platform_loading_http_only`
✓ Platform health endpoint
✓ API endpoints available
✓ No browser required

### `test_botserver_startup`
✓ Server process running
✓ Health checks pass
✓ No browser required

### `test_login_and_chat_flow`
✓ Minimal path through login and chat
✓ Requires browser

## Next Steps

1. **Run a simple test first**:
   ```bash
   cargo test --test e2e test_platform_loading_http_only -- --nocapture
   ```

2. **Setup WebDriver for browser tests**:
   ```bash
   chromedriver --port=4444
   ```

3. **Run the complete flow**:
   ```bash
   cargo test --test e2e test_complete_platform_flow_login_chat_logout -- --nocapture
   ```

4. **Add custom tests** in `tests/e2e/` using the same pattern

5. **Integrate into CI/CD** using the GitHub Actions example above

## Documentation

- [E2E Testing Plan](E2E_TESTING_PLAN.md) - Architecture and design
- [Temporary Stack Setup](TEMP_STACK_SETUP.md) - Advanced: Using isolated test stacks
- [Test Harness](src/harness.rs) - Test utilities and helpers
- [Platform Flow Tests](tests/e2e/platform_flow.rs) - Complete implementation

## Support

For issues or questions:
1. Check the troubleshooting section above
2. Review test output with `--nocapture` flag
3. Run with `RUST_LOG=debug` for detailed logging
4. Check server logs in `/tmp/bottest-*/`
5. Use `HEADED=1` to watch browser in action

## Key Metrics

Running `test_complete_platform_flow_login_chat_logout` provides:

- **Response Times**: Platform, API, and chat latencies
- **Resource Usage**: Memory and CPU during test
- **Error Rates**: Login failures, message timeouts, etc.
- **Session Management**: Login/logout cycle validation
- **Message Flow**: End-to-end chat message delivery

These metrics help identify performance bottlenecks and regressions.