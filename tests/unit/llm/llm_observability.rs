


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_default_config() {
        let config = ObservabilityConfig::default();
        assert!(config.enabled);
        assert!(config.cost_tracking);
        assert_eq!(config.budget_daily, 100.0);
    }

    #[test]


    fn test_calculate_cost() {
        let manager = ObservabilityManager::new(ObservabilityConfig::default());


        let cost = manager.calculate_cost("gpt-4", 1000, 500);
        assert!(cost > 0.0);


        let cost = manager.calculate_cost("local", 1000, 500);
        assert_eq!(cost, 0.0);


        let cost = manager.calculate_cost("unknown-model", 1000, 500);
        assert_eq!(cost, 0.0);
    }

    #[test]


    fn test_quick_stats() {
        let manager = ObservabilityManager::new(ObservabilityConfig::default());
        let stats = manager.get_quick_stats();

        assert_eq!(stats.total_requests, 0);
        assert_eq!(stats.total_tokens, 0);
        assert_eq!(stats.cache_hit_rate, 0.0);
    }

    #[test]


    fn test_start_span() {
        let manager = ObservabilityManager::new(ObservabilityConfig::default());
        let trace_id = Uuid::new_v4();
        let span = manager.start_span(trace_id, "test_operation", "test_component", None);

        assert_eq!(span.name, "test_operation");
        assert_eq!(span.component, "test_component");
        assert_eq!(span.status, TraceStatus::InProgress);
        assert!(span.duration_ms.is_none());
    }

    #[test]


    fn test_end_span() {
        let manager = ObservabilityManager::new(ObservabilityConfig::default());
        let trace_id = Uuid::new_v4();
        let mut span = manager.start_span(trace_id, "test_operation", "test_component", None);


        std::thread::sleep(std::time::Duration::from_millis(10));

        manager.end_span(&mut span, TraceStatus::Ok, None);

        assert_eq!(span.status, TraceStatus::Ok);
        assert!(span.duration_ms.is_some());
        assert!(span.end_time.is_some());
    }

    #[tokio::test]
    async fn test_budget_status() {
        let manager = ObservabilityManager::new(ObservabilityConfig::default());
        let status = manager.get_budget_status().await;

        assert_eq!(status.daily_limit, 100.0);
        assert_eq!(status.daily_spend, 0.0);
        assert!(!status.daily_exceeded);
    }

    #[tokio::test]
    async fn test_budget_check() {
        let manager = ObservabilityManager::new(ObservabilityConfig::default());


        let result = manager.check_budget(1.0).await;
        assert_eq!(result, BudgetCheckResult::Ok);


        let result = manager.check_budget(150.0).await;
        assert_eq!(result, BudgetCheckResult::DailyExceeded);
    }

    #[test]


    fn test_metrics_to_dynamic() {
        let metrics = LLMRequestMetrics {
            request_id: Uuid::new_v4(),
            session_id: Uuid::new_v4(),
            bot_id: Uuid::new_v4(),
            model: "gpt-4".to_string(),
            request_type: RequestType::Chat,
            input_tokens: 100,
            output_tokens: 50,
            total_tokens: 150,
            latency_ms: 500,
            ttft_ms: Some(100),
            cached: false,
            success: true,
            error: None,
            estimated_cost: 0.01,
            timestamp: Utc::now(),
            metadata: HashMap::new(),
        };

        let dynamic = metrics.to_dynamic();
        assert!(dynamic.is::<Map>());
    }