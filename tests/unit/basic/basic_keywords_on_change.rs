//! Unit tests migrated from src/basic/keywords/on_change.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_parse_folder_path_account() {
        let (provider, email, path) =
            parse_folder_path("account://user@gmail.com/Documents/invoices");
        assert_eq!(provider, FolderProvider::GDrive);
        assert_eq!(email, Some("user@gmail.com".to_string()));
        assert_eq!(path, "/Documents/invoices");
    }

    #[test]

    
    fn test_parse_folder_path_gdrive() {
        let (provider, email, path) = parse_folder_path("gdrive:///shared/reports");
        assert_eq!(provider, FolderProvider::GDrive);
        assert_eq!(email, None);
        assert_eq!(path, "/shared/reports");
    }

    #[test]

    
    fn test_parse_folder_path_onedrive() {
        let (provider, email, path) = parse_folder_path("onedrive:///business/docs");
        assert_eq!(provider, FolderProvider::OneDrive);
        assert_eq!(email, None);
        assert_eq!(path, "/business/docs");
    }

    #[test]

    
    fn test_parse_folder_path_dropbox() {
        let (provider, email, path) = parse_folder_path("dropbox:///team/assets");
        assert_eq!(provider, FolderProvider::Dropbox);
        assert_eq!(email, None);
        assert_eq!(path, "/team/assets");
    }

    #[test]

    
    fn test_parse_folder_path_local() {
        let (provider, email, path) = parse_folder_path("/home/user/documents");
        assert_eq!(provider, FolderProvider::Local);
        assert_eq!(email, None);
        assert_eq!(path, "/home/user/documents");
    }

    #[test]

    
    fn test_is_cloud_path() {
        assert!(is_cloud_path("account://user@gmail.com/docs"));
        assert!(is_cloud_path("gdrive:///shared"));
        assert!(is_cloud_path("onedrive:///files"));
        assert!(is_cloud_path("dropbox:///folder"));
        assert!(!is_cloud_path("/local/path"));
        assert!(!is_cloud_path("./relative/path"));
    }

    #[test]

    
    fn test_folder_provider_from_str() {
        assert_eq!(
            FolderProvider::from_str("gdrive"),
            Some(FolderProvider::GDrive)
        );
        assert_eq!(
            FolderProvider::from_str("GDRIVE"),
            Some(FolderProvider::GDrive)
        );
        assert_eq!(
            FolderProvider::from_str("googledrive"),
            Some(FolderProvider::GDrive)
        );
        assert_eq!(
            FolderProvider::from_str("onedrive"),
            Some(FolderProvider::OneDrive)
        );
        assert_eq!(
            FolderProvider::from_str("microsoft"),
            Some(FolderProvider::OneDrive)
        );
        assert_eq!(
            FolderProvider::from_str("dropbox"),
            Some(FolderProvider::Dropbox)
        );
        assert_eq!(
            FolderProvider::from_str("dbx"),
            Some(FolderProvider::Dropbox)
        );
        assert_eq!(
            FolderProvider::from_str("local"),
            Some(FolderProvider::Local)
        );
        assert_eq!(
            FolderProvider::from_str("filesystem"),
            Some(FolderProvider::Local)
        );
        assert_eq!(FolderProvider::from_str("unknown"), None);
    }

    #[test]

    
    fn test_change_event_type_from_str() {
        assert_eq!(
            ChangeEventType::from_str("create"),
            Some(ChangeEventType::Create)
        );
        assert_eq!(
            ChangeEventType::from_str("created"),
            Some(ChangeEventType::Create)
        );
        assert_eq!(
            ChangeEventType::from_str("modify"),
            Some(ChangeEventType::Modify)
        );
        assert_eq!(
            ChangeEventType::from_str("changed"),
            Some(ChangeEventType::Modify)
        );
        assert_eq!(
            ChangeEventType::from_str("delete"),
            Some(ChangeEventType::Delete)
        );
        assert_eq!(
            ChangeEventType::from_str("removed"),
            Some(ChangeEventType::Delete)
        );
        assert_eq!(
            ChangeEventType::from_str("rename"),
            Some(ChangeEventType::Rename)
        );
        assert_eq!(
            ChangeEventType::from_str("move"),
            Some(ChangeEventType::Move)
        );
        assert_eq!(ChangeEventType::from_str("invalid"), None);
    }

    #[test]

    
    fn test_sanitize_path() {
        assert_eq!(
            sanitize_path_for_filename("/home/user/docs"),
            "_home_user_docs"
        );
        assert_eq!(
            sanitize_path_for_filename("C:\\Users\\docs"),
            "c__users_docs"
        );
        assert_eq!(
            sanitize_path_for_filename("path with spaces"),
            "path_with_spaces"
        );
    }

    #[test]

    
    fn test_folder_monitor_struct() {
        let monitor = FolderMonitor {
            id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            provider: "gdrive".to_string(),
            account_email: Some("user@gmail.com".to_string()),
            folder_path: "/my/folder".to_string(),
            folder_id: Some("folder123".to_string()),
            script_path: "on_change.rhai".to_string(),
            is_active: true,
            watch_subfolders: true,
            event_types: vec!["create".to_string(), "modify".to_string()],
        };

        assert_eq!(monitor.provider, "gdrive");
        assert!(monitor.is_active);
        assert!(monitor.watch_subfolders);
        assert_eq!(monitor.account_email, Some("user@gmail.com".to_string()));
    }

    #[test]

    
    fn test_folder_change_event_struct() {
        let event = FolderChangeEvent {
            id: Uuid::new_v4(),
            monitor_id: Uuid::new_v4(),
            event_type: "create".to_string(),
            file_path: "/docs/new_file.pdf".to_string(),
            file_id: Some("file123".to_string()),
            file_name: Some("new_file.pdf".to_string()),
            file_size: Some(1024),
            mime_type: Some("application/pdf".to_string()),
            old_path: None,
        };

        assert_eq!(event.event_type, "create");
        assert_eq!(event.file_size, Some(1024));
    }

    #[test]

    
    fn test_detect_provider_from_email() {
        assert_eq!(
            detect_provider_from_email("user@gmail.com"),
            FolderProvider::GDrive
        );
        assert_eq!(
            detect_provider_from_email("user@outlook.com"),
            FolderProvider::OneDrive
        );
        assert_eq!(
            detect_provider_from_email("user@hotmail.com"),
            FolderProvider::OneDrive
        );
        assert_eq!(
            detect_provider_from_email("user@company.com"),
            FolderProvider::GDrive
        );
    }