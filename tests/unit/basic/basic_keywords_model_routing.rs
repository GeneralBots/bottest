


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_model_router_new() {
        let router = ModelRouter::new();
        assert_eq!(router.default_model, "default");
        assert!(router.models.is_empty());
        assert_eq!(router.routing_strategy, RoutingStrategy::Manual);
    }

    #[test]


    fn test_auto_routing_code() {
        let mut router = ModelRouter::new();
        router.models.insert(
            "code".to_string(),
            ModelConfig {
                name: "code".to_string(),
                url: "http://localhost:8081".to_string(),
                model_path: "codellama.gguf".to_string(),
                api_key: None,
                max_tokens: None,
                temperature: None,
            },
        );
        router.routing_strategy = RoutingStrategy::Auto;

        let result = router.route_query("Help me debug this code");
        assert_eq!(result, "code");
    }

    #[test]


    fn test_auto_routing_quality() {
        let mut router = ModelRouter::new();
        router.models.insert(
            "quality".to_string(),
            ModelConfig {
                name: "quality".to_string(),
                url: "http://localhost:8081".to_string(),
                model_path: "large-model.gguf".to_string(),
                api_key: None,
                max_tokens: None,
                temperature: None,
            },
        );
        router.routing_strategy = RoutingStrategy::Auto;

        let result =
            router.route_query("Please analyze and compare these two approaches in detail");
        assert_eq!(result, "quality");
    }

    #[test]


    fn test_auto_routing_fast() {
        let mut router = ModelRouter::new();
        router.models.insert(
            "fast".to_string(),
            ModelConfig {
                name: "fast".to_string(),
                url: "http://localhost:8081".to_string(),
                model_path: "small-model.gguf".to_string(),
                api_key: None,
                max_tokens: None,
                temperature: None,
            },
        );
        router.routing_strategy = RoutingStrategy::Auto;

        let result = router.route_query("What is AI?");
        assert_eq!(result, "fast");
    }

    #[test]


    fn test_routing_strategy_default() {
        let strategy = RoutingStrategy::default();
        assert_eq!(strategy, RoutingStrategy::Manual);
    }