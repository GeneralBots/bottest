//! Page Object Pattern implementations for E2E testing
//!
//! Provides structured page objects for interacting with botserver's web interface.
//! Each page object encapsulates the locators and actions for a specific page.

use anyhow::Result;
use std::time::Duration;

use super::browser::{Browser, Element};
use super::Locator;

/// Base trait for all page objects
#[async_trait::async_trait]
pub trait Page {
    /// Get the expected URL pattern for this page
    fn url_pattern(&self) -> &str;

    /// Check if we're on this page
    async fn is_current(&self, browser: &Browser) -> Result<bool> {
        let url = browser.current_url().await?;
        Ok(url.contains(self.url_pattern()))
    }

    /// Wait for the page to be fully loaded
    async fn wait_for_load(&self, browser: &Browser) -> Result<()>;
}

// Login Page

/// Login page object
pub struct LoginPage {
    pub base_url: String,
}

impl LoginPage {
    /// Create a new login page object
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Navigate to the login page
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser.goto(&format!("{}/login", self.base_url)).await
    }

    /// Email input locator
    pub fn email_input() -> Locator {
        Locator::css("#email, input[name='email'], input[type='email']")
    }

    /// Password input locator
    pub fn password_input() -> Locator {
        Locator::css("#password, input[name='password'], input[type='password']")
    }

    /// Login button locator
    pub fn login_button() -> Locator {
        Locator::css(
            "#login-button, button[type='submit'], input[type='submit'], .login-btn, .btn-login",
        )
    }

    /// Error message locator
    pub fn error_message() -> Locator {
        Locator::css(".error, .error-message, .alert-error, .alert-danger, [role='alert']")
    }

    /// Enter email
    pub async fn enter_email(&self, browser: &Browser, email: &str) -> Result<()> {
        browser.fill(Self::email_input(), email).await
    }

    /// Enter password
    pub async fn enter_password(&self, browser: &Browser, password: &str) -> Result<()> {
        browser.fill(Self::password_input(), password).await
    }

    /// Click login button
    pub async fn click_login(&self, browser: &Browser) -> Result<()> {
        browser.click(Self::login_button()).await
    }

    /// Perform full login
    pub async fn login(&self, browser: &Browser, email: &str, password: &str) -> Result<()> {
        self.navigate(browser).await?;
        self.wait_for_load(browser).await?;
        self.enter_email(browser, email).await?;
        self.enter_password(browser, password).await?;
        self.click_login(browser).await?;
        // Wait for navigation
        tokio::time::sleep(Duration::from_millis(500)).await;
        Ok(())
    }

    /// Check if error message is displayed
    pub async fn has_error(&self, browser: &Browser) -> bool {
        browser.exists(Self::error_message()).await
    }

    /// Get error message text
    pub async fn get_error_message(&self, browser: &Browser) -> Result<String> {
        browser.text(Self::error_message()).await
    }
}

#[async_trait::async_trait]
impl Page for LoginPage {
    fn url_pattern(&self) -> &str {
        "/login"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::email_input()).await?;
        browser.wait_for(Self::password_input()).await?;
        Ok(())
    }
}

// Dashboard Page

/// Dashboard home page object
pub struct DashboardPage {
    pub base_url: String,
}

impl DashboardPage {
    /// Create a new dashboard page object
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Navigate to the dashboard
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser.goto(&format!("{}/dashboard", self.base_url)).await
    }

    /// Stats cards container locator
    /// Stats cards locator
    pub fn stats_cards() -> Locator {
        Locator::css(".stats-card, .dashboard-stat, .metric-card")
    }

    /// Navigation menu locator
    pub fn nav_menu() -> Locator {
        Locator::css("nav, .nav, .sidebar, .navigation")
    }

    /// User profile button locator
    pub fn user_profile() -> Locator {
        Locator::css(".user-profile, .user-menu, .profile-dropdown, .avatar")
    }

    /// Logout button locator
    pub fn logout_button() -> Locator {
        Locator::css(".logout, .logout-btn, #logout, a[href*='logout'], button:contains('Logout')")
    }

    /// Get navigation menu items
    pub async fn get_nav_items(&self, browser: &Browser) -> Result<Vec<Element>> {
        browser
            .find_all(Locator::css("nav a, .nav-item, .menu-item"))
            .await
    }

    /// Click a navigation item by text
    pub async fn navigate_to(&self, browser: &Browser, menu_text: &str) -> Result<()> {
        let locator = Locator::xpath(&format!("//nav//a[contains(text(), '{}')]", menu_text));
        browser.click(locator).await
    }

    /// Click logout
    pub async fn logout(&self, browser: &Browser) -> Result<()> {
        // First try to open user menu if needed
        if browser.exists(Self::user_profile()).await {
            let _ = browser.click(Self::user_profile()).await;
            tokio::time::sleep(Duration::from_millis(200)).await;
        }
        browser.click(Self::logout_button()).await
    }
}

