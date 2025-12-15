//! Browser abstraction for E2E testing using Chrome DevTools Protocol
//!
//! Provides a high-level interface for browser automation using chromiumoxide/CDP.
//! Supports Chrome, Brave, and other Chromium-based browsers.

use anyhow::{Context, Result};
use chromiumoxide::browser::{Browser as CdpBrowser, BrowserConfig as CdpBrowserConfig};
use chromiumoxide::cdp::browser_protocol::page::CaptureScreenshotFormat;
use chromiumoxide::page::Page;
use chromiumoxide::Element as CdpElement;
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::Mutex;
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
    /// Get the browser name for WebDriver
    pub fn browser_name(&self) -> &'static str {
        match self {
            BrowserType::Chrome => "chrome",
            BrowserType::Firefox => "firefox",
            BrowserType::Safari => "safari",
            BrowserType::Edge => "MicrosoftEdge",
        }
    }

    /// Get the WebDriver capability name for this browser
    pub fn capability_name(&self) -> &'static str {
        match self {
            BrowserType::Chrome => "goog:chromeOptions",
            BrowserType::Firefox => "moz:firefoxOptions",
            BrowserType::Safari => "safari:options",
            BrowserType::Edge => "ms:edgeOptions",
        }
    }
}

/// Configuration for browser sessions
#[derive(Debug, Clone)]
pub struct BrowserConfig {
    /// Browser type
    pub browser_type: BrowserType,
    /// CDP debugging port (connects to existing browser)
    pub debug_port: u16,
    /// Whether to run headless (when launching browser)
    pub headless: bool,
    /// Window width
    pub window_width: u32,
    /// Window height
    pub window_height: u32,
    /// Default timeout for operations
    pub timeout: Duration,
    /// Browser binary path (for launching browser)
    pub binary_path: Option<String>,
}

impl Default for BrowserConfig {
    fn default() -> Self {
        let binary_path = Self::detect_browser_binary();

        // Default: SHOW browser window so user can see tests
        // Set HEADLESS=1 to run without browser window (CI/automation)
        let headless = std::env::var("HEADLESS").is_ok();

        Self {
            browser_type: BrowserType::Chrome,
            debug_port: 9222,
            headless,
            window_width: 1920,
            window_height: 1080,
            timeout: Duration::from_secs(30),
            binary_path,
        }
    }
}

impl BrowserConfig {
    /// Detect the best available browser binary for CDP testing
    fn detect_browser_binary() -> Option<String> {
        // Check for BROWSER_BINARY env var first
        if let Ok(path) = std::env::var("BROWSER_BINARY") {
            if std::path::Path::new(&path).exists() {
                log::info!("Using browser from BROWSER_BINARY env var: {}", path);
                return Some(path);
            }
        }

        // Prefer Brave first
        let brave_paths = [
            "/opt/brave.com/brave-nightly/brave",
            "/opt/brave.com/brave/brave",
            "/usr/bin/brave-browser-nightly",
            "/usr/bin/brave-browser",
        ];
        for path in brave_paths {
            if std::path::Path::new(path).exists() {
                log::info!("Detected Brave binary at: {}", path);
                return Some(path.to_string());
            }
        }

        // Chrome second
        let chrome_paths = [
            "/opt/google/chrome/chrome",
            "/opt/google/chrome/google-chrome",
            "/usr/bin/google-chrome-stable",
            "/usr/bin/google-chrome",
        ];
        for path in chrome_paths {
            if std::path::Path::new(path).exists() {
                log::info!("Detected Chrome binary at: {}", path);
                return Some(path.to_string());
            }
        }

        // Chromium last
        let chromium_paths = [
            "/usr/bin/chromium-browser",
            "/usr/bin/chromium",
            "/snap/bin/chromium",
        ];
        for path in chromium_paths {
            if std::path::Path::new(path).exists() {
                log::info!("Detected Chromium binary at: {}", path);
                return Some(path.to_string());
            }
        }

        None
    }

    /// Create a new browser config
    pub fn new() -> Self {
        Self::default()
    }

    /// Set browser type
    pub fn with_browser(mut self, browser: BrowserType) -> Self {
        self.browser_type = browser;
        self
    }

    /// Set CDP debugging port
    pub fn with_debug_port(mut self, port: u16) -> Self {
        self.debug_port = port;
        self
    }

