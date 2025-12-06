//! Unit Tests for BotServer
//!
//! These tests verify BASIC language functions (string, math, etc.)
//! and core logic like attendance queue handling.
//! No external services required (PostgreSQL, Redis, MinIO).

mod attendance;
mod math_functions;
mod string_functions;

/// Verify the test module loads correctly
#[test]
fn test_unit_module_loads() {
    // If this compiles and runs, the test infrastructure is working
    assert!(true);
}
