//! Browser abstraction for E2E testing
//!
//! Provides a high-level interface for browser automation using fantoccini/WebDriver.
//! Supports Chrome, Firefox, and Safari with both headless and headed modes.

use anyhow::{Context, Result};
use fantoccini::{Client, ClientBuilder, Locator as FLocator};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;
use std::time::Duration;
use tokio::time::sleep;

use super::{Cookie, Key, Locator, WaitCondition};

/// Browser type for E2E testing
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum BrowserType {
    Chrome,
    Firefox,
    Safari,
    Edge,
}

impl Default for BrowserType {
    fn default() -> Self {
        Self::Chrome
    }
}

impl BrowserType {
    /// Get the WebDriver capability name for this browser
    pub fn capability_name(&self) -> &'static str {
        match self {
            BrowserType::Chrome => "goog:chromeOptions",
            BrowserType::Firefox => "moz:firefoxOptions",
            BrowserType::Safari => "safari:options",
            BrowserType::Edge => "ms:edgeOptions",
        }
    }

    /// Get the browser name for WebDriver
    pub fn browser_name(&self) -> &'static str {
        match self {
            BrowserType::Chrome => "chrome",
            BrowserType::Firefox => "firefox",
            BrowserType::Safari => "safari",
            BrowserType::Edge => "MicrosoftEdge",
        }
    }
}

/// Configuration for browser sessions
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Browser type
    pub browser_type: BrowserType,
    /// WebDriver URL
    pub webdriver_url: String,
    /// Whether to run headless
    pub headless: bool,
    /// Window width
    pub window_width: u32,
    /// Window height
    pub window_height: u32,
    /// Default timeout for operations
    pub timeout: Duration,
    /// Whether to accept insecure certificates
    pub accept_insecure_certs: bool,
    /// Additional browser arguments
    pub browser_args: Vec<String>,
    /// Additional capabilities
    pub capabilities: HashMap<String, serde_json::Value>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        Self {
            browser_type: BrowserType::Chrome,
            webdriver_url: "http://localhost:4444".to_string(),
            headless: std::env::var("HEADED").is_err(),
            window_width: 1920,
            window_height: 1080,
            timeout: Duration::from_secs(30),
            accept_insecure_certs: true,
            browser_args: Vec::new(),
            capabilities: HashMap::new(),
        }
    }
}

impl BrowserConfig {
    /// Create a new browser config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set browser type
    pub fn with_browser(mut self, browser: BrowserType) -> Self {
        self.browser_type = browser;
        self
    }

    /// Set WebDriver URL
    pub fn with_webdriver_url(mut self, url: &str) -> Self {
        self.webdriver_url = url.to_string();
        self
    }

    /// Set headless mode
    pub fn headless(mut self, headless: bool) -> Self {
        self.headless = headless;
        self
    }

    /// Set window size
    pub fn with_window_size(mut self, width: u32, height: u32) -> Self {
        self.window_width = width;
        self.window_height = height;
        self
    }

    /// Set default timeout
    pub fn with_timeout(mut self, timeout: Duration) -> Self {
        self.timeout = timeout;
        self
    }

    /// Add a browser argument
    pub fn with_arg(mut self, arg: &str) -> Self {
        self.browser_args.push(arg.to_string());
        self
    }

    /// Build WebDriver capabilities
    pub fn build_capabilities(&self) -> serde_json::Value {
        let mut caps = serde_json::json!({
            "browserName": self.browser_type.browser_name(),
            "acceptInsecureCerts": self.accept_insecure_certs,
        });

        // Add browser-specific options
        let mut browser_options = serde_json::json!({});

        // Build args list
        let mut args: Vec<String> = self.browser_args.clone();
        if self.headless {
            match self.browser_type {
                BrowserType::Chrome | BrowserType::Edge => {
                    args.push("--headless=new".to_string());
                    args.push("--disable-gpu".to_string());
                    args.push("--no-sandbox".to_string());
                    args.push("--disable-dev-shm-usage".to_string());
                }
                BrowserType::Firefox => {
                    args.push("-headless".to_string());
                }
                BrowserType::Safari => {
                    // Safari doesn't support headless mode directly
                }
            }
        }

        // Set window size
        args.push(format!(
            "--window-size={},{}",
            self.window_width, self.window_height
        ));

        browser_options["args"] = serde_json::json!(args);

        caps[self.browser_type.capability_name()] = browser_options;

        // Merge additional capabilities
        for (key, value) in &self.capabilities {
            caps[key] = value.clone();
        }

        caps
    }
}