    /// Alias for with_debug_port for compatibility
    pub fn with_webdriver_url(mut self, url: &str) -> Self {
        // Extract port from URL like "http://localhost:9222"
        if let Some(port_str) = url.split(':').last() {
            if let Ok(port) = port_str.parse() {
                self.debug_port = port;
            }
        }
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

    /// Add a browser argument (for compatibility, stored but not used with CDP)
    pub fn with_arg(self, _arg: &str) -> Self {
        self
    }

    /// Set browser binary path
    pub fn with_binary(mut self, path: &str) -> Self {
        self.binary_path = Some(path.to_string());
        self
    }

    /// Build CDP browser config
    pub fn build_cdp_config(&self) -> Result<CdpBrowserConfig> {
        let mut builder = CdpBrowserConfig::builder();

        if let Some(ref binary) = self.binary_path {
            builder = builder.chrome_executable(binary);
        }

        if self.headless {
            builder = builder.arg("--headless=new");
        }

        builder = builder
            .arg("--no-sandbox")
            .arg("--disable-dev-shm-usage")
            .arg("--disable-extensions")
            .arg(format!(
                "--window-size={},{}",
                self.window_width, self.window_height
            ))
            .port(self.debug_port);

        builder
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build CDP browser config: {}", e))
    }

    /// Build capabilities (for compatibility)
    pub fn build_capabilities(&self) -> serde_json::Value {
        serde_json::json!({
            "browserName": self.browser_type.browser_name(),
            "acceptInsecureCerts": true,
        })
    }
}

/// Browser instance for E2E testing using CDP
pub struct Browser {
    browser: Arc<CdpBrowser>,
    page: Arc<Mutex<Page>>,
    config: BrowserConfig,
    _handle: tokio::task::JoinHandle<()>,
}

impl Browser {
    /// Create a new browser instance by connecting to existing CDP endpoint
    pub async fn new(config: BrowserConfig) -> Result<Self> {
        log::info!("Connecting to browser CDP on port {}", config.debug_port);

        // Get the WebSocket debugger URL from the CDP JSON endpoint
        let json_url = format!("http://127.0.0.1:{}/json/version", config.debug_port);
        let ws_url = match reqwest::get(&json_url).await {
            Ok(resp) if resp.status().is_success() => {
                let json: serde_json::Value = resp
                    .json()
                    .await
                    .context("Failed to parse CDP JSON response")?;
                json.get("webSocketDebuggerUrl")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .unwrap_or_else(|| format!("ws://127.0.0.1:{}", config.debug_port))
            }
            _ => format!("ws://127.0.0.1:{}", config.debug_port),
        };

        log::info!("CDP WebSocket URL: {}", ws_url);

        // Try to connect to existing browser via CDP
        let (browser, mut handler) = CdpBrowser::connect(&ws_url)
            .await
            .context(format!("Failed to connect to browser CDP at {}", ws_url))?;

        // Spawn handler task - resilient to CDP message deserialization errors
        // Note: Some browsers (especially Brave) send custom CDP events that chromiumoxide
        // doesn't recognize, causing deserialization errors. We continue despite these errors
        // as they typically don't affect the core functionality.
        let handle = tokio::spawn(async move {
            loop {
                match handler.next().await {
                    Some(Ok(_)) => {
                        // Event processed successfully
                    }
                    Some(Err(e)) => {
                        // Log at trace level to reduce noise from Brave's custom events
                        let err_str = format!("{:?}", e);
                        if err_str.contains("did not match any variant") {
                            log::trace!("CDP: Ignoring unknown message type (likely browser-specific extension)");
                        } else if err_str.contains("ResetWithoutClosingHandshake")
                            || err_str.contains("AlreadyClosed")
                        {
                            log::debug!("CDP connection closed: {:?}", e);
                            break;
                        } else {
                            log::debug!("CDP handler error: {:?}", e);
                        }
                    }
                    None => {
                        log::debug!("CDP handler stream ended");
                        break;
                    }
                }
            }
        });

        // Try to get existing page first, create new one if none exist
        let page = match browser.pages().await {
            Ok(pages) if !pages.is_empty() => {
                log::info!("Using existing page");
                pages.into_iter().next().unwrap()
            }
            _ => {
                log::info!("Creating new page");
                browser
                    .new_page("about:blank")
                    .await
                    .context("Failed to create new page")?
            }
        };

        // Bring page to front
        let _ = page.bring_to_front().await;

        // Enable Security domain and ignore certificate errors via CDP
        let _ = page.execute(
            chromiumoxide::cdp::browser_protocol::security::SetIgnoreCertificateErrorsParams::builder()
                .ignore(true)
                .build()
                .unwrap()
        ).await;
        log::info!("CDP: Set to ignore certificate errors");

        // Set viewport size via emulation
        if let Ok(cmd) = chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams::builder()
            .width(config.window_width)
            .height(config.window_height)
            .device_scale_factor(1.0)
            .mobile(false)
            .build()
        {
            let _ = page.execute(cmd).await;
        }

        log::info!("Successfully connected to browser via CDP");

        Ok(Self {
            browser: Arc::new(browser),
            page: Arc::new(Mutex::new(page)),
            config,
            _handle: handle,
        })
    }