#[async_trait::async_trait]
impl Page for DashboardPage {
    fn url_pattern(&self) -> &str {
        "/dashboard"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::nav_menu()).await?;
        Ok(())
    }
}

// Chat Page

/// Chat interface page object
pub struct ChatPage {
    pub base_url: String,
    pub bot_name: String,
}

impl ChatPage {
    /// Create a new chat page object
    pub fn new(base_url: &str, bot_name: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            bot_name: bot_name.to_string(),
        }
    }

    /// Navigate to the chat page
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser
            .goto(&format!("{}/chat/{}", self.base_url, self.bot_name))
            .await
    }

    /// Chat input locator
    /// Chat input field locator
    pub fn chat_input() -> Locator {
        Locator::css(
            "#chat-input, .chat-input, input[name='message'], textarea[name='message'], .message-input",
        )
    }

    /// Send button locator
    pub fn send_button() -> Locator {
        Locator::css("#send, .send-btn, button[type='submit'], .send-message")
    }

    /// Message list container locator
    pub fn message_list() -> Locator {
        Locator::css(".messages, .message-list, .chat-messages, #messages")
    }

    /// Bot message locator
    pub fn bot_message() -> Locator {
        Locator::css(".bot-message, .message-bot, .assistant-message, [data-role='bot']")
    }

    /// User message locator
    pub fn user_message() -> Locator {
        Locator::css(".user-message, .message-user, [data-role='user']")
    }

    /// Typing indicator locator
    pub fn typing_indicator() -> Locator {
        Locator::css(".typing, .typing-indicator, .is-typing, [data-typing]")
    }

    /// File upload button locator
    pub fn file_upload_button() -> Locator {
        Locator::css(".upload-btn, .file-upload, input[type='file'], .attach-file")
    }

    /// Quick reply buttons locator
    pub fn quick_reply_buttons() -> Locator {
        Locator::css(".quick-replies, .quick-reply, .suggested-reply")
    }

    /// Send a message
    pub async fn send_message(&self, browser: &Browser, message: &str) -> Result<()> {
        browser.fill(Self::chat_input(), message).await?;
        browser.click(Self::send_button()).await?;
        Ok(())
    }

    /// Wait for bot response
    pub async fn wait_for_response(&self, browser: &Browser, timeout: Duration) -> Result<()> {
        let start = std::time::Instant::now();

        // First wait for typing indicator to appear
        while start.elapsed() < timeout {
            if browser.exists(Self::typing_indicator()).await {
                break;
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        // Then wait for typing indicator to disappear
        while start.elapsed() < timeout {
            if !browser.exists(Self::typing_indicator()).await {
                return Ok(());
            }
            tokio::time::sleep(Duration::from_millis(100)).await;
        }

        anyhow::bail!("Timeout waiting for bot response")
    }

    /// Get all bot messages
    pub async fn get_bot_messages(&self, browser: &Browser) -> Result<Vec<String>> {
        let elements = browser.find_all(Self::bot_message()).await?;
        let mut messages = Vec::new();
        for elem in elements {
            if let Ok(text) = elem.text().await {
                messages.push(text);
            }
        }
        Ok(messages)
    }

    /// Get all user messages
    pub async fn get_user_messages(&self, browser: &Browser) -> Result<Vec<String>> {
        let elements = browser.find_all(Self::user_message()).await?;
        let mut messages = Vec::new();
        for elem in elements {
            if let Ok(text) = elem.text().await {
                messages.push(text);
            }
        }
        Ok(messages)
    }

    /// Get the last bot message
    pub async fn get_last_bot_message(&self, browser: &Browser) -> Result<String> {
        let messages = self.get_bot_messages(browser).await?;
        messages
            .last()
            .cloned()
            .ok_or_else(|| anyhow::anyhow!("No bot messages found"))
    }

    /// Check if typing indicator is visible
    pub async fn is_typing(&self, browser: &Browser) -> bool {
        browser.exists(Self::typing_indicator()).await
    }

    /// Click a quick reply button by text
    pub async fn click_quick_reply(&self, browser: &Browser, text: &str) -> Result<()> {
        let locator = Locator::xpath(&format!(
            "//*[contains(@class, 'quick-reply') and contains(text(), '{}')]",
            text
        ));
        browser.click(locator).await
    }
}

#[async_trait::async_trait]
impl Page for ChatPage {
    fn url_pattern(&self) -> &str {
        "/chat/"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::chat_input()).await?;
        browser.wait_for(Self::message_list()).await?;
        Ok(())
    }
}

