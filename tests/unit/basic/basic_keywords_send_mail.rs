


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_apply_template_variables() {
        let template = "Hello {{name}}, your order {{order_id}} is ready!";
        let vars = json!({
            "name": "John",
            "order_id": "12345"
        });

        let result = apply_template_variables(template, &vars, "john@example.com").unwrap();
        assert!(result.contains("John"));
        assert!(result.contains("12345"));
    }

    #[test]


    fn test_extract_template_subject() {
        let content = "Subject: Welcome to our service\n\nHello there!";
        let subject = extract_template_subject(content);
        assert_eq!(subject, Some("Welcome to our service".to_string()));
    }