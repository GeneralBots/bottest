use super::{should_run_e2e_tests, E2ETestContext};
use bottest::prelude::*;
use bottest::web::Locator;

/// Simple "hi" chat test with real botserver
#[tokio::test]
async fn test_chat_hi() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Test failed: {}", e);
            panic!("Failed to setup E2E context: {}", e);
        }
    };

    if !ctx.has_browser() {
        ctx.close().await;
        panic!("Browser not available - cannot run E2E test");
    }

    // Chat UI requires botui
    if ctx.ui.is_none() {
        ctx.close().await;
        panic!("BotUI not available - chat tests require botui running on port 3000");
    }

    let browser = ctx.browser.as_ref().unwrap();
    // Use botui URL for chat (botserver is API only)
    let ui_url = ctx.ui.as_ref().unwrap().url.clone();
    let chat_url = format!("{}/#chat", ui_url);

    println!("🌐 Navigating to: {}", chat_url);

    if let Err(e) = browser.goto(&chat_url).await {
        ctx.close().await;
        panic!("Failed to navigate to chat: {}", e);
    }

    // Wait for page to load and HTMX to initialize chat content
    println!("⏳ Waiting for page to load...");
    tokio::time::sleep(std::time::Duration::from_secs(3)).await;

    // Chat input: botui uses #messageInput or #ai-input
    let input = Locator::css("#messageInput, #ai-input, .ai-input");

    // Try to find input with retries (HTMX loads content dynamically)
    let mut found_input = false;
    for attempt in 1..=10 {
        if browser.exists(input.clone()).await {
            found_input = true;
            println!("✓ Chat input found (attempt {})", attempt);
            break;
        }
        println!("  ... waiting for chat input (attempt {}/10)", attempt);
        tokio::time::sleep(std::time::Duration::from_secs(1)).await;
    }

    if !found_input {
        // Take screenshot on failure
        if let Ok(screenshot) = browser.screenshot().await {
            let _ = std::fs::write("/tmp/bottest-chat-fail.png", &screenshot);
            println!("Screenshot saved to /tmp/bottest-chat-fail.png");
        }
        // Also print page source for debugging
        if let Ok(source) = browser.page_source().await {
            let preview: String = source.chars().take(2000).collect();
            println!("Page source preview:\n{}", preview);
        }
        ctx.close().await;
        panic!("Chat input not found after 10 attempts");
    }

    // Type "hi"
    println!("⌨️ Typing 'hi'...");
    if let Err(e) = browser.type_text(input.clone(), "hi").await {
        ctx.close().await;
        panic!("Failed to type: {}", e);
    }

    // Click send button or press Enter
    let send_btn = Locator::css("#sendBtn, #ai-send, .ai-send, button[type='submit']");
    match browser.click(send_btn).await {
        Ok(_) => println!("✓ Message sent (click)"),
        Err(_) => {
            // Try Enter key instead
            match browser.press_key(input, "Enter").await {
                Ok(_) => println!("✓ Message sent (Enter key)"),
                Err(e) => println!("⚠ Send may have failed: {}", e),
            }
        }
    }

    // Wait for response
    println!("⏳ Waiting for bot response...");
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;

    // Check for response - botui uses .message.bot or .assistant class
    let response =
        Locator::css(".message.bot, .message.assistant, .bot-message, .assistant-message");
    match browser.find_elements(response).await {
        Ok(elements) if !elements.is_empty() => {
            println!("✓ Bot responded! ({} messages)", elements.len());
        }
        _ => {
            println!("⚠ No bot response detected (may need LLM configuration)");
        }
    }

    // Take final screenshot
    if let Ok(screenshot) = browser.screenshot().await {
        let _ = std::fs::write("/tmp/bottest-chat-result.png", &screenshot);
        println!("📸 Screenshot: /tmp/bottest-chat-result.png");
    }

    ctx.close().await;
    println!("✅ Chat test complete!");
}

#[tokio::test]
async fn test_chat_page_loads() {
    if !should_run_e2e_tests() {
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            panic!("Setup failed: {}", e);
        }
    };

    if !ctx.has_browser() {
        ctx.close().await;
        panic!("Browser not available");
    }

    // Chat UI requires botui
    if ctx.ui.is_none() {
        ctx.close().await;
        panic!("BotUI not available - chat tests require botui. Start it with: cd ../botui && cargo run");
    }

    let browser = ctx.browser.as_ref().unwrap();
    // Use botui URL for chat (botserver is API only)
    let ui_url = ctx.ui.as_ref().unwrap().url.clone();
    let chat_url = format!("{}/#chat", ui_url);

    if let Err(e) = browser.goto(&chat_url).await {
        ctx.close().await;
        panic!("Navigation failed: {}", e);
    }

    tokio::time::sleep(std::time::Duration::from_secs(1)).await;

    let input = Locator::css("#messageInput, input[type='text'], textarea");
    match browser.wait_for(input).await {
        Ok(_) => println!("✓ Chat loaded"),
        Err(e) => {
            if let Ok(s) = browser.screenshot().await {
                let _ = std::fs::write("/tmp/bottest-fail.png", &s);
            }
            ctx.close().await;
            panic!("Chat not loaded: {}", e);
        }
    }

    ctx.close().await;
}