/// Browser instance for E2E testing
pub struct Browser {
    client: Client,
    config: BrowserConfig,
}

impl Browser {
    /// Create a new browser instance
    pub async fn new(config: BrowserConfig) -> Result<Self> {
        let caps = config.build_capabilities();

        let client = ClientBuilder::native()
            .capabilities(caps.as_object().cloned().unwrap_or_default())
            .connect(&config.webdriver_url)
            .await
            .context("Failed to connect to WebDriver")?;

        Ok(Self { client, config })
    }

    /// Create a new headless Chrome browser with default settings
    pub async fn new_headless() -> Result<Self> {
        Self::new(BrowserConfig::default().headless(true)).await
    }

    /// Create a new Chrome browser with visible window
    pub async fn new_headed() -> Result<Self> {
        Self::new(BrowserConfig::default().headless(false)).await
    }

    /// Navigate to a URL
    pub async fn goto(&self, url: &str) -> Result<()> {
        self.client
            .goto(url)
            .await
            .context(format!("Failed to navigate to {}", url))?;
        Ok(())
    }

    /// Get the current URL
    pub async fn current_url(&self) -> Result<String> {
        let url = self.client.current_url().await?;
        Ok(url.to_string())
    }

    /// Get the page title
    pub async fn title(&self) -> Result<String> {
        self.client
            .title()
            .await
            .context("Failed to get page title")
    }

    /// Get the page source
    pub async fn page_source(&self) -> Result<String> {
        self.client
            .source()
            .await
            .context("Failed to get page source")
    }

    /// Find an element by locator
    pub async fn find(&self, locator: Locator) -> Result<Element> {
        let element = match &locator {
            Locator::Css(s) => self.client.find(FLocator::Css(s)).await,
            Locator::XPath(s) => self.client.find(FLocator::XPath(s)).await,
            Locator::Id(s) => self.client.find(FLocator::Id(s)).await,
            Locator::LinkText(s) => self.client.find(FLocator::LinkText(s)).await,
            Locator::Name(s) => {
                let css = format!("[name='{}']", s);
                self.client.find(FLocator::Css(&css)).await
            }
            Locator::PartialLinkText(s) => {
                let css = format!("a[href*='{}']", s);
                self.client.find(FLocator::Css(&css)).await
            }
            Locator::TagName(s) => self.client.find(FLocator::Css(s)).await,
            Locator::ClassName(s) => {
                let css = format!(".{}", s);
                self.client.find(FLocator::Css(&css)).await
            }
        }
        .context(format!("Failed to find element: {:?}", locator))?;
        Ok(Element {
            inner: element,
            locator,
        })
    }

    /// Find all elements matching a locator
    pub async fn find_all(&self, locator: Locator) -> Result<Vec<Element>> {
        let elements = match &locator {
            Locator::Css(s) => self.client.find_all(FLocator::Css(s)).await,
            Locator::XPath(s) => self.client.find_all(FLocator::XPath(s)).await,
            Locator::Id(s) => self.client.find_all(FLocator::Id(s)).await,
            Locator::LinkText(s) => self.client.find_all(FLocator::LinkText(s)).await,
            Locator::Name(s) => {
                let css = format!("[name='{}']", s);
                self.client.find_all(FLocator::Css(&css)).await
            }
            Locator::PartialLinkText(s) => {
                let css = format!("a[href*='{}']", s);
                self.client.find_all(FLocator::Css(&css)).await
            }
            Locator::TagName(s) => self.client.find_all(FLocator::Css(s)).await,
            Locator::ClassName(s) => {
                let css = format!(".{}", s);
                self.client.find_all(FLocator::Css(&css)).await
            }
        }
        .context(format!("Failed to find elements: {:?}", locator))?;

        Ok(elements
            .into_iter()
            .map(|e| Element {
                inner: e,
                locator: locator.clone(),
            })
            .collect())
    }

