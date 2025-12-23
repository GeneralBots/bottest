
use rhai::Engine;


#[test]
fn test_instr_finds_substring() {
    let mut engine = Engine::new();

    engine.register_fn("INSTR", |haystack: &str, needle: &str| -> i64 {
        if haystack.is_empty() || needle.is_empty() {
            return 0;
        }
        match haystack.find(needle) {
            Some(pos) => (pos + 1) as i64,
            None => 0,
        }
    });

    let result: i64 = engine.eval(r#"INSTR("Hello World", "World")"#).unwrap();
    assert_eq!(result, 7);
}

#[test]
fn test_instr_not_found() {
    let mut engine = Engine::new();

    engine.register_fn("INSTR", |haystack: &str, needle: &str| -> i64 {
        if haystack.is_empty() || needle.is_empty() {
            return 0;
        }
        match haystack.find(needle) {
            Some(pos) => (pos + 1) as i64,
            None => 0,
        }
    });

    let result: i64 = engine.eval(r#"INSTR("Hello World", "xyz")"#).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_instr_case_sensitive() {
    let mut engine = Engine::new();

    engine.register_fn("INSTR", |haystack: &str, needle: &str| -> i64 {
        if haystack.is_empty() || needle.is_empty() {
            return 0;
        }
        match haystack.find(needle) {
            Some(pos) => (pos + 1) as i64,
            None => 0,
        }
    });

    let result: i64 = engine.eval(r#"INSTR("Hello", "hello")"#).unwrap();
    assert_eq!(result, 0);
}


#[test]
fn test_upper_basic() {
    let mut engine = Engine::new();
    engine.register_fn("UPPER", |s: &str| -> String { s.to_uppercase() });

    let result: String = engine.eval(r#"UPPER("hello")"#).unwrap();
    assert_eq!(result, "HELLO");
}

#[test]
fn test_upper_mixed_case() {
    let mut engine = Engine::new();
    engine.register_fn("UPPER", |s: &str| -> String { s.to_uppercase() });

    let result: String = engine.eval(r#"UPPER("HeLLo WoRLd")"#).unwrap();
    assert_eq!(result, "HELLO WORLD");
}

#[test]
fn test_ucase_alias() {
    let mut engine = Engine::new();
    engine.register_fn("UCASE", |s: &str| -> String { s.to_uppercase() });

    let result: String = engine.eval(r#"UCASE("test")"#).unwrap();
    assert_eq!(result, "TEST");
}


#[test]
fn test_lower_basic() {
    let mut engine = Engine::new();
    engine.register_fn("LOWER", |s: &str| -> String { s.to_lowercase() });

    let result: String = engine.eval(r#"LOWER("HELLO")"#).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_lcase_alias() {
    let mut engine = Engine::new();
    engine.register_fn("LCASE", |s: &str| -> String { s.to_lowercase() });

    let result: String = engine.eval(r#"LCASE("TEST")"#).unwrap();
    assert_eq!(result, "test");
}


#[test]
fn test_len_basic() {
    let mut engine = Engine::new();
    engine.register_fn("LEN", |s: &str| -> i64 { s.len() as i64 });

    let result: i64 = engine.eval(r#"LEN("Hello")"#).unwrap();
    assert_eq!(result, 5);
}

#[test]
fn test_len_empty() {
    let mut engine = Engine::new();
    engine.register_fn("LEN", |s: &str| -> i64 { s.len() as i64 });

    let result: i64 = engine.eval(r#"LEN("")"#).unwrap();
    assert_eq!(result, 0);
}

#[test]
fn test_len_with_spaces() {
    let mut engine = Engine::new();
    engine.register_fn("LEN", |s: &str| -> i64 { s.len() as i64 });

    let result: i64 = engine.eval(r#"LEN("Hello World")"#).unwrap();
    assert_eq!(result, 11);
}


#[test]
fn test_trim_both_sides() {
    let mut engine = Engine::new();
    engine.register_fn("TRIM", |s: &str| -> String { s.trim().to_string() });

    let result: String = engine.eval(r#"TRIM("  hello  ")"#).unwrap();
    assert_eq!(result, "hello");
}

#[test]
fn test_ltrim() {
    let mut engine = Engine::new();
    engine.register_fn("LTRIM", |s: &str| -> String { s.trim_start().to_string() });

    let result: String = engine.eval(r#"LTRIM("  hello  ")"#).unwrap();
    assert_eq!(result, "hello  ");
}

#[test]
fn test_rtrim() {
    let mut engine = Engine::new();
    engine.register_fn("RTRIM", |s: &str| -> String { s.trim_end().to_string() });

    let result: String = engine.eval(r#"RTRIM("  hello  ")"#).unwrap();
    assert_eq!(result, "  hello");
}


#[test]
fn test_left_basic() {
    let mut engine = Engine::new();
    engine.register_fn("LEFT", |s: &str, count: i64| -> String {
        let count = count.max(0) as usize;
        s.chars().take(count).collect()
    });

    let result: String = engine.eval(r#"LEFT("Hello World", 5)"#).unwrap();
    assert_eq!(result, "Hello");
}

#[test]
fn test_left_exceeds_length() {
    let mut engine = Engine::new();
    engine.register_fn("LEFT", |s: &str, count: i64| -> String {
        let count = count.max(0) as usize;
        s.chars().take(count).collect()
    });

    let result: String = engine.eval(r#"LEFT("Hi", 10)"#).unwrap();
    assert_eq!(result, "Hi");
}

#[test]
fn test_left_zero() {
    let mut engine = Engine::new();
    engine.register_fn("LEFT", |s: &str, count: i64| -> String {
        let count = count.max(0) as usize;
        s.chars().take(count).collect()
    });

    let result: String = engine.eval(r#"LEFT("Hello", 0)"#).unwrap();
    assert_eq!(result, "");
}


#[test]
fn test_right_basic() {
    let mut engine = Engine::new();
    engine.register_fn("RIGHT", |s: &str, count: i64| -> String {
        let count = count.max(0) as usize;
        let len = s.chars().count();
        if count >= len {
            s.to_string()
        } else {
            s.chars().skip(len - count).collect()
        }
    });

    let result: String = engine.eval(r#"RIGHT("Hello World", 5)"#).unwrap();
    assert_eq!(result, "World");
}

#[test]
fn test_right_exceeds_length() {
    let mut engine = Engine::new();
    engine.register_fn("RIGHT", |s: &str, count: i64| -> String {
        let count = count.max(0) as usize;
        let len = s.chars().count();
        if count >= len {
            s.to_string()
        } else {
            s.chars().skip(len - count).collect()
        }
    });

    let result: String = engine.eval(r#"RIGHT("Hi", 10)"#).unwrap();
    assert_eq!(result, "Hi");
}


#[test]
fn test_mid_with_length() {
    let mut engine = Engine::new();
    engine.register_fn("MID", |s: &str, start: i64, length: i64| -> String {
        let start_idx = if start < 1 { 0 } else { (start - 1) as usize };
        let len = length.max(0) as usize;
        s.chars().skip(start_idx).take(len).collect()
    });

    let result: String = engine.eval(r#"MID("Hello World", 7, 5)"#).unwrap();
    assert_eq!(result, "World");
}

#[test]
fn test_mid_one_based_index() {
    let mut engine = Engine::new();
    engine.register_fn("MID", |s: &str, start: i64, length: i64| -> String {
        let start_idx = if start < 1 { 0 } else { (start - 1) as usize };
        let len = length.max(0) as usize;
        s.chars().skip(start_idx).take(len).collect()
    });

    let result: String = engine.eval(r#"MID("ABCDE", 1, 1)"#).unwrap();
    assert_eq!(result, "A");

    let result: String = engine.eval(r#"MID("ABCDE", 3, 1)"#).unwrap();
    assert_eq!(result, "C");
}


#[test]
fn test_replace_basic() {
    let mut engine = Engine::new();
    engine.register_fn("REPLACE", |s: &str, find: &str, replace: &str| -> String {
        s.replace(find, replace)
    });

    let result: String = engine
        .eval(r#"REPLACE("Hello World", "World", "Rust")"#)
        .unwrap();
    assert_eq!(result, "Hello Rust");
}

#[test]
fn test_replace_multiple() {
    let mut engine = Engine::new();
    engine.register_fn("REPLACE", |s: &str, find: &str, replace: &str| -> String {
        s.replace(find, replace)
    });

    let result: String = engine.eval(r#"REPLACE("aaa", "a", "b")"#).unwrap();
    assert_eq!(result, "bbb");
}

#[test]
fn test_replace_not_found() {
    let mut engine = Engine::new();
    engine.register_fn("REPLACE", |s: &str, find: &str, replace: &str| -> String {
        s.replace(find, replace)
    });

    let result: String = engine.eval(r#"REPLACE("Hello", "xyz", "abc")"#).unwrap();
    assert_eq!(result, "Hello");
}


#[test]
fn test_is_numeric_integer() {
    let mut engine = Engine::new();
    engine.register_fn("IS_NUMERIC", |value: &str| -> bool {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return false;
        }
        trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok()
    });

    let result: bool = engine.eval(r#"IS_NUMERIC("42")"#).unwrap();
    assert!(result);
}

#[test]
fn test_is_numeric_decimal() {
    let mut engine = Engine::new();
    engine.register_fn("IS_NUMERIC", |value: &str| -> bool {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return false;
        }
        trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok()
    });

    let result: bool = engine.eval(r#"IS_NUMERIC("3.14")"#).unwrap();
    assert!(result);
}

#[test]
fn test_is_numeric_invalid() {
    let mut engine = Engine::new();
    engine.register_fn("IS_NUMERIC", |value: &str| -> bool {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return false;
        }
        trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok()
    });

    let result: bool = engine.eval(r#"IS_NUMERIC("abc")"#).unwrap();
    assert!(!result);
}

#[test]
fn test_is_numeric_empty() {
    let mut engine = Engine::new();
    engine.register_fn("IS_NUMERIC", |value: &str| -> bool {
        let trimmed = value.trim();
        if trimmed.is_empty() {
            return false;
        }
        trimmed.parse::<i64>().is_ok() || trimmed.parse::<f64>().is_ok()
    });

    let result: bool = engine.eval(r#"IS_NUMERIC("")"#).unwrap();
    assert!(!result);
}


#[test]
fn test_combined_string_operations() {
    let mut engine = Engine::new();
    engine.register_fn("UPPER", |s: &str| -> String { s.to_uppercase() });
    engine.register_fn("TRIM", |s: &str| -> String { s.trim().to_string() });
    engine.register_fn("LEN", |s: &str| -> i64 { s.len() as i64 });

    let result: String = engine.eval(r#"UPPER(TRIM("  hello  "))"#).unwrap();
    assert_eq!(result, "HELLO");

    let result: i64 = engine.eval(r#"LEN(TRIM("  hi  "))"#).unwrap();
    assert_eq!(result, 2);
}
