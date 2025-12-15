//! Web E2E testing module
//!
//! Provides tools for browser-based end-to-end testing using WebDriver
//! (via fantoccini) to automate browser interactions with the chat interface.

pub mod browser;
pub mod pages;

pub use browser::{Browser, BrowserConfig, BrowserType};

use serde::{Deserialize, Serialize};
use std::time::Duration;

/// Configuration for E2E tests
#[derive(Debug, Clone)]
pub struct E2EConfig {
    /// Browser type to use
    pub browser: BrowserType,
    /// Whether to run headless
    pub headless: bool,
    /// Default timeout for operations
    pub timeout: Duration,
    /// Window width
    pub window_width: u32,
    /// Window height
    pub window_height: u32,
    /// WebDriver URL
    pub webdriver_url: String,
    /// Whether to capture screenshots on failure
    pub screenshot_on_failure: bool,
    /// Directory to save screenshots
    pub screenshot_dir: String,
}

impl Default for E2EConfig {
    fn default() -> Self {
        Self {
            browser: BrowserType::Chrome,
            headless: std::env::var("HEADED").is_err(),
            timeout: Duration::from_secs(30),
            window_width: 1920,
            window_height: 1080,
            webdriver_url: "http://localhost:4444".to_string(),
            screenshot_on_failure: true,
            screenshot_dir: "./test-screenshots".to_string(),
        }
    }
}

impl E2EConfig {
    /// Create a BrowserConfig from this E2EConfig
    pub fn to_browser_config(&self) -> BrowserConfig {
        BrowserConfig::default()
            .with_browser(self.browser)
            .with_webdriver_url(&self.webdriver_url)
            .headless(self.headless)
            .with_window_size(self.window_width, self.window_height)
            .with_timeout(self.timeout)
    }
}

/// Result of an E2E test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct E2ETestResult {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub steps: Vec<TestStep>,
    pub screenshots: Vec<String>,
    pub error: Option<String>,
}

/// A step in an E2E test
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestStep {
    pub name: String,
    pub passed: bool,
    pub duration_ms: u64,
    pub error: Option<String>,
}

/// Element locator strategies
#[derive(Debug, Clone)]
pub enum Locator {
    /// CSS selector
    Css(String),
    /// XPath expression
    XPath(String),
    /// Element ID
    Id(String),
    /// Element name attribute
    Name(String),
    /// Link text
    LinkText(String),
    /// Partial link text
    PartialLinkText(String),
    /// Tag name
    TagName(String),
    /// Class name
    ClassName(String),
}

impl Locator {
    pub fn css(selector: &str) -> Self {
        Self::Css(selector.to_string())
    }

    pub fn xpath(expr: &str) -> Self {
        Self::XPath(expr.to_string())
    }

    pub fn id(id: &str) -> Self {
        Self::Id(id.to_string())
    }

    pub fn name(name: &str) -> Self {
        Self::Name(name.to_string())
    }

    pub fn link_text(text: &str) -> Self {
        Self::LinkText(text.to_string())
    }

    pub fn class(name: &str) -> Self {
        Self::ClassName(name.to_string())
    }
    
    /// Convert locator to CSS selector string for CDP
    pub fn to_css_selector(&self) -> String {
        match self {
            Locator::Css(s) => s.clone(),
            Locator::XPath(_) => {
                // XPath not directly supported in CSS - log warning and return generic
                log::warn!("XPath locators not directly supported in CDP, use CSS selectors");
                "*".to_string()
            }
            Locator::Id(s) => format!("#{}", s),
            Locator::Name(s) => format!("[name='{}']", s),
            Locator::LinkText(s) => format!("a:contains('{}')", s),
            Locator::PartialLinkText(s) => format!("a[href*='{}']", s),
            Locator::TagName(s) => s.clone(),
            Locator::ClassName(s) => format!(".{}", s),
        }
    }
}

/// Keyboard keys for special key presses
#[derive(Debug, Clone, Copy)]
pub enum Key {
    Enter,
    Tab,
    Escape,
    Backspace,
    Delete,
    ArrowUp,
    ArrowDown,
    ArrowLeft,
    ArrowRight,
    Home,
    End,
    PageUp,
    PageDown,
    F1,
    F2,
    F3,
    F4,
    F5,
    F6,
    F7,
    F8,
    F9,
    F10,
    F11,
    F12,
    Shift,
    Control,
    Alt,
    Meta,
}

/// Mouse button
#[derive(Debug, Clone, Copy)]
pub enum MouseButton {
    Left,
    Right,
    Middle,
}

/// Wait condition for elements
#[derive(Debug, Clone)]
pub enum WaitCondition {
    /// Element is present in DOM
    Present,
    /// Element is visible
    Visible,
    /// Element is clickable
    Clickable,
    /// Element is not present
    NotPresent,
    /// Element is not visible
    NotVisible,
    /// Element contains text
    ContainsText(String),
    /// Element has attribute value
    HasAttribute(String, String),
    /// Custom JavaScript condition
    Script(String),
}

/// Action chain for complex interactions
pub struct ActionChain {
    actions: Vec<Action>,
}

/// Individual action in a chain
#[derive(Debug, Clone)]
pub enum Action {
    Click(Locator),
    DoubleClick(Locator),
    RightClick(Locator),
    MoveTo(Locator),
    MoveByOffset(i32, i32),
    KeyDown(Key),
    KeyUp(Key),
    SendKeys(String),
    Pause(Duration),
    DragAndDrop(Locator, Locator),
    ScrollTo(Locator),
    ScrollByAmount(i32, i32),
}