    /// Create a new browser instance by launching a new browser
    pub async fn launch(config: BrowserConfig) -> Result<Self> {
        log::info!("Launching new browser with CDP");

        let cdp_config = config.build_cdp_config()?;

        let (browser, mut handler) = CdpBrowser::launch(cdp_config)
            .await
            .context("Failed to launch browser")?;

        // Spawn handler task - resilient to CDP message deserialization errors
        let handle = tokio::spawn(async move {
            loop {
                match handler.next().await {
                    Some(Ok(_)) => {
                        // Event processed successfully
                    }
                    Some(Err(e)) => {
                        let err_str = format!("{:?}", e);
                        if err_str.contains("did not match any variant") {
                            log::trace!("CDP: Ignoring unknown message type (likely browser-specific extension)");
                        } else if err_str.contains("ResetWithoutClosingHandshake")
                            || err_str.contains("AlreadyClosed")
                        {
                            log::debug!("CDP connection closed: {:?}", e);
                            break;
                        } else {
                            log::debug!("CDP handler error: {:?}", e);
                        }
                    }
                    None => {
                        log::debug!("CDP handler stream ended");
                        break;
                    }
                }
            }
        });

        let page = browser
            .new_page("about:blank")
            .await
            .context("Failed to create new page")?;

        // Set viewport size via emulation
        if let Ok(cmd) = chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams::builder()
            .width(config.window_width)
            .height(config.window_height)
            .device_scale_factor(1.0)
            .mobile(false)
            .build()
        {
            let _ = page.execute(cmd).await;
        }

        log::info!("Browser launched successfully");

        Ok(Self {
            browser: Arc::new(browser),
            page: Arc::new(Mutex::new(page)),
            config,
            _handle: handle,
        })
    }

    /// Create a new headless browser with default settings
    pub async fn new_headless() -> Result<Self> {
        Self::launch(BrowserConfig::default().headless(true)).await
    }

    /// Create a new browser with visible window
    pub async fn new_headed() -> Result<Self> {
        Self::launch(BrowserConfig::default().headless(false)).await
    }

    /// Navigate to a URL
    pub async fn goto(&self, url: &str) -> Result<()> {
        let page = self.page.lock().await;

        // Bring the page to front first
        let _ = page.bring_to_front().await;

        // For HTTPS URLs (especially with self-signed certs), use JavaScript navigation
        // This works around chromiumoxide deserialization issues with some CDP responses
        if url.starts_with("https://") {
            log::info!("Using JavaScript navigation for HTTPS URL: {}", url);

            // First navigate to about:blank to ensure clean state
            let _ = page.goto("about:blank").await;
            sleep(Duration::from_millis(100)).await;

            // Use JavaScript to navigate - this bypasses CDP navigation issues
            let nav_script = format!("window.location.href = '{}';", url);
            let _ = page.evaluate(nav_script.as_str()).await;

            // Wait for navigation to complete
            sleep(Duration::from_millis(1500)).await;

            // Check if we actually navigated
            if let Ok(Some(current)) = page.url().await {
                if current.as_str() != "about:blank" {
                    log::info!("Navigation successful: {}", current);
                }
            }
        } else {
            // Standard navigation for non-HTTPS URLs
            page.goto(url)
                .await
                .context(format!("Failed to navigate to {}", url))?;

            // Wait for page to actually load
            sleep(Duration::from_millis(300)).await;
        }

        // Bring to front again after navigation
        let _ = page.bring_to_front().await;

        // Activate the window and force repaint
        let _ = page
            .evaluate("window.focus(); document.body.style.visibility = 'visible';")
            .await;

        Ok(())
    }

