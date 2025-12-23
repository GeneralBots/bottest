//! Unit tests migrated from src/timeseries/mod.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_metric_point_line_protocol() {
        let point = MetricPoint::new("test_measurement")
            .tag("host", "server01")
            .tag("region", "us-west")
            .field_f64("temperature", 23.5)
            .field_i64("humidity", 45);

        let line = point.to_line_protocol();
        assert!(line.starts_with("test_measurement,"));
        assert!(line.contains("host=server01"));
        assert!(line.contains("region=us-west"));
        assert!(line.contains("temperature=23.5"));
        assert!(line.contains("humidity=45i"));
    }

    #[test]

    
    fn test_metric_point_escaping() {
        let point = MetricPoint::new("test")
            .tag("key with space", "value,with=special")
            .field_str("message", "Hello \"world\"");

        let line = point.to_line_protocol();
        assert!(line.contains("key\\ with\\ space=value\\,with\\=special"));
        assert!(line.contains("message=\"Hello \\\"world\\\"\""));
    }

    #[test]

    
    fn test_predefined_metrics() {
        let msg = Metrics::message("bot-1", "whatsapp", "incoming");
        assert_eq!(msg.measurement, "messages");
        assert_eq!(msg.tags.get("channel"), Some(&"whatsapp".to_string()));

        let resp = Metrics::response_time("bot-1", 150.5);
        assert_eq!(resp.measurement, "response_time");

        let tokens = Metrics::llm_tokens("bot-1", "gpt-4", 100, 50);
        assert_eq!(tokens.measurement, "llm_tokens");
    }