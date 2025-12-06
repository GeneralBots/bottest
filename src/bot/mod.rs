//! Bot conversation testing module
//!
//! Provides tools for simulating and testing bot conversations
//! including message exchanges, flow validation, and response assertions.

mod conversation;
mod runner;

use crate::fixtures::MessageDirection;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::Duration;
use uuid::Uuid;

/// Response from the bot
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotResponse {
    pub id: Uuid,
    pub content: String,
    pub content_type: ResponseContentType,
    pub metadata: HashMap<String, serde_json::Value>,
    pub latency_ms: u64,
}

/// Type of response content
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ResponseContentType {
    Text,
    Image,
    Audio,
    Video,
    Document,
    Interactive,
    Template,
    Location,
    Contact,
}

impl Default for ResponseContentType {
    fn default() -> Self {
        Self::Text
    }
}

/// Assertion result for conversation tests
#[derive(Debug, Clone)]
pub struct AssertionResult {
    pub passed: bool,
    pub message: String,
    pub expected: Option<String>,
    pub actual: Option<String>,
}

impl AssertionResult {
    pub fn pass(message: &str) -> Self {
        Self {
            passed: true,
            message: message.to_string(),
            expected: None,
            actual: None,
        }
    }

    pub fn fail(message: &str, expected: &str, actual: &str) -> Self {
        Self {
            passed: false,
            message: message.to_string(),
            expected: Some(expected.to_string()),
            actual: Some(actual.to_string()),
        }
    }
}

/// Configuration for conversation tests
#[derive(Debug, Clone)]
pub struct ConversationConfig {
    /// Maximum time to wait for a response
    pub response_timeout: Duration,
    /// Whether to record the conversation for later analysis
    pub record: bool,
    /// Whether to use the mock LLM
    pub use_mock_llm: bool,
    /// Custom variables to inject into the conversation
    pub variables: HashMap<String, String>,
}

impl Default for ConversationConfig {
    fn default() -> Self {
        Self {
            response_timeout: Duration::from_secs(30),
            record: true,
            use_mock_llm: true,
            variables: HashMap::new(),
        }
    }
}

/// Recorded conversation for analysis
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConversationRecord {
    pub id: Uuid,
    pub bot_name: String,
    pub started_at: chrono::DateTime<chrono::Utc>,
    pub ended_at: Option<chrono::DateTime<chrono::Utc>>,
    pub messages: Vec<RecordedMessage>,
    pub assertions: Vec<AssertionRecord>,
    pub passed: bool,
}

/// Recorded message in a conversation
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecordedMessage {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub direction: MessageDirection,
    pub content: String,
    pub latency_ms: Option<u64>,
}

/// Recorded assertion
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AssertionRecord {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub assertion_type: String,
    pub passed: bool,
    pub message: String,
}

/// State of a conversation flow
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConversationState {
    /// Initial state, conversation not started
    Initial,
    /// Waiting for user input
    WaitingForUser,
    /// Waiting for bot response
    WaitingForBot,
    /// Conversation transferred to human
    Transferred,
    /// Conversation ended normally
    Ended,
    /// Conversation ended with error
    Error,
}

impl Default for ConversationState {
    fn default() -> Self {
        Self::Initial
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assertion_result_pass() {
        let result = AssertionResult::pass("Test passed");
        assert!(result.passed);
        assert_eq!(result.message, "Test passed");
    }

    #[test]
    fn test_assertion_result_fail() {
        let result = AssertionResult::fail("Test failed", "expected", "actual");
        assert!(!result.passed);
        assert_eq!(result.expected, Some("expected".to_string()));
        assert_eq!(result.actual, Some("actual".to_string()));
    }

    #[test]
    fn test_conversation_config_default() {
        let config = ConversationConfig::default();
        assert_eq!(config.response_timeout, Duration::from_secs(30));
        assert!(config.record);
        assert!(config.use_mock_llm);
    }

    #[test]
    fn test_conversation_state_default() {
        let state = ConversationState::default();
        assert_eq!(state, ConversationState::Initial);
    }

    #[test]
    fn test_bot_response_serialization() {
        let response = BotResponse {
            id: Uuid::new_v4(),
            content: "Hello!".to_string(),
            content_type: ResponseContentType::Text,
            metadata: HashMap::new(),
            latency_ms: 150,
        };

        let json = serde_json::to_string(&response).unwrap();
        assert!(json.contains("Hello!"));
        assert!(json.contains("text"));
    }
}
