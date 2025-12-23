


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_email_monitor_struct() {
        let monitor = EmailMonitor {
            id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            email_address: "test@example.com".to_string(),
            script_path: "on_email_test.rhai".to_string(),
            is_active: true,
            filter_from: None,
            filter_subject: None,
        };

        assert_eq!(monitor.email_address, "test@example.com");
        assert!(monitor.is_active);
        assert!(monitor.filter_from.is_none());
        assert!(monitor.filter_subject.is_none());
    }

    #[test]


    fn test_email_monitor_with_filters() {
        let monitor = EmailMonitor {
            id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            email_address: "orders@company.com".to_string(),
            script_path: "on_email_orders.rhai".to_string(),
            is_active: true,
            filter_from: Some("supplier@vendor.com".to_string()),
            filter_subject: Some("Invoice".to_string()),
        };

        assert_eq!(monitor.email_address, "orders@company.com");
        assert_eq!(monitor.filter_from, Some("supplier@vendor.com".to_string()));
        assert_eq!(monitor.filter_subject, Some("Invoice".to_string()));
    }

    #[test]


    fn test_email_attachment_struct() {
        let attachment = EmailAttachment {
            filename: "document.pdf".to_string(),
            mime_type: "application/pdf".to_string(),
            size: 1024,
        };

        assert_eq!(attachment.filename, "document.pdf");
        assert_eq!(attachment.mime_type, "application/pdf");
        assert_eq!(attachment.size, 1024);
    }

    #[test]


    fn test_email_received_event_struct() {
        let event = EmailReceivedEvent {
            id: Uuid::new_v4(),
            monitor_id: Uuid::new_v4(),
            message_uid: 12345,
            message_id: Some("<msg123@example.com>".to_string()),
            from_address: "sender@example.com".to_string(),
            to_addresses: vec!["recipient@example.com".to_string()],
            subject: Some("Test Subject".to_string()),
            has_attachments: true,
            attachments: vec![EmailAttachment {
                filename: "file.pdf".to_string(),
                mime_type: "application/pdf".to_string(),
                size: 2048,
            }],
        };

        assert_eq!(event.message_uid, 12345);
        assert_eq!(event.from_address, "sender@example.com");
        assert!(event.has_attachments);
        assert_eq!(event.attachments.len(), 1);
        assert_eq!(event.attachments[0].filename, "file.pdf");
    }

    #[test]


    fn test_parse_email_path_basic() {
        let result = parse_email_path("email://user@gmail.com");
        assert!(result.is_some());
        let (email, folder) = result.unwrap();
        assert_eq!(email, "user@gmail.com");
        assert!(folder.is_none());
    }

    #[test]


    fn test_parse_email_path_with_folder() {
        let result = parse_email_path("email://user@gmail.com/INBOX");
        assert!(result.is_some());
        let (email, folder) = result.unwrap();
        assert_eq!(email, "user@gmail.com");
        assert_eq!(folder, Some("INBOX".to_string()));
    }

    #[test]


    fn test_parse_email_path_invalid() {
        assert!(parse_email_path("user@gmail.com").is_none());
        assert!(parse_email_path("mailto:user@gmail.com").is_none());
        assert!(parse_email_path("/local/path").is_none());
    }

    #[test]


    fn test_is_email_path() {
        assert!(is_email_path("email://user@gmail.com"));
        assert!(is_email_path("email://user@company.com/INBOX"));
        assert!(!is_email_path("user@gmail.com"));
        assert!(!is_email_path("mailto:user@gmail.com"));
        assert!(!is_email_path("account://user@gmail.com"));
    }

    #[test]


    fn test_sanitize_email_for_filename() {
        assert_eq!(
            sanitize_email_for_filename("user@gmail.com"),
            "user_at_gmail_com"
        );
        assert_eq!(
            sanitize_email_for_filename("test.user@company.co.uk"),
            "test_user_at_company_co_uk"
        );
        assert_eq!(
            sanitize_email_for_filename("USER@EXAMPLE.COM"),
            "user_at_example_com"
        );
    }

    #[test]


    fn test_email_event_without_attachments() {
        let event = EmailReceivedEvent {
            id: Uuid::new_v4(),
            monitor_id: Uuid::new_v4(),
            message_uid: 1,
            message_id: None,
            from_address: "no-reply@system.com".to_string(),
            to_addresses: vec![],
            subject: None,
            has_attachments: false,
            attachments: vec![],
        };

        assert!(!event.has_attachments);
        assert!(event.attachments.is_empty());
        assert!(event.subject.is_none());
    }

    #[test]


    fn test_multiple_to_addresses() {
        let event = EmailReceivedEvent {
            id: Uuid::new_v4(),
            monitor_id: Uuid::new_v4(),
            message_uid: 999,
            message_id: Some("<multi@example.com>".to_string()),
            from_address: "sender@example.com".to_string(),
            to_addresses: vec![
                "user1@example.com".to_string(),
                "user2@example.com".to_string(),
                "user3@example.com".to_string(),
            ],
            subject: Some("Group Message".to_string()),
            has_attachments: false,
            attachments: vec![],
        };

        assert_eq!(event.to_addresses.len(), 3);
        assert!(event
            .to_addresses
            .contains(&"user2@example.com".to_string()));
    }

    #[test]


    fn test_multiple_attachments() {
        let attachments = vec![
            EmailAttachment {
                filename: "doc1.pdf".to_string(),
                mime_type: "application/pdf".to_string(),
                size: 1024,
            },
            EmailAttachment {
                filename: "image.png".to_string(),
                mime_type: "image/png".to_string(),
                size: 2048,
            },
            EmailAttachment {
                filename: "data.xlsx".to_string(),
                mime_type: "application/vnd.openxmlformats-officedocument.spreadsheetml.sheet"
                    .to_string(),
                size: 4096,
            },
        ];

        assert_eq!(attachments.len(), 3);
        assert_eq!(attachments[0].filename, "doc1.pdf");
        assert_eq!(attachments[1].mime_type, "image/png");
        assert_eq!(attachments[2].size, 4096);
    }