


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_is_label_line() {
        assert!(is_label_line("start:"));
        assert!(is_label_line("  mainLoop:"));
        assert!(is_label_line("my_label:"));
        assert!(is_label_line("label123:"));

        assert!(!is_label_line("TALK \"hello:\""));
        assert!(!is_label_line("' comment:"));
        assert!(!is_label_line("CASE:"));
        assert!(!is_label_line("123label:"));
        assert!(!is_label_line("has space:"));
    }

    #[test]


    fn test_extract_goto_target() {
        assert_eq!(extract_goto_target("GOTO start"), Some("start".to_string()));
        assert_eq!(
            extract_goto_target("  GOTO myLabel"),
            Some("myLabel".to_string())
        );
        assert_eq!(
            extract_goto_target("IF x > 5 THEN GOTO done"),
            Some("done".to_string())
        );
        assert_eq!(extract_goto_target("TALK \"hello\""), None);
    }

    #[test]


    fn test_transform_line_simple_goto() {
        assert_eq!(
            transform_line("GOTO start"),
            "__goto_label = \"start\"; continue;"
        );
        assert_eq!(
            transform_line("  GOTO myLoop  "),
            "__goto_label = \"myLoop\"; continue;"
        );
    }

    #[test]


    fn test_transform_line_if_then_goto() {
        let result = transform_line("IF x < 10 THEN GOTO start");
        assert!(result.contains("if x < 10"));
        assert!(result.contains("__goto_label = \"start\""));
        assert!(result.contains("continue"));
    }

    #[test]


    fn test_transform_line_if_goto_no_then() {
        let result = transform_line("IF x < 10 GOTO start");
        assert!(result.contains("if x < 10"));
        assert!(result.contains("__goto_label = \"start\""));
    }

    #[test]


    fn test_transform_line_not_goto() {
        assert_eq!(transform_line("TALK \"Hello\""), "TALK \"Hello\"");
        assert_eq!(transform_line("x = x + 1"), "x = x + 1");
        assert_eq!(transform_line("ON ERROR GOTO 0"), "ON ERROR GOTO 0");
    }

    #[test]


    fn test_has_goto_constructs() {
        assert!(has_goto_constructs("start:\nTALK \"hi\"\nGOTO start"));
        assert!(has_goto_constructs("IF x > 0 THEN GOTO done"));
        assert!(!has_goto_constructs("TALK \"hello\"\nWAIT 1"));
        assert!(!has_goto_constructs("ON ERROR GOTO 0"));
    }

    #[test]


    fn test_transform_goto_simple() {
        let input = r#"start:
    TALK "Hello"
    x = x + 1
    IF x < 3 THEN GOTO start
    TALK "Done""#;

        let output = transform_goto(input);

        assert!(output.contains("__goto_label"));
        assert!(output.contains("while"));
        assert!(output.contains("\"start\""));
        assert!(output.contains("WARNING"));
    }

    #[test]


    fn test_transform_goto_no_goto() {
        let input = "TALK \"Hello\"\nTALK \"World\"";
        let output = transform_goto(input);
        assert_eq!(output, input);
    }

    #[test]


    fn test_transform_goto_multiple_labels() {
        let input = r#"start:
    TALK "Start"
    GOTO middle
middle:
    TALK "Middle"
    GOTO done
done:
    TALK "Done""#;

        let output = transform_goto(input);

        assert!(output.contains("\"start\""));
        assert!(output.contains("\"middle\""));
        assert!(output.contains("\"done\""));
    }

    #[test]


    fn test_infinite_loop_protection() {
        let output = transform_goto("loop:\nGOTO loop");
        assert!(output.contains("__goto_max_iterations"));
        assert!(output.contains("throw"));
    }