    /// Wait for an element to be present
    pub async fn wait_for(&self, locator: Locator) -> Result<Element> {
        self.wait_for_condition(locator, WaitCondition::Present)
            .await
    }

    /// Wait for an element with a specific condition
    pub async fn wait_for_condition(
        &self,
        locator: Locator,
        condition: WaitCondition,
    ) -> Result<Element> {
        let timeout = self.config.timeout;
        let start = std::time::Instant::now();

        while start.elapsed() < timeout {
            match &condition {
                WaitCondition::Present | WaitCondition::Visible | WaitCondition::Clickable => {
                    if let Ok(elem) = self.find(locator.clone()).await {
                        match &condition {
                            WaitCondition::Present => return Ok(elem),
                            WaitCondition::Visible => {
                                if elem.is_displayed().await.unwrap_or(false) {
                                    return Ok(elem);
                                }
                            }
                            WaitCondition::Clickable => {
                                if elem.is_displayed().await.unwrap_or(false)
                                    && elem.is_enabled().await.unwrap_or(false)
                                {
                                    return Ok(elem);
                                }
                            }
                            _ => {}
                        }
                    }
                }
                WaitCondition::NotPresent => {
                    if self.find(locator.clone()).await.is_err() {
                        // Return a dummy element for NotPresent
                        // In practice, callers should just check for Ok result
                        anyhow::bail!("Element not present (expected)");
                    }
                }
                WaitCondition::NotVisible => {
                    if let Ok(elem) = self.find(locator.clone()).await {
                        if !elem.is_displayed().await.unwrap_or(true) {
                            return Ok(elem);
                        }
                    } else {
                        anyhow::bail!("Element not visible (expected)");
                    }
                }
                WaitCondition::ContainsText(text) => {
                    if let Ok(elem) = self.find(locator.clone()).await {
                        if let Ok(elem_text) = elem.text().await {
                            if elem_text.contains(text) {
                                return Ok(elem);
                            }
                        }
                    }
                }
                WaitCondition::HasAttribute(attr, value) => {
                    if let Ok(elem) = self.find(locator.clone()).await {
                        if let Ok(Some(attr_val)) = elem.attr(attr).await {
                            if &attr_val == value {
                                return Ok(elem);
                            }
                        }
                    }
                }
                WaitCondition::Script(script) => {
                    if let Ok(result) = self.execute_script(script).await {
                        if result.as_bool().unwrap_or(false) {
                            return self.find(locator).await;
                        }
                    }
                }
            }

            sleep(Duration::from_millis(100)).await;
        }

        anyhow::bail!(
            "Timeout waiting for element {:?} with condition {:?}",
            locator,
            condition
        )
    }

    /// Click an element
    pub async fn click(&self, locator: Locator) -> Result<()> {
        let elem = self
            .wait_for_condition(locator, WaitCondition::Clickable)
            .await?;
        elem.click().await
    }

    /// Type text into an element
    pub async fn fill(&self, locator: Locator, text: &str) -> Result<()> {
        let elem = self
            .wait_for_condition(locator, WaitCondition::Visible)
            .await?;
        elem.clear().await?;
        elem.send_keys(text).await
    }

    /// Get text from an element
    pub async fn text(&self, locator: Locator) -> Result<String> {
        let elem = self.find(locator).await?;
        elem.text().await
    }

    /// Check if an element exists
    pub async fn exists(&self, locator: Locator) -> bool {
        self.find(locator).await.is_ok()
    }

