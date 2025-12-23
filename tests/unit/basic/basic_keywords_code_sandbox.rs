


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_sandbox_config_default() {
        let config = SandboxConfig::default();
        assert!(config.enabled);
        assert_eq!(config.timeout_seconds, 30);
        assert_eq!(config.memory_limit_mb, 256);
        assert!(!config.network_enabled);
    }

    #[test]


    fn test_execution_result_success() {
        let result = ExecutionResult::success("Hello, World!".to_string(), String::new(), 100);
        assert!(result.is_success());
        assert_eq!(result.output(), "Hello, World!");
    }

    #[test]


    fn test_execution_result_error() {
        let result = ExecutionResult::error("Something went wrong");
        assert!(!result.is_success());
        assert!(result.output().contains("Error"));
    }

    #[test]


    fn test_execution_result_timeout() {
        let result = ExecutionResult::timeout();
        assert!(!result.is_success());
        assert!(result.timed_out);
    }

    #[test]


    fn test_code_language_from_str() {
        assert_eq!(CodeLanguage::from("python"), CodeLanguage::Python);
        assert_eq!(CodeLanguage::from("PYTHON"), CodeLanguage::Python);
        assert_eq!(CodeLanguage::from("py"), CodeLanguage::Python);
        assert_eq!(CodeLanguage::from("javascript"), CodeLanguage::JavaScript);
        assert_eq!(CodeLanguage::from("js"), CodeLanguage::JavaScript);
        assert_eq!(CodeLanguage::from("node"), CodeLanguage::JavaScript);
        assert_eq!(CodeLanguage::from("bash"), CodeLanguage::Bash);
    }

    #[test]


    fn test_code_language_file_extension() {
        assert_eq!(CodeLanguage::Python.file_extension(), "py");
        assert_eq!(CodeLanguage::JavaScript.file_extension(), "js");
        assert_eq!(CodeLanguage::Bash.file_extension(), "sh");
    }

    #[test]


    fn test_code_language_interpreter() {
        assert_eq!(CodeLanguage::Python.interpreter(), "python3");
        assert_eq!(CodeLanguage::JavaScript.interpreter(), "node");
        assert_eq!(CodeLanguage::Bash.interpreter(), "bash");
    }

    #[test]


    fn test_sandbox_runtime_from_str() {
        assert_eq!(SandboxRuntime::from("lxc"), SandboxRuntime::LXC);
        assert_eq!(SandboxRuntime::from("docker"), SandboxRuntime::Docker);
        assert_eq!(
            SandboxRuntime::from("firecracker"),
            SandboxRuntime::Firecracker
        );
        assert_eq!(SandboxRuntime::from("unknown"), SandboxRuntime::Process);
    }

    #[test]


    fn test_lxc_config_generation() {
        let python_config = generate_python_lxc_config();
        assert!(python_config.contains("gb-sandbox-python"));
        assert!(python_config.contains("memory.max"));

        let node_config = generate_node_lxc_config();
        assert!(node_config.contains("gb-sandbox-node"));
        assert!(node_config.contains("/usr/bin/node"));
    }