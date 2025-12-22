# BotTest Development Prompt

**Version:** 6.1.0  
**Purpose:** Test infrastructure for General Bots ecosystem

---

## ZERO TOLERANCE POLICY

**This project has the strictest code quality requirements possible.**

**EVERY SINGLE WARNING MUST BE FIXED. NO EXCEPTIONS.**

---

## ABSOLUTE PROHIBITIONS

```
❌ NEVER use #![allow()] or #[allow()] to silence warnings
❌ NEVER use _ prefix for unused variables - DELETE the variable or USE it
❌ NEVER use .unwrap() - use ? or proper error handling
❌ NEVER use .expect() - use ? or proper error handling  
❌ NEVER use panic!() or unreachable!() - handle all cases
❌ NEVER use todo!() or unimplemented!() - write real code
❌ NEVER leave unused imports - DELETE them
❌ NEVER leave dead code - DELETE it or IMPLEMENT it
❌ NEVER use approximate constants (3.14159) - use std::f64::consts::PI
❌ NEVER silence clippy - FIX THE CODE
❌ NEVER add comments explaining what code does - code must be self-documenting
```

---

## MANDATORY CODE PATTERNS

### Error Handling - Use `?` Operator

```rust
// ❌ WRONG
let value = something.unwrap();
let value = something.expect("msg");

// ✅ CORRECT
let value = something?;
let value = something.ok_or_else(|| Error::NotFound)?;
```

### Self Usage in Impl Blocks

```rust
// ❌ WRONG
impl MyStruct {
    fn new() -> MyStruct { MyStruct { } }
}

// ✅ CORRECT
impl MyStruct {
    fn new() -> Self { Self { } }
}
```

### Format Strings - Inline Variables

```rust
// ❌ WRONG
format!("Hello {}", name)

// ✅ CORRECT
format!("Hello {name}")
```

### Derive Eq with PartialEq

```rust
// ❌ WRONG
#[derive(PartialEq)]
struct MyStruct { }

// ✅ CORRECT
#[derive(PartialEq, Eq)]
struct MyStruct { }
```

---

## Weekly Maintenance - EVERY MONDAY

### Package Review Checklist

**Every Monday, review the following:**

1. **Dependency Updates**
   ```bash
   cargo outdated
   cargo audit
   ```

2. **Package Consolidation Opportunities**
   - Check if new crates can replace custom code
   - Look for crates that combine multiple dependencies
   - Review `Cargo.toml` for redundant dependencies

3. **Code Reduction Candidates**
   - Custom mock implementations that can use crates
   - Test utilities that have crate equivalents
   - Boilerplate that can be replaced with macros

4. **Test Infrastructure Updates**
   - Check for new testing patterns
   - Review mock server libraries
   - Update fixture generation approaches

### Packages to Watch

| Area | Potential Packages | Purpose |
|------|-------------------|---------|
| Mocking | `wiremock`, `mockall` | Simplify mock creation |
| Assertions | `assertables`, `pretty_assertions` | Better test output |
| Fixtures | `fake`, `proptest` | Generate test data |
| Async Testing | `tokio-test` | Async test utilities |

---

## CRITICAL RULE

🚫 **NO .md FILES IN ROOT OF ANY PROJECT**

All documentation goes in `botbook/src/17-testing/`:
- `README.md` - Testing overview
- `e2e-testing.md` - E2E test guide
- `architecture.md` - Testing architecture
- `performance.md` - Performance testing
- `best-practices.md` - Best practices

This PROMPT.md is the ONLY exception (it's for developers).

---

## Core Principle

**Reuse botserver bootstrap code** - Don't duplicate installation logic. The bootstrap module already knows how to install PostgreSQL, MinIO, Redis. We wrap it with test-specific configuration (custom ports, temp directories).

---

## Architecture

**IMPORTANT:** E2E tests always use `USE_BOTSERVER_BOOTSTRAP=1` mode. No global PostgreSQL or other services are required. The botserver handles all service installation during bootstrap.

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

TestContext provides:
    - db_pool() -> Database connection
    - minio_client() -> S3 client
    - redis_client() -> Redis client
    - mock_*() -> Mock server controls

On Drop:
    - Stop all services
    - Remove temp directory
```

---

## Code Style

Same as botserver PROMPT.md:
- KISS, NO TALK, SECURED CODE ONLY
- No comments, no placeholders
- Complete, production-ready code
- Return 0 warnings

---

## Test Categories

### Unit Tests (no services)
```rust
#[test]
fn test_pure_logic() {
    // No TestHarness needed
    // Test pure functions directly
}
```

### Integration Tests (with services)
```rust
#[tokio::test]
async fn test_with_database() {
    let ctx = TestHarness::quick().await.unwrap();
    let pool = ctx.db_pool().await.unwrap();
    // Use real database
}
```

### E2E Tests (with browser)
```rust
#[tokio::test]
async fn test_user_flow() {
    let ctx = TestHarness::full().await.unwrap();
    let server = ctx.start_botserver().await.unwrap();
    let browser = Browser::new().await.unwrap();
    // Automate browser
}
```

---

## Mock Server Patterns

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

## Fixture Patterns

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

## Cleanup

Always automatic via Drop trait. But can force:
```rust
ctx.cleanup().await;  // Explicit cleanup
```

---

## Parallel Safety

- Each test gets unique ports via PortAllocator
- Each test gets unique temp directory
- No shared state between tests
- Safe to run with `cargo test -j 8`

---

## Documentation Location

For guides, tutorials, and reference:
→ Use `botbook/src/17-testing/`

Examples:
- E2E testing setup → `botbook/src/17-testing/e2e-testing.md`
- Architecture details → `botbook/src/17-testing/architecture.md`
- Performance tips → `botbook/src/17-testing/performance.md`

Never create .md files at:
- ✗ Root of bottest/
- ✗ Root of botserver/
- ✗ Root of botapp/
- ✗ Any project root

All non-PROMPT.md documentation belongs in botbook.

---

## Remember

- **ZERO WARNINGS** - Every clippy warning must be fixed
- **NO ALLOW ATTRIBUTES** - Never silence warnings, fix the code
- **NO DEAD CODE** - Delete unused code, never prefix with _
- **NO UNWRAP/EXPECT** - Use ? operator or proper error handling
- **INLINE FORMAT ARGS** - format!("{name}") not format!("{}", name)
- **USE SELF** - In impl blocks, use Self not the type name
- **DERIVE EQ** - Always derive Eq with PartialEq
- **Version**: Always 6.1.0 - do not change without approval
- **Session Continuation**: When running out of context, create detailed summary: (1) what was done, (2) what remains, (3) specific files and line numbers, (4) exact next steps.