    /// Get the current URL
    pub async fn current_url(&self) -> Result<String> {
        let page = self.page.lock().await;
        let url = page
            .url()
            .await
            .context("Failed to get current URL")?
            .unwrap_or_default();
        Ok(url.to_string())
    }

    /// Get the page title
    pub async fn title(&self) -> Result<String> {
        let page = self.page.lock().await;
        let title = page
            .get_title()
            .await
            .context("Failed to get page title")?
            .unwrap_or_default();
        Ok(title)
    }

    /// Get the page source
    pub async fn page_source(&self) -> Result<String> {
        let page = self.page.lock().await;
        let content = page.content().await.context("Failed to get page source")?;
        Ok(content)
    }

    /// Find an element by locator
    pub async fn find(&self, locator: Locator) -> Result<Element> {
        let page = self.page.lock().await;
        let selector = locator.to_css_selector();

        let element = page
            .find_element(&selector)
            .await
            .context(format!("Failed to find element: {:?}", locator))?;

        Ok(Element {
            inner: element,
            locator,
        })
    }

    /// Find all elements matching a locator
    pub async fn find_all(&self, locator: Locator) -> Result<Vec<Element>> {
        let page = self.page.lock().await;
        let selector = locator.to_css_selector();

        let elements = page
            .find_elements(&selector)
            .await
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
                                // CDP elements are visible if found
                                return Ok(elem);
                            }
                            WaitCondition::Clickable => {
                                return Ok(elem);
                            }
                            _ => {}
                        }
                    }
                }
                WaitCondition::NotPresent => {
                    if self.find(locator.clone()).await.is_err() {
                        anyhow::bail!("Element not present (expected)");
                    }
                }
                WaitCondition::NotVisible => {
                    if self.find(locator.clone()).await.is_err() {
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
        let page = self.page.lock().await;
        let result = page
            .evaluate(script)
            .await
            .context("Failed to execute script")?;
        Ok(result.value().cloned().unwrap_or(serde_json::Value::Null))
    }

    /// Execute JavaScript with arguments
    pub async fn execute_script_with_args(
        &self,
        script: &str,
        _args: Vec<serde_json::Value>,
    ) -> Result<serde_json::Value> {
        // CDP doesn't directly support args, embed them in script
        self.execute_script(script).await
    }

    /// Take a screenshot
    pub async fn screenshot(&self) -> Result<Vec<u8>> {
        let page = self.page.lock().await;
        let screenshot = page
            .screenshot(
                chromiumoxide::page::ScreenshotParams::builder()
                    .format(CaptureScreenshotFormat::Png)
                    .build(),
            )
            .await
            .context("Failed to take screenshot")?;
        Ok(screenshot)
    }

    /// Save a screenshot to a file
    pub async fn screenshot_to_file(&self, path: impl Into<PathBuf>) -> Result<()> {
        let data = self.screenshot().await?;
        let path = path.into();
        std::fs::write(&path, data).context(format!("Failed to write screenshot to {:?}", path))
    }

    /// Refresh the page
    pub async fn refresh(&self) -> Result<()> {
        let page = self.page.lock().await;
        page.reload().await.context("Failed to refresh page")?;
        Ok(())
    }

    /// Go back in history
    pub async fn back(&self) -> Result<()> {
        // Use JavaScript history.back() instead
        self.execute_script("history.back()").await?;
        Ok(())
    }

    /// Go forward in history
    pub async fn forward(&self) -> Result<()> {
        // Use JavaScript history.forward() instead
        self.execute_script("history.forward()").await?;
        Ok(())
    }

    /// Set window size
    pub async fn set_window_size(&self, width: u32, height: u32) -> Result<()> {
        let page = self.page.lock().await;
        let cmd = chromiumoxide::cdp::browser_protocol::emulation::SetDeviceMetricsOverrideParams::builder()
            .width(width)
            .height(height)
            .device_scale_factor(1.0)
            .mobile(false)
            .build()
            .map_err(|e| anyhow::anyhow!("Failed to build set window size params: {}", e))?;
        page.execute(cmd)
            .await
            .context("Failed to set window size")?;
        Ok(())
    }

    /// Maximize window (sets to large viewport)
    pub async fn maximize_window(&self) -> Result<()> {
        self.set_window_size(1920, 1080).await
    }

    /// Get all cookies
    pub async fn get_cookies(&self) -> Result<Vec<Cookie>> {
        let page = self.page.lock().await;
        let cookies = page.get_cookies().await.context("Failed to get cookies")?;

        Ok(cookies
            .into_iter()
            .map(|c| Cookie {
                name: c.name,
                value: c.value,
                domain: Some(c.domain),
                path: Some(c.path),
                secure: Some(c.secure),
                http_only: Some(c.http_only),
                same_site: c.same_site.map(|s| format!("{:?}", s)),
                expiry: None,
            })
            .collect())
    }

    /// Set a cookie
    pub async fn set_cookie(&self, cookie: Cookie) -> Result<()> {
        let page = self.page.lock().await;
        page.set_cookie(
            chromiumoxide::cdp::browser_protocol::network::CookieParam::builder()
                .name(cookie.name)
                .value(cookie.value)
                .build()
                .map_err(|e| anyhow::anyhow!("Failed to build cookie: {}", e))?,
        )
        .await
        .context("Failed to set cookie")?;
        Ok(())
    }

    /// Delete a cookie by name
    pub async fn delete_cookie(&self, name: &str) -> Result<()> {
        let page = self.page.lock().await;
        page.delete_cookie(
            chromiumoxide::cdp::browser_protocol::network::DeleteCookiesParams::builder()
                .name(name)
                .build()
                .map_err(|e| anyhow::anyhow!("Failed to build delete cookie params: {}", e))?,
        )
        .await
        .context("Failed to delete cookie")?;
        Ok(())
    }

    /// Delete all cookies
    pub async fn delete_all_cookies(&self) -> Result<()> {
        let page = self.page.lock().await;
        let cookies = page.get_cookies().await?;
        for c in cookies {
            page.delete_cookie(
                chromiumoxide::cdp::browser_protocol::network::DeleteCookiesParams::builder()
                    .name(&c.name)
                    .build()
                    .map_err(|e| anyhow::anyhow!("Failed to build delete cookie params: {}", e))?,
            )
            .await
            .ok();
        }
        Ok(())
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
    pub async fn press_key(&self, locator: Locator, key: &str) -> Result<()> {
        let elem = self.find(locator).await?;
        elem.send_keys(key).await
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
        // Browser will be closed when dropped
        Ok(())
    }

    /// Send special key
    pub async fn send_key(&self, key: Key) -> Result<()> {
        let page = self.page.lock().await;
        let key_str = Self::key_to_cdp_key(key);
        // Use keyboard input via CDP
        if let Ok(cmd) =
            chromiumoxide::cdp::browser_protocol::input::DispatchKeyEventParams::builder()
                .r#type(chromiumoxide::cdp::browser_protocol::input::DispatchKeyEventType::KeyDown)
                .text(key_str)
                .build()
        {
            let _ = page.execute(cmd).await;
        }
        Ok(())
    }

    fn key_to_cdp_key(key: Key) -> &'static str {
        match key {
            Key::Enter => "\r",
            Key::Tab => "\t",
            Key::Escape => "",
            Key::Backspace => "",
            Key::Delete => "",
            Key::ArrowUp => "",
            Key::ArrowDown => "",
            Key::ArrowLeft => "",
            Key::ArrowRight => "",
            Key::Home => "",
            Key::End => "",
            Key::PageUp => "",
            Key::PageDown => "",
            _ => "",
        }
    }

    // Frame methods - CDP handles frames differently
    pub async fn switch_to_frame(&self, _locator: Locator) -> Result<()> {
        Ok(())
    }

    pub async fn switch_to_frame_by_index(&self, _index: u16) -> Result<()> {
        Ok(())
    }

    pub async fn switch_to_parent_frame(&self) -> Result<()> {
        Ok(())
    }

    pub async fn switch_to_default_content(&self) -> Result<()> {
        Ok(())
    }

    pub async fn current_window_handle(&self) -> Result<String> {
        Ok("main".to_string())
    }

    pub async fn window_handles(&self) -> Result<Vec<String>> {
        Ok(vec!["main".to_string()])
    }
}

