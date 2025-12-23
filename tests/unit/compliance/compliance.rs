//! Unit tests migrated from src/compliance/mod.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[tokio::test]
    async fn test_compliance_monitor() {
        let monitor = ComplianceMonitor::new(vec![ComplianceFramework::GDPR]);
        let results = monitor.run_checks().await.unwrap();
        assert!(!results.is_empty());
    }

    #[test]

    
    fn test_compliance_score() {
        let results = vec![
            ComplianceCheckResult {
                framework: ComplianceFramework::GDPR,
                control_id: "test_1".to_string(),
                control_name: "Test Control 1".to_string(),
                status: ComplianceStatus::Compliant,
                score: 100.0,
                checked_at: Utc::now(),
                issues: vec![],
                evidence: vec![],
            },
            ComplianceCheckResult {
                framework: ComplianceFramework::GDPR,
                control_id: "test_2".to_string(),
                control_name: "Test Control 2".to_string(),
                status: ComplianceStatus::Compliant,
                score: 90.0,
                checked_at: Utc::now(),
                issues: vec![],
                evidence: vec![],
            },
        ];

        let score = ComplianceMonitor::calculate_compliance_score(&results);
        assert_eq!(score, 95.0);
    }