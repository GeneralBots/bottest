


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_pattern_matching() {
        let scanner = CodeScanner::new("/tmp/test");


        let password_pattern = scanner
            .patterns
            .iter()
            .find(|p| matches!(p.issue_type, IssueType::PasswordInConfig))
            .unwrap();
        assert!(password_pattern.regex.is_match(r#"password = "secret123""#));
        assert!(password_pattern.regex.is_match(r#"PASSWORD = 'mypass'"#));


        let underscore_pattern = scanner
            .patterns
            .iter()
            .find(|p| matches!(p.issue_type, IssueType::UnderscoreInKeyword))
            .unwrap();
        assert!(underscore_pattern.regex.is_match("GET_BOT_MEMORY"));
        assert!(underscore_pattern.regex.is_match("SET_USER_MEMORY"));
    }

    #[test]


    fn test_severity_ordering() {
        assert!(IssueSeverity::Critical > IssueSeverity::High);
        assert!(IssueSeverity::High > IssueSeverity::Medium);
        assert!(IssueSeverity::Medium > IssueSeverity::Low);
        assert!(IssueSeverity::Low > IssueSeverity::Info);
    }

    #[test]


    fn test_stats_merge() {
        let mut stats1 = ScanStats {
            critical: 1,
            high: 2,
            medium: 3,
            low: 4,
            info: 5,
            total: 15,
        };

        let stats2 = ScanStats {
            critical: 1,
            high: 1,
            medium: 1,
            low: 1,
            info: 1,
            total: 5,
        };

        stats1.merge(&stats2);

        assert_eq!(stats1.critical, 2);
        assert_eq!(stats1.high, 3);
        assert_eq!(stats1.total, 20);
    }

    #[test]


    fn test_csv_escape() {
        assert_eq!(escape_csv("simple"), "simple");
        assert_eq!(escape_csv("with,comma"), "\"with,comma\"");
        assert_eq!(escape_csv("with\"quote"), "\"with\"\"quote\"");
    }

    #[test]


    fn test_redact_sensitive() {
        let scanner = CodeScanner::new("/tmp/test");

        let line = r#"password = "supersecretpassword123""#;
        let redacted = scanner.redact_sensitive(line);
        assert!(redacted.contains("***REDACTED***"));
        assert!(!redacted.contains("supersecretpassword123"));
    }