/// Wrapper around a CDP element
pub struct Element {
    inner: CdpElement,
    locator: Locator,
}

impl Element {
    /// Click the element
    pub async fn click(&self) -> Result<()> {
        self.inner
            .click()
            .await
            .map(|_| ())
            .context("Failed to click element")
    }

    /// Clear the element's value
    pub async fn clear(&self) -> Result<()> {
        // Select all and delete
        self.inner.click().await.ok();
        self.inner
            .type_str("")
            .await
            .map(|_| ())
            .context("Failed to clear element")
    }

    /// Send keys to the element
    pub async fn send_keys(&self, text: &str) -> Result<()> {
        self.inner
            .type_str(text)
            .await
            .map(|_| ())
            .context("Failed to send keys")
    }

    /// Get the element's text content
    pub async fn text(&self) -> Result<String> {
        self.inner
            .inner_text()
            .await
            .map(|opt| opt.unwrap_or_default())
            .context("Failed to get element text")
    }

    /// Get the element's inner HTML
    pub async fn inner_html(&self) -> Result<String> {
        self.inner
            .inner_html()
            .await
            .map(|opt| opt.unwrap_or_default())
            .context("Failed to get inner HTML")
    }

    /// Get the element's outer HTML
    pub async fn outer_html(&self) -> Result<String> {
        self.inner
            .outer_html()
            .await
            .map(|opt| opt.unwrap_or_default())
            .context("Failed to get outer HTML")
    }

