//! Unit tests migrated from src/basic/keywords/procedures.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;

// Original: use super::*; - tests used internal functions from botserver

    fn setup() {
        clear_procedures();
    }

    #[test]

    
    fn test_preprocess_sub() {
        setup();

        let input = r#"
x = 1
SUB MySub(a, b)
    TALK a + b
END SUB
y = 2
"#;

        let result = preprocess_subs(input);

        // SUB should be extracted
        assert!(!result.contains("SUB MySub"));
        assert!(!result.contains("END SUB"));
        assert!(result.contains("x = 1"));
        assert!(result.contains("y = 2"));

        // Procedure should be registered
        assert!(has_procedure("MYSUB"));
        let proc = get_procedure("MYSUB").unwrap();
        assert_eq!(proc.params.len(), 2);
        assert!(!proc.is_function);
    }

    #[test]

    
    fn test_preprocess_function() {
        setup();

        let input = r#"
FUNCTION Add(a, b)
    RETURN a + b
END FUNCTION
result = Add(1, 2)
"#;

        let result = preprocess_functions(input);

        // FUNCTION should be extracted
        assert!(!result.contains("FUNCTION Add"));
        assert!(!result.contains("END FUNCTION"));
        assert!(result.contains("result = Add(1, 2)"));

        // Procedure should be registered
        assert!(has_procedure("ADD"));
        let proc = get_procedure("ADD").unwrap();
        assert!(proc.is_function);
    }

    #[test]

    
    fn test_preprocess_sub_no_params() {
        setup();

        let input = r#"
SUB PrintHello
    TALK "Hello"
END SUB
"#;

        preprocess_subs(input);

        assert!(has_procedure("PRINTHELLO"));
        let proc = get_procedure("PRINTHELLO").unwrap();
        assert!(proc.params.is_empty());
    }

    #[test]

    
    fn test_preprocess_call() {
        setup();

        // First register a SUB
        let sub_input = r#"
SUB Greet(name)
    TALK "Hello " + name
END SUB
"#;
        preprocess_subs(sub_input);

        // Then preprocess CALL
        let call_input = "CALL Greet(\"World\")";
        let result = preprocess_calls(call_input);

        // Should contain parameter assignment and body
        assert!(result.contains("let name = \"World\""));
        assert!(result.contains("TALK \"Hello \" + name"));
    }

    #[test]

    
    fn test_eval_bool_condition() {
        assert!(eval_bool_condition(&Dynamic::from(true)));
        assert!(!eval_bool_condition(&Dynamic::from(false)));
        assert!(eval_bool_condition(&Dynamic::from(1)));
        assert!(!eval_bool_condition(&Dynamic::from(0)));
        assert!(eval_bool_condition(&Dynamic::from(1.5)));
        assert!(!eval_bool_condition(&Dynamic::from(0.0)));
        assert!(eval_bool_condition(&Dynamic::from("hello")));
        assert!(!eval_bool_condition(&Dynamic::from("")));
        assert!(!eval_bool_condition(&Dynamic::from("false")));
        assert!(!eval_bool_condition(&Dynamic::from("0")));
    }

    #[test]

    
    fn test_clear_procedures() {
        setup();

        let input = "SUB Test\n    TALK \"test\"\nEND SUB";
        preprocess_subs(input);

        assert!(has_procedure("TEST"));

        clear_procedures();

        assert!(!has_procedure("TEST"));
    }

    #[test]

    
    fn test_full_pipeline() {
        setup();

        let input = r#"
SUB SendGreeting(name, greeting)
    TALK greeting + ", " + name + "!"
END SUB

FUNCTION Calculate(x, y)
    result = x * y + 10
    RETURN result
END FUNCTION

' Main code
CALL SendGreeting("User", "Hello")
total = Calculate(5, 3)
"#;

        let result = preprocess_procedures(input);

        // Should have inlined the CALL
        assert!(result.contains("let name = \"User\""));
        assert!(result.contains("let greeting = \"Hello\""));

        // Original definitions should be gone
        assert!(!result.contains("SUB SendGreeting"));
        assert!(!result.contains("END SUB"));
        assert!(!result.contains("FUNCTION Calculate"));
        assert!(!result.contains("END FUNCTION"));

        // Both should be registered
        assert!(has_procedure("SENDGREETING"));
        assert!(has_procedure("CALCULATE"));
    }