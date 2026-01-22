# BotTest Development Guide

**Version:** 6.2.0  
**Purpose:** Test infrastructure for General Bots ecosystem

---

## ZERO TOLERANCE POLICY

**EVERY SINGLE WARNING MUST BE FIXED. NO EXCEPTIONS.**

---

## ❌ ABSOLUTE PROHIBITIONS

```
❌ NEVER use #![allow()] or #[allow()] in source code
❌ NEVER use _ prefix for unused variables - DELETE or USE them
❌ NEVER use .unwrap() - use ? or proper error handling
❌ NEVER use .expect() - use ? or proper error handling  
❌ NEVER use panic!() or unreachable!()
❌ NEVER use todo!() or unimplemented!()
❌ NEVER leave unused imports or dead code
❌ NEVER add comments - code must be self-documenting
```

---

## 🏗️ ARCHITECTURE

E2E tests use `USE_BOTSERVER_BOOTSTRAP=1` mode. The botserver handles all service installation during bootstrap.

```
TestHarness::full() / E2E Tests
    │
    ├── Allocate unique ports (15000+)
    ├── Create ./tmp/bottest-{uuid}/
    │
    ├── Start mock servers only
    │   ├── MockZitadel (wiremock)
    │   └── MockLLM (wiremock)
    │
    ├── Start botserver with --stack-path
    │   └── Botserver auto-installs:
    │       ├── PostgreSQL (tables)
    │       ├── MinIO (drive)
    │       └── Redis (cache)
    │
    └── Return TestContext
```

---

## 🧪 TEST CATEGORIES

### Unit Tests (no services)
```rust
#[test]
fn test_pure_logic() {
    // No TestHarness needed
}
```

### Integration Tests (with services)
```rust
#[tokio::test]
async fn test_with_database() {
    let ctx = TestHarness::quick().await?;
    let pool = ctx.db_pool().await?;
    // Use real database
}
```

### E2E Tests (with browser)
```rust
#[tokio::test]
async fn test_user_flow() {
    let ctx = TestHarness::full().await?;
    let server = ctx.start_botserver().await?;
    let browser = Browser::new().await?;
    // Automate browser
}
```

---

## 🎭 MOCK SERVER PATTERNS

### Expect specific calls
```rust
ctx.mock_llm().expect_completion("hello", "Hi there!");
```

### Verify calls were made
```rust
ctx.mock_llm().assert_called_times(2);
```

### Simulate errors
```rust
ctx.mock_llm().next_call_fails(500, "Internal error");
```

---

## 🏭 FIXTURE PATTERNS

### Factory functions
```rust
let user = fixtures::admin_user();
let bot = fixtures::bot_with_kb();
let session = fixtures::active_session(&user, &bot);
```

### Insert into database
```rust
ctx.insert(&user).await;
ctx.insert(&bot).await;
```

---

## ⚡ PARALLEL SAFETY

- Each test gets unique ports via PortAllocator
- Each test gets unique temp directory
- No shared state between tests
- Safe to run with `cargo test -j 8`

---

## 📖 DOCUMENTATION LOCATION

All documentation goes in `botbook/src/17-testing/`:
- `README.md` - Testing overview
- `e2e-testing.md` - E2E test guide
- `architecture.md` - Testing architecture
- `best-practices.md` - Best practices

---

## 🔑 REMEMBER

- **ZERO WARNINGS** - Every clippy warning must be fixed
- **NO ALLOW ATTRIBUTES** - Never silence warnings
- **NO DEAD CODE** - Delete unused code
- **NO UNWRAP/EXPECT** - Use ? operator
- **INLINE FORMAT ARGS** - `format!("{name}")` not `format!("{}", name)`
- **USE SELF** - In impl blocks, use Self not the type name
- **Reuse bootstrap** - Don't duplicate botserver installation logic
- **Version 6.2.0** - do not change without approval
