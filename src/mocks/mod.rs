//! Mock servers for testing external service integrations
//!
//! Provides mock implementations of:
//! - LLM API (OpenAI-compatible)
//! - WhatsApp Business API
//! - Microsoft Teams Bot Framework
//! - Zitadel Auth/OIDC

mod llm;
mod teams;
mod whatsapp;
mod zitadel;

pub use llm::MockLLM;
pub use teams::MockTeams;
pub use whatsapp::MockWhatsApp;
pub use zitadel::MockZitadel;

use anyhow::Result;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

/// Registry of all mock servers for a test
pub struct MockRegistry {
    pub llm: Option<MockLLM>,
    pub whatsapp: Option<MockWhatsApp>,
    pub teams: Option<MockTeams>,
    pub zitadel: Option<MockZitadel>,
}

impl MockRegistry {
    /// Create an empty registry
    pub fn new() -> Self {
        Self {
            llm: None,
            whatsapp: None,
            teams: None,
            zitadel: None,
        }
    }

    /// Get the LLM mock, panics if not configured
    pub fn llm(&self) -> &MockLLM {
        self.llm.as_ref().expect("LLM mock not configured")
    }

    /// Get the WhatsApp mock, panics if not configured
    pub fn whatsapp(&self) -> &MockWhatsApp {
        self.whatsapp.as_ref().expect("WhatsApp mock not configured")
    }

    /// Get the Teams mock, panics if not configured
    pub fn teams(&self) -> &MockTeams {
        self.teams.as_ref().expect("Teams mock not configured")
    }

    /// Get the Zitadel mock, panics if not configured
    pub fn zitadel(&self) -> &MockZitadel {
        self.zitadel.as_ref().expect("Zitadel mock not configured")
    }

    /// Verify all mock expectations were met
    pub fn verify_all(&self) -> Result<()> {
        if let Some(ref llm) = self.llm {
            llm.verify()?;
        }
        if let Some(ref whatsapp) = self.whatsapp {
            whatsapp.verify()?;
        }
        if let Some(ref teams) = self.teams {
            teams.verify()?;
        }
        if let Some(ref zitadel) = self.zitadel {
            zitadel.verify()?;
        }
        Ok(())
    }

    /// Reset all mock servers
    pub async fn reset_all(&self) {
        if let Some(ref llm) = self.llm {
            llm.reset().await;
        }
        if let Some(ref whatsapp) = self.whatsapp {
            whatsapp.reset().await;
        }
        if let Some(ref teams) = self.teams {
            teams.reset().await;
        }
        if let Some(ref zitadel) = self.zitadel {
            zitadel.reset().await;
        }
    }
}

impl Default for MockRegistry {
    fn default() -> Self {
        Self::new()
    }
}

/// Expectation tracking for mock verification
#[derive(Debug, Clone)]
pub struct Expectation {
    pub name: String,
    pub expected_calls: Option<usize>,
    pub actual_calls: usize,
    pub matched: bool,
}

impl Expectation {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            expected_calls: None,
            actual_calls: 0,
            matched: false,
        }
    }

    pub fn times(mut self, n: usize) -> Self {
        self.expected_calls = Some(n);
        self
    }

    pub fn record_call(&mut self) {
        self.actual_calls += 1;
        self.matched = true;
    }

    pub fn verify(&self) -> Result<()> {
        if let Some(expected) = self.expected_calls {
            if self.actual_calls != expected {
                anyhow::bail!(
                    "Expectation '{}' expected {} calls but got {}",
                    self.name,
                    expected,
                    self.actual_calls
                );
            }
        }
        Ok(())
    }
}

/// Shared state for tracking expectations across async handlers
pub type ExpectationStore = Arc<Mutex<HashMap<String, Expectation>>>;

/// Create a new expectation store
pub fn new_expectation_store() -> ExpectationStore {
    Arc::new(Mutex::new(HashMap::new()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expectation_basic() {
        let mut exp = Expectation::new("test");
        assert_eq!(exp.actual_calls, 0);
        assert!(!exp.matched);

        exp.record_call();
        assert_eq!(exp.actual_calls, 1);
        assert!(exp.matched);
    }

    #[test]
    fn test_expectation_times() {
        let mut exp = Expectation::new("test").times(2);
        exp.record_call();
        exp.record_call();

        assert!(exp.verify().is_ok());
    }

    #[test]
    fn test_expectation_times_fail() {
        let mut exp = Expectation::new("test").times(2);
        exp.record_call();

        assert!(exp.verify().is_err());
    }

    #[test]
    fn test_mock_registry_default() {
        let registry = MockRegistry::new();
        assert!(registry.llm.is_none());
        assert!(registry.whatsapp.is_none());
        assert!(registry.teams.is_none());
        assert!(registry.zitadel.is_none());
    }
}