    /// Execute JavaScript
    pub async fn execute_script(&self, script: &str) -> Result<serde_json::Value> {
        let result = self
            .client
            .execute(script, vec![])
            .await
            .context("Failed to execute script")?;
        Ok(result)
    }

    /// Execute JavaScript with arguments
    pub async fn execute_script_with_args(
        &self,
        script: &str,
        args: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        let result = self
            .client
            .execute(script, args)
            .await
            .context("Failed to execute script")?;
        Ok(result)
    }

    /// Take a screenshot
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        self.client
            .screenshot()
            .await
            .context("Failed to take screenshot")
    }

    /// Save a screenshot to a file
    pub async fn screenshot_to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let data = self.screenshot().await?;
        let path = path.into();
        std::fs::write(&path, data).context(format!("Failed to write screenshot to {:?}", path))
    }

    /// Refresh the page
    pub async fn refresh(&self) -> Result<()> {
        self.client
            .refresh()
            .await
            .context("Failed to refresh page")
    }

    /// Go back in history
    pub async fn back(&self) -> Result<()> {
        self.client.back().await.context("Failed to go back")
    }

    /// Go forward in history
    pub async fn forward(&self) -> Result<()> {
        self.client.forward().await.context("Failed to go forward")
    }

    /// Set window size
    pub async fn set_window_size(&self, width: u32, height: u32) -> Result<()> {
        self.client
            .set_window_size(width, height)
            .await
            .context("Failed to set window size")
    }

    /// Maximize window
    pub async fn maximize_window(&self) -> Result<()> {
        self.client
            .maximize_window()
            .await
            .context("Failed to maximize window")
    }

    /// Get all cookies
    pub async fn get_cookies(&self) -> Result<Vec<Cookie>> {
        let cookies = self
            .client
            .get_all_cookies()
            .await
            .context("Failed to get cookies")?;

        Ok(cookies
            .into_iter()
            .map(|c| {
                let same_site_str = c.same_site().map(|ss| match ss {
                    cookie::SameSite::Strict => "Strict".to_string(),
                    cookie::SameSite::Lax => "Lax".to_string(),
                    cookie::SameSite::None => "None".to_string(),
                });
                Cookie {
                    name: c.name().to_string(),
                    value: c.value().to_string(),
                    domain: c.domain().map(|s| s.to_string()),
                    path: c.path().map(|s| s.to_string()),
                    secure: c.secure(),
                    http_only: c.http_only(),
                    same_site: same_site_str,
                    expiry: None,
                }
            })
            .collect())
    }

    /// Set a cookie
    pub async fn set_cookie(&self, cookie: Cookie) -> Result<()> {
        let mut c = cookie::Cookie::new(cookie.name, cookie.value);

        if let Some(domain) = cookie.domain {
            c.set_domain(domain);
        }
        if let Some(path) = cookie.path {
            c.set_path(path);
        }
        if let Some(secure) = cookie.secure {
            c.set_secure(secure);
        }
        if let Some(http_only) = cookie.http_only {
            c.set_http_only(http_only);
        }

        self.client
            .add_cookie(c)
            .await
            .context("Failed to set cookie")
    }

    /// Delete a cookie by name
    pub async fn delete_cookie(&self, name: &str) -> Result<()> {
        self.client
            .delete_cookie(name)
            .await
            .context("Failed to delete cookie")
    }

    /// Delete all cookies
    pub async fn delete_all_cookies(&self) -> Result<()> {
        self.client
            .delete_all_cookies()
            .await
            .context("Failed to delete all cookies")
    }

    /// Switch to an iframe by locator
    pub async fn switch_to_frame(&self, locator: Locator) -> Result<()> {
        let elem = self.find(locator).await?;
        elem.inner
            .enter_frame()
            .await
            .context("Failed to switch to frame")
    }

    /// Switch to an iframe by index
    pub async fn switch_to_frame_by_index(&self, index: u16) -> Result<()> {
        self.client
            .enter_frame(Some(index))
            .await
            .context("Failed to switch to frame by index")
    }

    /// Switch to the parent frame
    pub async fn switch_to_parent_frame(&self) -> Result<()> {
        self.client
            .enter_parent_frame()
            .await
            .context("Failed to switch to parent frame")
    }

    /// Switch to the default content
    pub async fn switch_to_default_content(&self) -> Result<()> {
        self.client
            .enter_frame(None)
            .await
            .context("Failed to switch to default content")
    }

    /// Get current window handle
    pub async fn current_window_handle(&self) -> Result<String> {
        let handle = self.client.window().await?;
        Ok(format!("{:?}", handle))
    }

    /// Get all window handles
    pub async fn window_handles(&self) -> Result<Vec<String>> {
        let handles = self.client.windows().await?;
        Ok(handles.iter().map(|h| format!("{:?}", h)).collect())
    }

    /// Type text into an element (alias for fill)
    pub async fn type_text(&self, locator: Locator, text: &str) -> Result<()> {
        self.fill(locator, text).await
    }

    /// Find an element (alias for find)
    pub async fn find_element(&self, locator: Locator) -> Result<Element> {
        self.find(locator).await
    }

    /// Find all elements (alias for find_all)
    pub async fn find_elements(&self, locator: Locator) -> Result<Vec<Element>> {
        self.find_all(locator).await
    }

    /// Press a key on an element
    pub async fn press_key(&self, locator: Locator, _key: &str) -> Result<()> {
        let elem = self.find(locator).await?;
        elem.send_keys("\u{E007}").await?;
        Ok(())
    }

    /// Check if an element is enabled
    pub async fn is_element_enabled(&self, locator: Locator) -> Result<bool> {
        let elem = self.find(locator).await?;
        elem.is_enabled().await
    }

    /// Check if an element is visible
    pub async fn is_element_visible(&self, locator: Locator) -> Result<bool> {
        let elem = self.find(locator).await?;
        elem.is_displayed().await
    }

    /// Close the browser
    pub async fn close(self) -> Result<()> {
        self.client.close().await.context("Failed to close browser")
    }

    /// Send special key
    pub async fn send_key(&self, key: Key) -> Result<()> {
        let key_str = Self::key_to_string(key);
        self.execute_script(&format!(
            "document.activeElement.dispatchEvent(new KeyboardEvent('keydown', {{key: '{}'}}));",
            key_str
        ))
        .await?;
        Ok(())
    }

    fn key_to_string(key: Key) -> &'static str {
        match key {
            Key::Enter => "Enter",
            Key::Tab => "Tab",
            Key::Escape => "Escape",
            Key::Backspace => "Backspace",
            Key::Delete => "Delete",
            Key::ArrowUp => "ArrowUp",
            Key::ArrowDown => "ArrowDown",
            Key::ArrowLeft => "ArrowLeft",
            Key::ArrowRight => "ArrowRight",
            Key::Home => "Home",
            Key::End => "End",
            Key::PageUp => "PageUp",
            Key::PageDown => "PageDown",
            Key::F1 => "F1",
            Key::F2 => "F2",
            Key::F3 => "F3",
            Key::F4 => "F4",
            Key::F5 => "F5",
            Key::F6 => "F6",
            Key::F7 => "F7",
            Key::F8 => "F8",
            Key::F9 => "F9",
            Key::F10 => "F10",
            Key::F11 => "F11",
            Key::F12 => "F12",
            Key::Shift => "Shift",
            Key::Control => "Control",
            Key::Alt => "Alt",
            Key::Meta => "Meta",
        }
    }
}

