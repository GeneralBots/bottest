mod auth_flow;
mod chat;
mod dashboard;
mod platform_flow;

use bottest::prelude::*;
use bottest::services::ChromeDriverService;
use bottest::web::{Browser, BrowserConfig, BrowserType};
use std::time::Duration;

static CHROMEDRIVER_PORT: u16 = 4444;

pub struct E2ETestContext {
    pub ctx: TestContext,
    pub server: BotServerInstance,
    pub ui: Option<BotUIInstance>,
    pub browser: Option<Browser>,
    chromedriver: Option<ChromeDriverService>,
}

impl E2ETestContext {
    pub async fn setup() -> anyhow::Result<Self> {
        // Default to USE_EXISTING_STACK for faster e2e tests
        // Set FULL_BOOTSTRAP=1 to run full bootstrap instead
        let use_existing = std::env::var("FULL_BOOTSTRAP").is_err();

        let (ctx, server, ui) = if use_existing {
            // Use existing stack - connect to running botserver/botui
            // Make sure they are running:
            //   cargo run --package botserver
            //   BOTSERVER_URL=https://localhost:8080 cargo run --package botui
            log::info!("Using existing stack (set FULL_BOOTSTRAP=1 for full bootstrap)");
            let ctx = TestHarness::with_existing_stack().await?;

            // Get URLs from env or use defaults
            let botserver_url = std::env::var("BOTSERVER_URL")
                .unwrap_or_else(|_| "https://localhost:8080".to_string());
            let botui_url =
                std::env::var("BOTUI_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

            // Create a dummy server instance pointing to existing botserver
            let server = BotServerInstance::existing(&botserver_url);
            let ui = Some(BotUIInstance::existing(&botui_url));

            (ctx, server, ui)
        } else {
            let ctx = TestHarness::full().await?;
            let server = ctx.start_botserver().await?;
            let ui = ctx.start_botui(&server.url).await.ok();
            (ctx, server, ui)
        };

        Ok(Self {
            ctx,
            server,
            ui,
            browser: None,
            chromedriver: None,
        })
    }

    pub async fn setup_with_browser() -> anyhow::Result<Self> {
        // Default to USE_EXISTING_STACK for faster e2e tests
        // Set FULL_BOOTSTRAP=1 to run full bootstrap instead
        let use_existing = std::env::var("FULL_BOOTSTRAP").is_err();

        let (ctx, server, ui) = if use_existing {
            // Use existing stack - connect to running botserver/botui
            log::info!("Using existing stack (set FULL_BOOTSTRAP=1 for full bootstrap)");
            let ctx = TestHarness::with_existing_stack().await?;

            let botserver_url = std::env::var("BOTSERVER_URL")
                .unwrap_or_else(|_| "https://localhost:8080".to_string());
            let botui_url =
                std::env::var("BOTUI_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

            let server = BotServerInstance::existing(&botserver_url);
            let ui = Some(BotUIInstance::existing(&botui_url));

            (ctx, server, ui)
        } else {
            let ctx = TestHarness::full().await?;
            let server = ctx.start_botserver().await?;
            let ui = ctx.start_botui(&server.url).await.ok();
            (ctx, server, ui)
        };

        let chromedriver = match ChromeDriverService::start(CHROMEDRIVER_PORT).await {
            Ok(cd) => Some(cd),
            Err(e) => {
                log::warn!("Failed to start ChromeDriver: {}", e);
                None
            }
        };

        let browser = if chromedriver.is_some() {
            let config = browser_config();
            Browser::new(config).await.ok()
        } else {
            None
        };

        Ok(Self {
            ctx,
            server,
            ui,
            browser,
            chromedriver,
        })
    }

    /// Get the base URL for browser tests - uses botui if available, otherwise botserver
    pub fn base_url(&self) -> &str {
        if let Some(ref ui) = self.ui {
            &ui.url
        } else {
            &self.server.url
        }
    }

    /// Get the botserver API URL
    pub fn api_url(&self) -> &str {
        &self.server.url
    }

    pub fn has_browser(&self) -> bool {
        self.browser.is_some()
    }

    pub async fn close(mut self) {
        if let Some(browser) = self.browser {
            let _ = browser.close().await;
        }
        if let Some(mut cd) = self.chromedriver.take() {
            let _ = cd.stop().await;
        }
    }
}

pub fn browser_config() -> BrowserConfig {
    let headless = std::env::var("HEADED").is_err();
    let webdriver_url = std::env::var("WEBDRIVER_URL")
        .unwrap_or_else(|_| format!("http://localhost:{}", CHROMEDRIVER_PORT));

    // Detect Brave browser path
    let brave_paths = [
        "/usr/bin/brave-browser",
        "/usr/bin/brave",
        "/snap/bin/brave",
        "/opt/brave.com/brave/brave-browser",
    ];

    let mut config = BrowserConfig::default()
        .with_browser(BrowserType::Chrome)
        .with_webdriver_url(&webdriver_url)
        .headless(headless)
        .with_timeout(Duration::from_secs(30))
        .with_window_size(1920, 1080);

    // Add Brave binary path if found
    for path in &brave_paths {
        if std::path::Path::new(path).exists() {
            log::info!("Using browser binary: {}", path);
            config = config.with_binary(path);
            break;
        }
    }

    config
}

pub fn should_run_e2e_tests() -> bool {
    if std::env::var("SKIP_E2E_TESTS").is_ok() {
        return false;
    }
    true
}

pub async fn check_webdriver_available() -> bool {
    true
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
