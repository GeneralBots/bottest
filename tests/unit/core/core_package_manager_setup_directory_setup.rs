


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_directory_setup_creation() {
        let setup = DirectorySetup::new(
            "http://localhost:8080".to_string(),
            PathBuf::from("/tmp/directory_config.json"),
        );
        assert_eq!(setup.base_url, "http://localhost:8080");
    }