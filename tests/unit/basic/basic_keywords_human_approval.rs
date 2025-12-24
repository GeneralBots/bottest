use botserver::basic::keywords::human_approval::{
    ApprovalChannel, ApprovalConfig, ApprovalDecision, ApprovalManager, ApprovalStatus,
};
use chrono::{Duration, Utc};
use rhai::Map;
use uuid::Uuid;

#[test]
fn test_default_config() {
    let config = ApprovalConfig::default();
    assert!(config.enabled);
    assert_eq!(config.default_timeout, 3600);
    assert_eq!(config.max_reminders, 3);
}

#[test]
fn test_create_request() {
    let manager = ApprovalManager::new(ApprovalConfig::default());
    let request = manager.create_request(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        "expense_approval",
        ApprovalChannel::Email,
        "manager@example.com",
        serde_json::json!({"amount": 1000}),
        "Please approve expense",
        None,
        None,
    );

    assert_eq!(request.status, ApprovalStatus::Pending);
    assert_eq!(request.approval_type, "expense_approval");
    assert!(request.expires_at > Utc::now());
}

#[test]
fn test_is_expired() {
    let manager = ApprovalManager::new(ApprovalConfig::default());
    let mut request = manager.create_request(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        "test",
        ApprovalChannel::Email,
        "test@example.com",
        serde_json::json!({}),
        "Test",
        Some(1),
        None,
    );

    assert!(!manager.is_expired(&request));

    request.expires_at = Utc::now() - Duration::seconds(10);
    assert!(manager.is_expired(&request));
}

#[test]
fn test_process_decision() {
    let manager = ApprovalManager::new(ApprovalConfig::default());
    let mut request = manager.create_request(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        "test",
        ApprovalChannel::Email,
        "test@example.com",
        serde_json::json!({}),
        "Test",
        None,
        None,
    );

    manager.process_decision(
        &mut request,
        ApprovalDecision::Approve,
        "manager@example.com",
        Some("Looks good!".to_string()),
    );

    assert_eq!(request.status, ApprovalStatus::Approved);
    assert_eq!(request.decision, Some(ApprovalDecision::Approve));
    assert_eq!(request.decided_by, Some("manager@example.com".to_string()));
    assert_eq!(request.comments, Some("Looks good!".to_string()));
}

#[test]
fn test_evaluate_condition() {
    let manager = ApprovalManager::new(ApprovalConfig::default());
    let context = serde_json::json!({
        "amount": 15000,
        "priority": 2
    });

    assert!(manager
        .evaluate_condition("amount > 10000", &context)
        .unwrap());
    assert!(!manager
        .evaluate_condition("amount > 20000", &context)
        .unwrap());
    assert!(manager
        .evaluate_condition("priority == 2", &context)
        .unwrap());
}

#[test]
fn test_handle_timeout_with_default() {
    let manager = ApprovalManager::new(ApprovalConfig::default());
    let mut request = manager.create_request(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        "test",
        ApprovalChannel::Email,
        "test@example.com",
        serde_json::json!({}),
        "Test",
        None,
        Some(ApprovalDecision::Approve),
    );

    manager.handle_timeout(&mut request);

    assert_eq!(request.status, ApprovalStatus::Approved);
    assert_eq!(request.decision, Some(ApprovalDecision::Approve));
    assert_eq!(request.decided_by, Some("system:timeout".to_string()));
}

#[test]
fn test_request_to_dynamic() {
    let manager = ApprovalManager::new(ApprovalConfig::default());
    let request = manager.create_request(
        Uuid::new_v4(),
        Uuid::new_v4(),
        Uuid::new_v4(),
        "test",
        ApprovalChannel::Email,
        "test@example.com",
        serde_json::json!({"key": "value"}),
        "Test message",
        None,
        None,
    );

    let dynamic = request.to_dynamic();
    assert!(dynamic.is::<Map>());
}