impl ActionChain {
    /// Create a new action chain
    pub fn new() -> Self {
        Self {
            actions: Vec::new(),
        }
    }

    /// Add a click action
    pub fn click(mut self, locator: Locator) -> Self {
        self.actions.push(Action::Click(locator));
        self
    }

    /// Add a double click action
    pub fn double_click(mut self, locator: Locator) -> Self {
        self.actions.push(Action::DoubleClick(locator));
        self
    }

    /// Add a right click action
    pub fn right_click(mut self, locator: Locator) -> Self {
        self.actions.push(Action::RightClick(locator));
        self
    }

    /// Move to an element
    pub fn move_to(mut self, locator: Locator) -> Self {
        self.actions.push(Action::MoveTo(locator));
        self
    }

    /// Move by offset
    pub fn move_by(mut self, x: i32, y: i32) -> Self {
        self.actions.push(Action::MoveByOffset(x, y));
        self
    }

    /// Press a key down
    pub fn key_down(mut self, key: Key) -> Self {
        self.actions.push(Action::KeyDown(key));
        self
    }

    /// Release a key
    pub fn key_up(mut self, key: Key) -> Self {
        self.actions.push(Action::KeyUp(key));
        self
    }

    /// Send keys (type text)
    pub fn send_keys(mut self, text: &str) -> Self {
        self.actions.push(Action::SendKeys(text.to_string()));
        self
    }

    /// Pause for a duration
    pub fn pause(mut self, duration: Duration) -> Self {
        self.actions.push(Action::Pause(duration));
        self
    }

    /// Drag and drop
    pub fn drag_and_drop(mut self, source: Locator, target: Locator) -> Self {
        self.actions.push(Action::DragAndDrop(source, target));
        self
    }

    /// Scroll to element
    pub fn scroll_to(mut self, locator: Locator) -> Self {
        self.actions.push(Action::ScrollTo(locator));
        self
    }

    /// Scroll by amount
    pub fn scroll_by(mut self, x: i32, y: i32) -> Self {
        self.actions.push(Action::ScrollByAmount(x, y));
        self
    }

    /// Get the actions
    pub fn actions(&self) -> &[Action] {
        &self.actions
    }
}

impl Default for ActionChain {
    fn default() -> Self {
        Self::new()
    }
}

/// Cookie data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Cookie {
    pub name: String,
    pub value: String,
    pub domain: Option<String>,
    pub path: Option<String>,
    pub secure: Option<bool>,
    pub http_only: Option<bool>,
    pub same_site: Option<String>,
    pub expiry: Option<u64>,
}

impl Cookie {
    pub fn new(name: &str, value: &str) -> Self {
        Self {
            name: name.to_string(),
            value: value.to_string(),
            domain: None,
            path: None,
            secure: None,
            http_only: None,
            same_site: None,
            expiry: None,
        }
    }

    pub fn with_domain(mut self, domain: &str) -> Self {
        self.domain = Some(domain.to_string());
        self
    }

    pub fn with_path(mut self, path: &str) -> Self {
        self.path = Some(path.to_string());
        self
    }

    pub fn secure(mut self) -> Self {
        self.secure = Some(true);
        self
    }

    pub fn http_only(mut self) -> Self {
        self.http_only = Some(true);
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_e2e_config_default() {
        let config = E2EConfig::default();
        assert_eq!(config.window_width, 1920);
        assert_eq!(config.window_height, 1080);
        assert!(config.screenshot_on_failure);
    }

    #[test]
    fn test_e2e_config_to_browser_config() {
        let e2e_config = E2EConfig::default();
        let browser_config = e2e_config.to_browser_config();
        assert_eq!(browser_config.browser_type, BrowserType::Chrome);
        assert_eq!(browser_config.window_width, 1920);
    }

    #[test]
    fn test_locator_constructors() {
        let css = Locator::css(".my-class");
        assert!(matches!(css, Locator::Css(_)));

        let xpath = Locator::xpath("//div[@id='test']");
        assert!(matches!(xpath, Locator::XPath(_)));

        let id = Locator::id("my-id");
        assert!(matches!(id, Locator::Id(_)));
    }

    #[test]
    fn test_action_chain() {
        let chain = ActionChain::new()
            .click(Locator::id("button"))
            .send_keys("Hello")
            .pause(Duration::from_millis(500))
            .key_down(Key::Enter);

        assert_eq!(chain.actions().len(), 4);
    }

    #[test]
    fn test_cookie_builder() {
        let cookie = Cookie::new("session", "abc123")
            .with_domain("example.com")
            .with_path("/")
            .secure()
            .http_only();

        assert_eq!(cookie.name, "session");
        assert_eq!(cookie.value, "abc123");
        assert_eq!(cookie.domain, Some("example.com".to_string()));
        assert!(cookie.secure.unwrap());
        assert!(cookie.http_only.unwrap());
    }

    #[test]
    fn test_e2e_test_result() {
        let result = E2ETestResult {
            name: "Test login flow".to_string(),
            passed: true,
            duration_ms: 5000,
            steps: vec![
                TestStep {
                    name: "Navigate to login".to_string(),
                    passed: true,
                    duration_ms: 1000,
                    error: None,
                },
                TestStep {
                    name: "Enter credentials".to_string(),
                    passed: true,
                    duration_ms: 2000,
                    error: None,
                },
            ],
            screenshots: vec![],
            error: None,
        };

        assert!(result.passed);
        assert_eq!(result.steps.len(), 2);
    }
}