// Queue Panel Page

/// Queue management panel page object
pub struct QueuePage {
    pub base_url: String,
}

impl QueuePage {
    /// Create a new queue page object
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Navigate to the queue panel
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser.goto(&format!("{}/queue", self.base_url)).await
    }

    /// Queue panel container locator
    pub fn queue_panel() -> Locator {
        Locator::css(".queue-panel, .queue-container, #queue-panel")
    }

    /// Queue count display locator
    pub fn queue_count() -> Locator {
        Locator::css(".queue-count, .waiting-count, #queue-count")
    }

    /// Queue entry locator
    pub fn queue_entry() -> Locator {
        Locator::css(".queue-entry, .queue-item, .waiting-customer")
    }

    /// Take next button locator
    pub fn take_next_button() -> Locator {
        Locator::css(".take-next, #take-next, button:contains('Take Next')")
    }

    /// Get queue count
    pub async fn get_queue_count(&self, browser: &Browser) -> Result<u32> {
        let text = browser.text(Self::queue_count()).await?;
        text.parse::<u32>()
            .map_err(|_| anyhow::anyhow!("Failed to parse queue count: {}", text))
    }

    /// Get all queue entries
    pub async fn get_queue_entries(&self, browser: &Browser) -> Result<Vec<Element>> {
        browser.find_all(Self::queue_entry()).await
    }

    /// Click take next button
    pub async fn take_next(&self, browser: &Browser) -> Result<()> {
        browser.click(Self::take_next_button()).await
    }
}

#[async_trait::async_trait]
impl Page for QueuePage {
    fn url_pattern(&self) -> &str {
        "/queue"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::queue_panel()).await?;
        Ok(())
    }
}

// Bot Management Page

/// Bot management page object
pub struct BotManagementPage {
    pub base_url: String,
}

impl BotManagementPage {
    /// Create a new bot management page object
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Navigate to bot management
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser.goto(&format!("{}/admin/bots", self.base_url)).await
    }

    /// Bot list container locator
    pub fn bot_list() -> Locator {
        Locator::css(".bot-list, .bots-container, #bots")
    }

    /// Bot item locator
    pub fn bot_item() -> Locator {
        Locator::css(".bot-item, .bot-card, .bot-entry")
    }

    /// Create bot button locator
    pub fn create_bot_button() -> Locator {
        Locator::css(".create-bot, .new-bot, #create-bot, button:contains('Create')")
    }

    /// Bot name input locator
    pub fn bot_name_input() -> Locator {
        Locator::css("#bot-name, input[name='name'], .bot-name-input")
    }

    /// Bot description input locator
    pub fn bot_description_input() -> Locator {
        Locator::css("#bot-description, textarea[name='description'], .bot-description-input")
    }

    /// Save button locator
    pub fn save_button() -> Locator {
        Locator::css(".save-btn, button[type='submit'], #save, button:contains('Save')")
    }

    /// Get all bots
    pub async fn get_bots(&self, browser: &Browser) -> Result<Vec<Element>> {
        browser.find_all(Self::bot_item()).await
    }

    /// Click create bot button
    pub async fn click_create_bot(&self, browser: &Browser) -> Result<()> {
        browser.click(Self::create_bot_button()).await
    }

    /// Create a new bot
    pub async fn create_bot(&self, browser: &Browser, name: &str, description: &str) -> Result<()> {
        self.click_create_bot(browser).await?;
        tokio::time::sleep(Duration::from_millis(300)).await;
        browser.fill(Self::bot_name_input(), name).await?;
        browser
            .fill(Self::bot_description_input(), description)
            .await?;
        browser.click(Self::save_button()).await?;
        Ok(())
    }

    /// Click edit on a bot by name
    /// Edit a bot by name
    pub async fn edit_bot(&self, browser: &Browser, bot_name: &str) -> Result<()> {
        let locator = Locator::xpath(&format!(
            "//*[contains(@class, 'bot-item') and contains(., '{}')]//button[contains(@class, 'edit')]",
            bot_name
        ));
        browser.click(locator).await
    }
}

