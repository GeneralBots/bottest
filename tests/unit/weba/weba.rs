//! Unit tests for botserver weba module
//! Tests web application builder functionality

use botserver::weba::{
    slugify, WebApp, WebAppConfig, WebAppStatus, WebAppTemplate, WebaState,
};
use uuid::Uuid;

#[test]
fn test_slugify() {
    assert_eq!(slugify("Hello World"), "hello-world");
    assert_eq!(slugify("My App 123"), "my-app-123");
    assert_eq!(slugify("  Test  App  "), "test-app");
}

#[test]
fn test_webapp_creation() {
    let now = chrono::Utc::now();
    let app = WebApp {
        id: Uuid::new_v4(),
        name: "Test App".to_string(),
        slug: "test-app".to_string(),
        description: None,
        template: WebAppTemplate::Blank,
        status: WebAppStatus::Draft,
        config: WebAppConfig::default(),
        created_at: now,
        updated_at: now,
    };
    assert_eq!(app.name, "Test App");
    assert_eq!(app.slug, "test-app");
}

#[tokio::test]
async fn test_weba_state() {
    let state = WebaState::new();
    let apps = state.apps.read().await;
    assert!(apps.is_empty());
}