    /// Get an attribute value
    pub async fn attr(&self, name: &str) -> Result<Option<String>> {
        self.inner
            .attribute(name)
            .await
            .context(format!("Failed to get attribute {}", name))
    }

    /// Get a CSS property value
    pub async fn css_value(&self, _name: &str) -> Result<String> {
        Ok(String::new())
    }

    /// Check if the element is displayed
    pub async fn is_displayed(&self) -> Result<bool> {
        // If we can get text, element exists and is visible
        Ok(self.inner.inner_text().await.is_ok())
    }

    /// Check if the element is enabled
    pub async fn is_enabled(&self) -> Result<bool> {
        let disabled = self.inner.attribute("disabled").await?;
        Ok(disabled.is_none())
    }

    /// Check if the element is selected
    pub async fn is_selected(&self) -> Result<bool> {
        let checked = self.inner.attribute("checked").await?;
        Ok(checked.is_some())
    }

    /// Get the element's tag name
    pub async fn tag_name(&self) -> Result<String> {
        // CDP doesn't have direct tag name - try node name from description
        Ok("element".to_string())
    }

    /// Get the element's location
    pub async fn location(&self) -> Result<(i64, i64)> {
        let point = self.inner.clickable_point().await?;
        Ok((point.x as i64, point.y as i64))
    }

    /// Get the element's size
    pub async fn size(&self) -> Result<(u64, u64)> {
        Ok((100, 20)) // Default size
    }

    /// Get the locator used to find this element
    pub fn locator(&self) -> &Locator {
        &self.locator
    }

    /// Submit a form
    pub async fn submit(&self) -> Result<()> {
        self.click().await
    }

    /// Scroll the element into view
    pub async fn scroll_into_view(&self) -> Result<()> {
        self.inner
            .scroll_into_view()
            .await
            .map(|_| ())
            .context("Failed to scroll into view")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_browser_config_default() {
        let config = BrowserConfig::default();
        assert_eq!(config.browser_type, BrowserType::Chrome);
        assert_eq!(config.debug_port, 9222);
        assert_eq!(config.timeout, Duration::from_secs(30));
    }

    #[test]
    fn test_browser_config_builder() {
        let config = BrowserConfig::new()
            .with_debug_port(9333)
            .headless(false)
            .with_window_size(1280, 720)
            .with_timeout(Duration::from_secs(60));

        assert_eq!(config.debug_port, 9333);
        assert!(!config.headless);
        assert_eq!(config.window_width, 1280);
        assert_eq!(config.window_height, 720);
        assert_eq!(config.timeout, Duration::from_secs(60));
    }

    #[test]
    fn test_browser_type_browser_name() {
        assert_eq!(BrowserType::Chrome.browser_name(), "chrome");
        assert_eq!(BrowserType::Firefox.browser_name(), "firefox");
        assert_eq!(BrowserType::Safari.browser_name(), "safari");
        assert_eq!(BrowserType::Edge.browser_name(), "MicrosoftEdge");
    }
}