#[async_trait::async_trait]
impl Page for BotManagementPage {
    fn url_pattern(&self) -> &str {
        "/admin/bots"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::bot_list()).await?;
        Ok(())
    }
}

// Knowledge Base Page

/// Knowledge base management page object
pub struct KnowledgeBasePage {
    pub base_url: String,
}

impl KnowledgeBasePage {
    /// Create a new knowledge base page object
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Navigate to knowledge base
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser.goto(&format!("{}/admin/kb", self.base_url)).await
    }

    /// KB entries list locator
    /// KB list container locator
    pub fn kb_list() -> Locator {
        Locator::css(".kb-list, .knowledge-base-list, #kb-list")
    }

    /// KB entry locator
    pub fn kb_entry() -> Locator {
        Locator::css(".kb-entry, .kb-item, .knowledge-entry")
    }

    /// Upload button locator
    pub fn upload_button() -> Locator {
        Locator::css(".upload-btn, #upload, button:contains('Upload')")
    }

    /// File input locator
    pub fn file_input() -> Locator {
        Locator::css("input[type='file']")
    }

    /// Search input locator
    pub fn search_input() -> Locator {
        Locator::css(".search-input, #search, input[placeholder*='search']")
    }

    /// Get all KB entries
    pub async fn get_entries(&self, browser: &Browser) -> Result<Vec<Element>> {
        browser.find_all(Self::kb_entry()).await
    }

    /// Search the knowledge base
    pub async fn search(&self, browser: &Browser, query: &str) -> Result<()> {
        browser.fill(Self::search_input(), query).await
    }
}

#[async_trait::async_trait]
impl Page for KnowledgeBasePage {
    fn url_pattern(&self) -> &str {
        "/admin/kb"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::kb_list()).await?;
        Ok(())
    }
}

// Analytics Page

/// Analytics dashboard page object
pub struct AnalyticsPage {
    pub base_url: String,
}

impl AnalyticsPage {
    /// Create a new analytics page object
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
        }
    }

    /// Navigate to analytics
    pub async fn navigate(&self, browser: &Browser) -> Result<()> {
        browser
            .goto(&format!("{}/admin/analytics", self.base_url))
            .await
    }

    /// Charts container locator
    pub fn charts_container() -> Locator {
        Locator::css(".charts, .analytics-charts, #charts")
    }

    /// Date range picker locator
    pub fn date_range_picker() -> Locator {
        Locator::css(".date-range, .date-picker, #date-range")
    }

    /// Metric card locator
    pub fn metric_card() -> Locator {
        Locator::css(".metric-card, .analytics-metric, .stat-card")
    }

    /// Get all metric cards
    pub async fn get_metrics(&self, browser: &Browser) -> Result<Vec<Element>> {
        browser.find_all(Self::metric_card()).await
    }
}

#[async_trait::async_trait]
impl Page for AnalyticsPage {
    fn url_pattern(&self) -> &str {
        "/admin/analytics"
    }

    async fn wait_for_load(&self, browser: &Browser) -> Result<()> {
        browser.wait_for(Self::charts_container()).await?;
        Ok(())
    }
}

// Tests

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_login_page_locators() {
        let _ = LoginPage::email_input();
        let _ = LoginPage::password_input();
        let _ = LoginPage::login_button();
        let _ = LoginPage::error_message();
    }

    #[test]
    fn test_chat_page_locators() {
        let _ = ChatPage::chat_input();
        let _ = ChatPage::send_button();
        let _ = ChatPage::bot_message();
        let _ = ChatPage::typing_indicator();
    }

    #[test]
    fn test_queue_page_locators() {
        let _ = QueuePage::queue_panel();
        let _ = QueuePage::queue_count();
        let _ = QueuePage::take_next_button();
    }

    #[test]
    fn test_page_url_patterns() {
        let login = LoginPage::new("http://localhost:4242");
        assert_eq!(login.url_pattern(), "/login");

        let dashboard = DashboardPage::new("http://localhost:4242");
        assert_eq!(dashboard.url_pattern(), "/dashboard");

        let chat = ChatPage::new("http://localhost:4242", "test-bot");
        assert_eq!(chat.url_pattern(), "/chat/");

        let queue = QueuePage::new("http://localhost:4242");
        assert_eq!(queue.url_pattern(), "/queue");

        let bots = BotManagementPage::new("http://localhost:4242");
        assert_eq!(bots.url_pattern(), "/admin/bots");
    }
}