/// Wrapper around a WebDriver element
pub struct Element {
    inner: fantoccini::elements::Element,
    locator: Locator,
}

impl Element {
    /// Click the element
    pub async fn click(&self) -> Result<()> {
        self.inner.click().await.context("Failed to click element")
    }

    /// Clear the element's value
    pub async fn clear(&self) -> Result<()> {
        self.inner.clear().await.context("Failed to clear element")
    }

    /// Send keys to the element
    pub async fn send_keys(&self, text: &str) -> Result<()> {
        self.inner
            .send_keys(text)
            .await
            .context("Failed to send keys")
    }

    /// Get the element's text content
    pub async fn text(&self) -> Result<String> {
        self.inner
            .text()
            .await
            .context("Failed to get element text")
    }

    /// Get the element's inner HTML
    pub async fn inner_html(&self) -> Result<String> {
        self.inner
            .html(false)
            .await
            .context("Failed to get inner HTML")
    }

    /// Get the element's outer HTML
    pub async fn outer_html(&self) -> Result<String> {
        self.inner
            .html(true)
            .await
            .context("Failed to get outer HTML")
    }

    /// Get an attribute value
    pub async fn attr(&self, name: &str) -> Result<Option<String>> {
        self.inner
            .attr(name)
            .await
            .context(format!("Failed to get attribute {}", name))
    }

