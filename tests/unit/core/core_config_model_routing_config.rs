


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_default_config() {
        let config = ModelRoutingConfig::default();
        assert_eq!(config.routing_strategy, RoutingStrategy::Default);
        assert_eq!(config.default_model, "gpt-4o");
        assert!(config.fallback_enabled);
        assert!(!config.fallback_order.is_empty());
    }

    #[test]


    fn test_routing_strategy_from_str() {
        assert_eq!(RoutingStrategy::from("default"), RoutingStrategy::Default);
        assert_eq!(
            RoutingStrategy::from("task-based"),
            RoutingStrategy::TaskBased
        );
        assert_eq!(
            RoutingStrategy::from("round-robin"),
            RoutingStrategy::RoundRobin
        );
        assert_eq!(RoutingStrategy::from("latency"), RoutingStrategy::Latency);
        assert_eq!(RoutingStrategy::from("cost"), RoutingStrategy::Cost);
        assert_eq!(RoutingStrategy::from("custom"), RoutingStrategy::Custom);
        assert_eq!(RoutingStrategy::from("unknown"), RoutingStrategy::Default);
    }

    #[test]


    fn test_get_model_for_task_default_strategy() {
        let config = ModelRoutingConfig::default();
        assert_eq!(config.get_model_for_task(TaskType::Simple), "gpt-4o");
        assert_eq!(config.get_model_for_task(TaskType::Complex), "gpt-4o");
        assert_eq!(config.get_model_for_task(TaskType::Code), "gpt-4o");
    }

    #[test]


    fn test_get_model_for_task_based_strategy() {
        let config = ModelRoutingConfig {
            routing_strategy: RoutingStrategy::TaskBased,
            ..Default::default()
        };
        assert_eq!(config.get_model_for_task(TaskType::Simple), "gpt-4o-mini");
        assert_eq!(config.get_model_for_task(TaskType::Complex), "gpt-4o");
        assert_eq!(config.get_model_for_task(TaskType::Code), "gpt-4o");
    }

    #[test]


    fn test_get_fallback_model() {
        let config = ModelRoutingConfig::default();
        assert_eq!(config.get_fallback_model("gpt-4o"), Some("gpt-4o-mini"));
        assert_eq!(
            config.get_fallback_model("gpt-4o-mini"),
            Some("gpt-3.5-turbo")
        );
        assert_eq!(config.get_fallback_model("gpt-3.5-turbo"), None);
        assert_eq!(config.get_fallback_model("unknown-model"), None);
    }

    #[test]


    fn test_get_fallback_model_disabled() {
        let config = ModelRoutingConfig {
            fallback_enabled: false,
            ..Default::default()
        };
        assert_eq!(config.get_fallback_model("gpt-4o"), None);
    }

    #[test]


    fn test_get_all_models() {
        let config = ModelRoutingConfig::default();
        let models = config.get_all_models();
        assert!(models.contains(&"gpt-4o"));
        assert!(models.contains(&"gpt-4o-mini"));
        assert!(models.contains(&"gpt-3.5-turbo"));
    }

    #[test]


    fn test_routing_strategy_display() {
        assert_eq!(format!("{}", RoutingStrategy::Default), "default");
        assert_eq!(format!("{}", RoutingStrategy::TaskBased), "task-based");
        assert_eq!(format!("{}", RoutingStrategy::RoundRobin), "round-robin");
    }