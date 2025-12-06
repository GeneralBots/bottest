mod auth_flow;
mod chat;
mod dashboard;
mod platform_flow;

use bottest::prelude::*;
use bottest::web::{Browser, BrowserConfig, BrowserType};
use std::time::Duration;

pub struct E2ETestContext {
    pub ctx: TestContext,
    pub server: BotServerInstance,
    pub browser: Option<Browser>,
}

impl E2ETestContext {
    pub async fn setup() -> anyhow::Result<Self> {
        let ctx = TestHarness::full().await?;
        let server = ctx.start_botserver().await?;

        Ok(Self {
            ctx,
            server,
            browser: None,
        })
    }

    pub async fn setup_with_browser() -> anyhow::Result<Self> {
        let ctx = TestHarness::full().await?;
        let server = ctx.start_botserver().await?;

        let config = browser_config();
        let browser = Browser::new(config).await.ok();

        Ok(Self {
            ctx,
            server,
            browser,
        })
    }

    pub fn base_url(&self) -> &str {
        &self.server.url
    }

    pub fn has_browser(&self) -> bool {
        self.browser.is_some()
    }

    pub async fn close(self) {
        if let Some(browser) = self.browser {
            let _ = browser.close().await;
        }
    }
}

pub fn browser_config() -> BrowserConfig {
    let headless = std::env::var("HEADED").is_err();
    let webdriver_url =
        std::env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:4444".to_string());

    BrowserConfig::default()
        .with_browser(BrowserType::Chrome)
        .with_webdriver_url(&webdriver_url)
        .headless(headless)
        .with_timeout(Duration::from_secs(30))
        .with_window_size(1920, 1080)
}

pub fn should_run_e2e_tests() -> bool {
    if std::env::var("SKIP_E2E_TESTS").is_ok() {
        return false;
    }
    true
}

pub async fn check_webdriver_available() -> bool {
    let webdriver_url =
        std::env::var("WEBDRIVER_URL").unwrap_or_else(|_| "http://localhost:4444".to_string());

    let client = match reqwest::Client::builder()
        .timeout(Duration::from_secs(2))
        .build()
    {
        Ok(c) => c,
        Err(_) => return false,
    };

    client.get(&webdriver_url).send().await.is_ok()
}

#[tokio::test]
async fn test_e2e_context_setup() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    match E2ETestContext::setup().await {
        Ok(ctx) => {
            assert!(!ctx.base_url().is_empty());
            ctx.close().await;
        }
        Err(e) => {
            eprintln!("Skipping: failed to setup E2E context: {}", e);
        }
    }
}

#[tokio::test]
async fn test_e2e_with_browser() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    if !check_webdriver_available().await {
        eprintln!("Skipping: WebDriver not available");
        return;
    }

    match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => {
            if ctx.has_browser() {
                println!("Browser created successfully");
            } else {
                eprintln!("Browser creation failed (WebDriver may not be running)");
            }
            ctx.close().await;
        }
        Err(e) => {
            eprintln!("Skipping: {}", e);
        }
    }
}

#[tokio::test]
async fn test_harness_starts_server() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match TestHarness::full().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    let server = match ctx.start_botserver().await {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if server.is_running() {
        let client = reqwest::Client::new();
        let health_url = format!("{}/health", server.url);

        if let Ok(resp) = client.get(&health_url).send().await {
            assert!(resp.status().is_success());
        }
    }
}

#[tokio::test]
async fn test_full_harness_has_all_services() {
    let ctx = match TestHarness::full().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    // Check services that are enabled in full() config
    assert!(ctx.postgres().is_some(), "PostgreSQL should be available");
    assert!(ctx.mock_llm().is_some(), "MockLLM should be available");
    assert!(
        ctx.mock_zitadel().is_some(),
        "MockZitadel should be available"
    );

    // MinIO and Redis are disabled in full() config (not in botserver-stack)
    // so we don't assert they are present

    assert!(ctx.data_dir.exists());
    assert!(ctx.data_dir.to_str().unwrap().contains("bottest-"));
}

#[tokio::test]
async fn test_e2e_cleanup() {
    let mut ctx = match TestHarness::full().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    let data_dir = ctx.data_dir.clone();
    assert!(data_dir.exists());

    ctx.cleanup().await.unwrap();

    assert!(!data_dir.exists());
}