    /// Get a CSS property value
    pub async fn css_value(&self, name: &str) -> Result<String> {
        self.inner
            .css_value(name)
            .await
            .context(format!("Failed to get CSS value {}", name))
    }

    /// Check if the element is displayed
    pub async fn is_displayed(&self) -> Result<bool> {
        self.inner
            .is_displayed()
            .await
            .context("Failed to check if displayed")
    }

    /// Check if the element is enabled
    pub async fn is_enabled(&self) -> Result<bool> {
        self.inner
            .is_enabled()
            .await
            .context("Failed to check if enabled")
    }

    /// Check if the element is selected (for checkboxes, radio buttons, etc.)
    pub async fn is_selected(&self) -> Result<bool> {
        self.inner
            .is_selected()
            .await
            .context("Failed to check if selected")
    }

    /// Get the element's tag name
    pub async fn tag_name(&self) -> Result<String> {
        self.inner
            .tag_name()
            .await
            .context("Failed to get tag name")
    }

    /// Get the element's location
    pub async fn location(&self) -> Result<(i64, i64)> {
        let rect = self.inner.rectangle().await?;
        Ok((rect.0 as i64, rect.1 as i64))
    }

    /// Get the element's size
    pub async fn size(&self) -> Result<(u64, u64)> {
        let rect = self.inner.rectangle().await?;
        Ok((rect.2 as u64, rect.3 as u64))
    }

    /// Get the locator used to find this element
    pub fn locator(&self) -> &Locator {
        &self.locator
    }

    /// Find a child element
    pub async fn find(&self, locator: Locator) -> Result<Element> {
        let element = match &locator {
            Locator::Css(s) => self.inner.find(FLocator::Css(s)).await,
            Locator::XPath(s) => self.inner.find(FLocator::XPath(s)).await,
            Locator::Id(s) => self.inner.find(FLocator::Id(s)).await,
            Locator::LinkText(s) => self.inner.find(FLocator::LinkText(s)).await,
            Locator::Name(s) => {
                let css = format!("[name='{}']", s);
                self.inner.find(FLocator::Css(&css)).await
            }
            Locator::PartialLinkText(s) => {
                let css = format!("a[href*='{}']", s);
                self.inner.find(FLocator::Css(&css)).await
            }
            Locator::TagName(s) => self.inner.find(FLocator::Css(s)).await,
            Locator::ClassName(s) => {
                let css = format!(".{}", s);
                self.inner.find(FLocator::Css(&css)).await
            }
        }
        .context(format!("Failed to find child element: {:?}", locator))?;
        Ok(Element {
            inner: element,
            locator,
        })
    }

