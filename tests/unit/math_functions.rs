//! Unit tests for BASIC math functions from botserver
//!
//! These tests create a Rhai engine, register math functions the same way
//! botserver does, and verify they work correctly.

use rhai::Engine;

// =============================================================================
// ABS Function Tests
// =============================================================================

#[test]
fn test_abs_positive() {
    let mut engine = Engine::new();
    engine.register_fn("ABS", |n: i64| -> i64 { n.abs() });
    engine.register_fn("ABS", |n: f64| -> f64 { n.abs() });

    let result: i64 = engine.eval("ABS(42)").unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_abs_negative() {
    let mut engine = Engine::new();
    engine.register_fn("ABS", |n: i64| -> i64 { n.abs() });

    let result: i64 = engine.eval("ABS(-42)").unwrap();
    assert_eq!(result, 42);
}

#[test]
fn test_abs_zero() {
    let mut engine = Engine::new();
    engine.register_fn("ABS", |n: i64| -> i64 { n.abs() });

    let result: i64 = engine.eval("ABS(0)").unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_abs_float() {
    let mut engine = Engine::new();
    engine.register_fn("ABS", |n: f64| -> f64 { n.abs() });

    let result: f64 = engine.eval("ABS(-3.14)").unwrap();
    assert!((result - 3.14).abs() < f64::EPSILON);
}

// =============================================================================
// ROUND Function Tests
// =============================================================================

#[test]
fn test_round_up() {
    let mut engine = Engine::new();
    engine.register_fn("ROUND", |n: f64| -> i64 { n.round() as i64 });

    let result: i64 = engine.eval("ROUND(3.7)").unwrap();
    assert_eq!(result, 4);
}

#[test]
fn test_round_down() {
    let mut engine = Engine::new();
    engine.register_fn("ROUND", |n: f64| -> i64 { n.round() as i64 });

    let result: i64 = engine.eval("ROUND(3.2)").unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_round_half() {
    let mut engine = Engine::new();
    engine.register_fn("ROUND", |n: f64| -> i64 { n.round() as i64 });

    let result: i64 = engine.eval("ROUND(3.5)").unwrap();
    assert_eq!(result, 4);
}

#[test]
fn test_round_negative() {
    let mut engine = Engine::new();
    engine.register_fn("ROUND", |n: f64| -> i64 { n.round() as i64 });

    let result: i64 = engine.eval("ROUND(-3.7)").unwrap();
    assert_eq!(result, -4);
}

// =============================================================================
// INT / FIX Function Tests (Truncation)
// =============================================================================

#[test]
fn test_int_positive() {
    let mut engine = Engine::new();
    engine.register_fn("INT", |n: f64| -> i64 { n.trunc() as i64 });

    let result: i64 = engine.eval("INT(3.9)").unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_int_negative() {
    let mut engine = Engine::new();
    engine.register_fn("INT", |n: f64| -> i64 { n.trunc() as i64 });

    let result: i64 = engine.eval("INT(-3.9)").unwrap();
    assert_eq!(result, -3);
}

#[test]
fn test_fix_alias() {
    let mut engine = Engine::new();
    engine.register_fn("FIX", |n: f64| -> i64 { n.trunc() as i64 });

    let result: i64 = engine.eval("FIX(7.8)").unwrap();
    assert_eq!(result, 7);
}

// =============================================================================
// FLOOR / CEIL Function Tests
// =============================================================================

#[test]
fn test_floor_positive() {
    let mut engine = Engine::new();
    engine.register_fn("FLOOR", |n: f64| -> i64 { n.floor() as i64 });

    let result: i64 = engine.eval("FLOOR(3.9)").unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_floor_negative() {
    let mut engine = Engine::new();
    engine.register_fn("FLOOR", |n: f64| -> i64 { n.floor() as i64 });

    let result: i64 = engine.eval("FLOOR(-3.1)").unwrap();
    assert_eq!(result, -4);
}

#[test]
fn test_ceil_positive() {
    let mut engine = Engine::new();
    engine.register_fn("CEIL", |n: f64| -> i64 { n.ceil() as i64 });

    let result: i64 = engine.eval("CEIL(3.1)").unwrap();
    assert_eq!(result, 4);
}

#[test]
fn test_ceil_negative() {
    let mut engine = Engine::new();
    engine.register_fn("CEIL", |n: f64| -> i64 { n.ceil() as i64 });

    let result: i64 = engine.eval("CEIL(-3.9)").unwrap();
    assert_eq!(result, -3);
}

// =============================================================================
// MIN / MAX Function Tests
// =============================================================================

#[test]
fn test_max_basic() {
    let mut engine = Engine::new();
    engine.register_fn("MAX", |a: i64, b: i64| -> i64 { a.max(b) });

    let result: i64 = engine.eval("MAX(5, 10)").unwrap();
    assert_eq!(result, 10);
}

#[test]
fn test_max_first_larger() {
    let mut engine = Engine::new();
    engine.register_fn("MAX", |a: i64, b: i64| -> i64 { a.max(b) });

    let result: i64 = engine.eval("MAX(10, 5)").unwrap();
    assert_eq!(result, 10);
}

#[test]
fn test_max_equal() {
    let mut engine = Engine::new();
    engine.register_fn("MAX", |a: i64, b: i64| -> i64 { a.max(b) });

    let result: i64 = engine.eval("MAX(7, 7)").unwrap();
    assert_eq!(result, 7);
}

#[test]
fn test_max_negative() {
    let mut engine = Engine::new();
    engine.register_fn("MAX", |a: i64, b: i64| -> i64 { a.max(b) });

    let result: i64 = engine.eval("MAX(-5, -10)").unwrap();
    assert_eq!(result, -5);
}

#[test]
fn test_min_basic() {
    let mut engine = Engine::new();
    engine.register_fn("MIN", |a: i64, b: i64| -> i64 { a.min(b) });

    let result: i64 = engine.eval("MIN(5, 10)").unwrap();
    assert_eq!(result, 5);
}

#[test]
fn test_min_first_smaller() {
    let mut engine = Engine::new();
    engine.register_fn("MIN", |a: i64, b: i64| -> i64 { a.min(b) });

    let result: i64 = engine.eval("MIN(3, 8)").unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_min_negative() {
    let mut engine = Engine::new();
    engine.register_fn("MIN", |a: i64, b: i64| -> i64 { a.min(b) });

    let result: i64 = engine.eval("MIN(-5, -10)").unwrap();
    assert_eq!(result, -10);
}

// =============================================================================
// MOD Function Tests
// =============================================================================

#[test]
fn test_mod_basic() {
    let mut engine = Engine::new();
    engine.register_fn("MOD", |a: i64, b: i64| -> i64 { a % b });

    let result: i64 = engine.eval("MOD(17, 5)").unwrap();
    assert_eq!(result, 2);
}

#[test]
fn test_mod_no_remainder() {
    let mut engine = Engine::new();
    engine.register_fn("MOD", |a: i64, b: i64| -> i64 { a % b });

    let result: i64 = engine.eval("MOD(10, 5)").unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_mod_smaller_dividend() {
    let mut engine = Engine::new();
    engine.register_fn("MOD", |a: i64, b: i64| -> i64 { a % b });

    let result: i64 = engine.eval("MOD(3, 10)").unwrap();
    assert_eq!(result, 3);
}

// =============================================================================
// SGN Function Tests
// =============================================================================

#[test]
fn test_sgn_positive() {
    let mut engine = Engine::new();
    engine.register_fn("SGN", |n: i64| -> i64 { n.signum() });

    let result: i64 = engine.eval("SGN(42)").unwrap();
    assert_eq!(result, 1);
}

#[test]
fn test_sgn_negative() {
    let mut engine = Engine::new();
    engine.register_fn("SGN", |n: i64| -> i64 { n.signum() });

    let result: i64 = engine.eval("SGN(-42)").unwrap();
    assert_eq!(result, -1);
}

#[test]
fn test_sgn_zero() {
    let mut engine = Engine::new();
    engine.register_fn("SGN", |n: i64| -> i64 { n.signum() });

    let result: i64 = engine.eval("SGN(0)").unwrap();
    assert_eq!(result, 0);
}

// =============================================================================
// SQRT / SQR Function Tests
// =============================================================================

#[test]
fn test_sqrt_perfect_square() {
    let mut engine = Engine::new();
    engine.register_fn("SQRT", |n: f64| -> f64 { n.sqrt() });

    let result: f64 = engine.eval("SQRT(16.0)").unwrap();
    assert!((result - 4.0).abs() < f64::EPSILON);
}

#[test]
fn test_sqrt_non_perfect() {
    let mut engine = Engine::new();
    engine.register_fn("SQRT", |n: f64| -> f64 { n.sqrt() });

    let result: f64 = engine.eval("SQRT(2.0)").unwrap();
    assert!((result - std::f64::consts::SQRT_2).abs() < 0.00001);
}

#[test]
fn test_sqr_alias() {
    let mut engine = Engine::new();
    engine.register_fn("SQR", |n: f64| -> f64 { n.sqrt() });

    let result: f64 = engine.eval("SQR(25.0)").unwrap();
    assert!((result - 5.0).abs() < f64::EPSILON);
}

// =============================================================================
// POW Function Tests
// =============================================================================

#[test]
fn test_pow_basic() {
    let mut engine = Engine::new();
    engine.register_fn("POW", |base: f64, exp: f64| -> f64 { base.powf(exp) });

    let result: f64 = engine.eval("POW(2.0, 10.0)").unwrap();
    assert!((result - 1024.0).abs() < f64::EPSILON);
}

#[test]
fn test_pow_zero_exponent() {
    let mut engine = Engine::new();
    engine.register_fn("POW", |base: f64, exp: f64| -> f64 { base.powf(exp) });

    let result: f64 = engine.eval("POW(5.0, 0.0)").unwrap();
    assert!((result - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_pow_square_root() {
    let mut engine = Engine::new();
    engine.register_fn("POW", |base: f64, exp: f64| -> f64 { base.powf(exp) });

    let result: f64 = engine.eval("POW(9.0, 0.5)").unwrap();
    assert!((result - 3.0).abs() < 0.00001);
}

// =============================================================================
// LOG / LOG10 / EXP Function Tests
// =============================================================================

#[test]
fn test_log_e() {
    let mut engine = Engine::new();
    engine.register_fn("LOG", |n: f64| -> f64 { n.ln() });

    let e = std::f64::consts::E;
    let result: f64 = engine.eval(&format!("LOG({})", e)).unwrap();
    assert!((result - 1.0).abs() < 0.00001);
}

#[test]
fn test_log10_hundred() {
    let mut engine = Engine::new();
    engine.register_fn("LOG10", |n: f64| -> f64 { n.log10() });

    let result: f64 = engine.eval("LOG10(100.0)").unwrap();
    assert!((result - 2.0).abs() < f64::EPSILON);
}

#[test]
fn test_exp_zero() {
    let mut engine = Engine::new();
    engine.register_fn("EXP", |n: f64| -> f64 { n.exp() });

    let result: f64 = engine.eval("EXP(0.0)").unwrap();
    assert!((result - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_exp_one() {
    let mut engine = Engine::new();
    engine.register_fn("EXP", |n: f64| -> f64 { n.exp() });

    let result: f64 = engine.eval("EXP(1.0)").unwrap();
    assert!((result - std::f64::consts::E).abs() < 0.00001);
}

// =============================================================================
// Trigonometric Function Tests
// =============================================================================

#[test]
fn test_sin_zero() {
    let mut engine = Engine::new();
    engine.register_fn("SIN", |n: f64| -> f64 { n.sin() });

    let result: f64 = engine.eval("SIN(0.0)").unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_cos_zero() {
    let mut engine = Engine::new();
    engine.register_fn("COS", |n: f64| -> f64 { n.cos() });

    let result: f64 = engine.eval("COS(0.0)").unwrap();
    assert!((result - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_tan_zero() {
    let mut engine = Engine::new();
    engine.register_fn("TAN", |n: f64| -> f64 { n.tan() });

    let result: f64 = engine.eval("TAN(0.0)").unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_pi_constant() {
    let mut engine = Engine::new();
    engine.register_fn("PI", || -> f64 { std::f64::consts::PI });

    let result: f64 = engine.eval("PI()").unwrap();
    assert!((result - std::f64::consts::PI).abs() < f64::EPSILON);
}

// =============================================================================
// VAL Function Tests (String to Number)
// =============================================================================

#[test]
fn test_val_integer() {
    let mut engine = Engine::new();
    engine.register_fn("VAL", |s: &str| -> f64 {
        s.trim().parse::<f64>().unwrap_or(0.0)
    });

    let result: f64 = engine.eval(r#"VAL("42")"#).unwrap();
    assert!((result - 42.0).abs() < f64::EPSILON);
}

#[test]
fn test_val_decimal() {
    let mut engine = Engine::new();
    engine.register_fn("VAL", |s: &str| -> f64 {
        s.trim().parse::<f64>().unwrap_or(0.0)
    });

    let result: f64 = engine.eval(r#"VAL("3.14")"#).unwrap();
    assert!((result - 3.14).abs() < f64::EPSILON);
}

#[test]
fn test_val_negative() {
    let mut engine = Engine::new();
    engine.register_fn("VAL", |s: &str| -> f64 {
        s.trim().parse::<f64>().unwrap_or(0.0)
    });

    let result: f64 = engine.eval(r#"VAL("-17")"#).unwrap();
    assert!((result - (-17.0)).abs() < f64::EPSILON);
}

#[test]
fn test_val_invalid_returns_zero() {
    let mut engine = Engine::new();
    engine.register_fn("VAL", |s: &str| -> f64 {
        s.trim().parse::<f64>().unwrap_or(0.0)
    });

    let result: f64 = engine.eval(r#"VAL("abc")"#).unwrap();
    assert!((result - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_val_with_whitespace() {
    let mut engine = Engine::new();
    engine.register_fn("VAL", |s: &str| -> f64 {
        s.trim().parse::<f64>().unwrap_or(0.0)
    });

    let result: f64 = engine.eval(r#"VAL("  42  ")"#).unwrap();
    assert!((result - 42.0).abs() < f64::EPSILON);
}

// =============================================================================
// Combined Math Expression Tests
// =============================================================================

#[test]
fn test_combined_abs_sqrt() {
    let mut engine = Engine::new();
    engine.register_fn("ABS", |n: f64| -> f64 { n.abs() });
    engine.register_fn("SQRT", |n: f64| -> f64 { n.sqrt() });

    // SQRT(ABS(-16)) should be 4
    let result: f64 = engine.eval("SQRT(ABS(-16.0))").unwrap();
    assert!((result - 4.0).abs() < f64::EPSILON);
}

#[test]
fn test_combined_round_after_division() {
    let mut engine = Engine::new();
    engine.register_fn("ROUND", |n: f64| -> i64 { n.round() as i64 });

    // ROUND(10.0 / 3.0) should be 3
    let result: i64 = engine.eval("ROUND(10.0 / 3.0)").unwrap();
    assert_eq!(result, 3);
}

#[test]
fn test_combined_max_of_abs() {
    let mut engine = Engine::new();
    engine.register_fn("ABS", |n: i64| -> i64 { n.abs() });
    engine.register_fn("MAX", |a: i64, b: i64| -> i64 { a.max(b) });

    // MAX(ABS(-5), ABS(-10)) should be 10
    let result: i64 = engine.eval("MAX(ABS(-5), ABS(-10))").unwrap();
    assert_eq!(result, 10);
}

#[test]
fn test_arithmetic_expression() {
    let engine = Engine::new();

    // Test standard arithmetic without custom functions
    let result: i64 = engine.eval("2 + 3 * 4").unwrap();
    assert_eq!(result, 14); // Verify operator precedence

    let result: i64 = engine.eval("(2 + 3) * 4").unwrap();
    assert_eq!(result, 20);
}
