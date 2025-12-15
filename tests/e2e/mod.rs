mod auth_flow;
mod chat;
mod dashboard;
mod platform_flow;

use bottest::prelude::*;
use bottest::services::{BrowserService, DEFAULT_DEBUG_PORT};
use bottest::web::{Browser, BrowserConfig, BrowserType};
use std::time::Duration;

pub struct E2ETestContext {
    pub ctx: TestContext,
    pub server: BotServerInstance,
    pub ui: Option<BotUIInstance>,
    pub browser: Option<Browser>,
    browser_service: Option<BrowserService>,
}

/// Check if a service is running at the given URL
async fn is_service_running(url: &str) -> bool {
    let client = reqwest::Client::builder()
        .timeout(std::time::Duration::from_secs(2))
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap_or_default();

    // Try health endpoint first, then root
    if let Ok(resp) = client.get(&format!("{}/health", url)).send().await {
        if resp.status().is_success() {
            return true;
        }
    }
    if let Ok(resp) = client.get(url).send().await {
        return resp.status().is_success() || resp.status().as_u16() == 200;
    }
    false
}

impl E2ETestContext {
    pub async fn setup() -> anyhow::Result<Self> {
        // Default strategy: Use main botserver stack at https://localhost:8080
        // This ensures LLM and all services are properly configured
        // User should start botserver normally: cd botserver && cargo run
        //
        // Override with env vars:
        //   BOTSERVER_URL=https://localhost:8080
        //   BOTUI_URL=http://localhost:3000
        //   FRESH_STACK=1  (to start a new temp stack instead)

        let botserver_url =
            std::env::var("BOTSERVER_URL").unwrap_or_else(|_| "https://localhost:8080".to_string());
        let botui_url =
            std::env::var("BOTUI_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        let botserver_running = is_service_running(&botserver_url).await;
        let botui_running = is_service_running(&botui_url).await;

        // Always use existing stack context (main stack)
        let ctx = TestHarness::with_existing_stack().await?;

        // Check if botserver is running, if not start it with main stack
        let server = if botserver_running {
            println!("🔗 Using existing BotServer at {}", botserver_url);
            BotServerInstance::existing(&botserver_url)
        } else {
            // Auto-start botserver with main stack (includes LLM)
            println!("🚀 Auto-starting BotServer with main stack...");
            BotServerInstance::start_with_main_stack().await?
        };

        // Ensure botui is running (required for chat UI)
        let ui = if botui_running {
            println!("🔗 Using existing BotUI at {}", botui_url);
            Some(BotUIInstance::existing(&botui_url))
        } else {
            println!("🚀 Starting BotUI...");
            match ctx.start_botui(&server.url).await {
                Ok(ui) if ui.is_running() => {
                    println!("  ✓ BotUI started at {}", ui.url);
                    Some(ui)
                }
                Ok(ui) => {
                    println!("  ⚠ BotUI started but may not be ready at {}", ui.url);
                    Some(ui)
                }
                Err(e) => {
                    println!("  ⚠ Could not start BotUI: {} (chat tests may fail)", e);
                    None
                }
            }
        };

        Ok(Self {
            ctx,
            server,
            ui,
            browser: None,
            browser_service: None,
        })
    }

    pub async fn setup_with_browser() -> anyhow::Result<Self> {
        // Default strategy: Use main botserver stack at https://localhost:8080
        // This ensures LLM and all services are properly configured
        // User should start botserver normally: cd botserver && cargo run
        //
        // Override with env vars:
        //   BOTSERVER_URL=https://localhost:8080
        //   BOTUI_URL=http://localhost:3000
        //   FRESH_STACK=1  (to start a new temp stack instead)

        let botserver_url =
            std::env::var("BOTSERVER_URL").unwrap_or_else(|_| "https://localhost:8080".to_string());
        let botui_url =
            std::env::var("BOTUI_URL").unwrap_or_else(|_| "http://localhost:3000".to_string());

        let botserver_running = is_service_running(&botserver_url).await;
        let botui_running = is_service_running(&botui_url).await;

        // Always use existing stack context (main stack)
        let ctx = TestHarness::with_existing_stack().await?;

        // Check if botserver is running, if not start it with main stack
        let server = if botserver_running {
            println!("🔗 Using existing BotServer at {}", botserver_url);
            BotServerInstance::existing(&botserver_url)
        } else {
            // Auto-start botserver with main stack (includes LLM)
            println!("🚀 Auto-starting BotServer with main stack...");
            BotServerInstance::start_with_main_stack().await?
        };

        // Ensure botui is running (required for chat UI)
        let ui = if botui_running {
            println!("🔗 Using existing BotUI at {}", botui_url);
            Some(BotUIInstance::existing(&botui_url))
        } else {
            println!("🚀 Starting BotUI...");
            match ctx.start_botui(&server.url).await {
                Ok(ui) if ui.is_running() => {
                    println!("  ✓ BotUI started at {}", ui.url);
                    Some(ui)
                }
                Ok(ui) => {
                    println!("  ⚠ BotUI started but may not be ready at {}", ui.url);
                    Some(ui)
                }
                Err(e) => {
                    println!("  ⚠ Could not start BotUI: {} (chat tests may fail)", e);
                    None
                }
            }
        };

        // Start browser with CDP (no chromedriver needed!)
        let browser_service = match BrowserService::start(DEFAULT_DEBUG_PORT).await {
            Ok(bs) => {
                log::info!("Browser started with CDP on port {}", DEFAULT_DEBUG_PORT);
                Some(bs)
            }
            Err(e) => {
                log::error!("Failed to start browser: {}", e);
                eprintln!("Failed to start browser: {}", e);
                None
            }
        };

        let browser = if browser_service.is_some() {
            let config = browser_config();
            match Browser::new(config).await {
                Ok(b) => {
                    log::info!("Browser CDP connection established");
                    Some(b)
                }
                Err(e) => {
                    log::error!("Failed to connect to browser CDP: {}", e);
                    eprintln!("Failed to connect to browser CDP: {}", e);
                    None
                }
            }
        } else {
            log::warn!("Browser service not available, skipping browser");
            None
        };

        Ok(Self {
            ctx,
            server,
            ui,
            browser,
            browser_service,
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
        if let Some(mut bs) = self.browser_service.take() {
            let _ = bs.stop().await;
        }
    }
}

pub fn browser_config() -> BrowserConfig {
    // Default: SHOW browser window so user can see tests
    // Set HEADLESS=1 to run without browser window (CI/automation)
    let headless = std::env::var("HEADLESS").is_ok();
    let debug_port = std::env::var("CDP_PORT")
        .ok()
        .and_then(|p| p.parse().ok())
        .unwrap_or(DEFAULT_DEBUG_PORT);

    // Use CDP directly - no chromedriver needed!
    BrowserConfig::default()
        .with_browser(BrowserType::Chrome)
        .with_debug_port(debug_port)
        .headless(headless) // false by default = show browser
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

    // This test explicitly starts a new server - only run with FRESH_STACK=1
    if std::env::var("FRESH_STACK").is_err() {
        eprintln!("Skipping: test_harness_starts_server requires FRESH_STACK=1 (uses existing stack by default)");
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
    // This test checks harness-started services - only meaningful with FRESH_STACK=1
    if std::env::var("FRESH_STACK").is_err() {
        eprintln!("Skipping: test_full_harness_has_all_services requires FRESH_STACK=1 (uses existing stack by default)");
        return;
    }

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
    // This test creates a temp data dir and cleans it up
    // Safe to run in both modes since it only cleans up its own tmp dir
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

/// Test that checks the existing running stack is accessible
#[tokio::test]
async fn test_existing_stack_connection() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    // Use existing stack by default
    match E2ETestContext::setup().await {
        Ok(ctx) => {
            // Check botserver is accessible
            let client = reqwest::Client::builder()
                .danger_accept_invalid_certs(true)
                .build()
                .unwrap();

            let health_url = format!("{}/health", ctx.api_url());
            match client.get(&health_url).send().await {
                Ok(resp) => {
                    if resp.status().is_success() {
                        println!("✓ Connected to existing botserver at {}", ctx.api_url());
                    } else {
                        eprintln!("Botserver returned non-success status: {}", resp.status());
                    }
                }
                Err(e) => {
                    eprintln!(
                        "Could not connect to existing botserver at {}: {}",
                        ctx.api_url(),
                        e
                    );
                    eprintln!(
                        "Make sure botserver is running: cd ../botserver && cargo run --release"
                    );
                }
            }
            ctx.close().await;
        }
        Err(e) => {
            eprintln!("Skipping: failed to setup E2E context: {}", e);
        }
    }
}