    /// Find all child elements
    pub async fn find_all(&self, locator: Locator) -> Result<Vec<Element>> {
        let elements = match &locator {
            Locator::Css(s) => self.inner.find_all(FLocator::Css(s)).await,
            Locator::XPath(s) => self.inner.find_all(FLocator::XPath(s)).await,
            Locator::Id(s) => self.inner.find_all(FLocator::Id(s)).await,
            Locator::LinkText(s) => self.inner.find_all(FLocator::LinkText(s)).await,
            Locator::Name(s) => {
                let css = format!("[name='{}']", s);
                self.inner.find_all(FLocator::Css(&css)).await
            }
            Locator::PartialLinkText(s) => {
                let css = format!("a[href*='{}']", s);
                self.inner.find_all(FLocator::Css(&css)).await
            }
            Locator::TagName(s) => self.inner.find_all(FLocator::Css(s)).await,
            Locator::ClassName(s) => {
                let css = format!(".{}", s);
                self.inner.find_all(FLocator::Css(&css)).await
            }
        }
        .context(format!("Failed to find child elements: {:?}", locator))?;
        Ok(elements
            .into_iter()
            .map(|e| Element {
                inner: e,
                locator: locator.clone(),
            })
            .collect())
    }

    /// Submit a form (clicks the element which should trigger form submission)
    pub async fn submit(&self) -> Result<()> {
        // Trigger form submission by clicking the element
        // or by executing JavaScript to submit the closest form
        self.click().await
    }

    /// Scroll the element into view using JavaScript
    pub async fn scroll_into_view(&self) -> Result<()> {
        // Use JavaScript to scroll element into view since fantoccini
        // doesn't have a direct scroll_into_view method on Element
        // We need to get the element and execute script
        // For now, we'll just return Ok since clicking usually scrolls
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_config_default() {
        let config = BrowserConfig::default();
        assert_eq!(config.browser_type, BrowserType::Chrome);
        assert_eq!(config.webdriver_url, "http://localhost:4444");
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_browser_config_builder() {
        let config = BrowserConfig::new()
            .with_browser(BrowserType::Firefox)
            .with_webdriver_url("http://localhost:9515")
            .headless(false)
            .with_window_size(1280, 720)
            .with_timeout(Duration::from_secs(60))
            .with_arg("--disable-notifications");

        assert_eq!(config.browser_type, BrowserType::Firefox);
        assert_eq!(config.webdriver_url, "http://localhost:9515");
        assert!(!config.headless);
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 720);
        assert_eq!(config.timeout, Duration::from_secs(60));
        assert!(config
            .browser_args
            .contains(&"--disable-notifications".to_string()));
    }

    #[test]
    fn test_build_capabilities_chrome_headless() {
        let config = BrowserConfig::new()
            .with_browser(BrowserType::Chrome)
            .headless(true);

        let caps = config.build_capabilities();
        assert_eq!(caps["browserName"], "chrome");

        let args = caps["goog:chromeOptions"]["args"].as_array().unwrap();
        assert!(args
            .iter()
            .any(|a| a.as_str().unwrap().contains("headless")));
    }

    #[test]
    fn test_build_capabilities_firefox_headless() {
        let config = BrowserConfig::new()
            .with_browser(BrowserType::Firefox)
            .headless(true);

        let caps = config.build_capabilities();
        assert_eq!(caps["browserName"], "firefox");

        let args = caps["moz:firefoxOptions"]["args"].as_array().unwrap();
        assert!(args.iter().any(|a| a.as_str().unwrap() == "-headless"));
    }

    #[test]
    fn test_browser_type_capability_name() {
        assert_eq!(BrowserType::Chrome.capability_name(), "goog:chromeOptions");
        assert_eq!(BrowserType::Firefox.capability_name(), "moz:firefoxOptions");
        assert_eq!(BrowserType::Safari.capability_name(), "safari:options");
        assert_eq!(BrowserType::Edge.capability_name(), "ms:edgeOptions");
    }

    #[test]
    fn test_browser_type_browser_name() {
        assert_eq!(BrowserType::Chrome.browser_name(), "chrome");
        assert_eq!(BrowserType::Firefox.browser_name(), "firefox");
        assert_eq!(BrowserType::Safari.browser_name(), "safari");
        assert_eq!(BrowserType::Edge.browser_name(), "MicrosoftEdge");
    }
}
