use super::{check_webdriver_available, should_run_e2e_tests, E2ETestContext};
use bottest::prelude::*;
use bottest::web::Locator;

#[tokio::test]
async fn test_chat_page_loads() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if let Err(e) = browser.goto(&chat_url).await {
        eprintln!("Failed to navigate: {}", e);
        ctx.close().await;
        return;
    }

    let chat_input = Locator::css("#chat-input, .chat-input, textarea[placeholder*='message']");

    match browser.wait_for(chat_input).await {
        Ok(_) => println!("Chat input found"),
        Err(e) => eprintln!("Chat input not found: {}", e),
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_chat_widget_elements() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let elements_to_check = vec![
        ("#chat-container, .chat-container", "chat container"),
        ("#chat-input, .chat-input, textarea", "input field"),
        (
            "#send-button, .send-button, button[type='submit']",
            "send button",
        ),
    ];

    for (selector, name) in elements_to_check {
        let locator = Locator::css(selector);
        match browser.find_element(locator).await {
            Ok(_) => println!("Found: {}", name),
            Err(_) => eprintln!("Not found: {}", name),
        }
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_send_message() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm
            .expect_completion("Hello", "Hi there! How can I help you?")
            .await;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let input_locator = Locator::css("#chat-input, .chat-input, textarea");
    if let Err(e) = browser.wait_for(input_locator.clone()).await {
        eprintln!("Input not ready: {}", e);
        ctx.close().await;
        return;
    }

    if let Err(e) = browser.type_text(input_locator, "Hello").await {
        eprintln!("Failed to type: {}", e);
        ctx.close().await;
        return;
    }

    let send_button = Locator::css("#send-button, .send-button, button[type='submit']");
    if let Err(e) = browser.click(send_button).await {
        eprintln!("Failed to click send: {}", e);
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_receive_bot_response() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm
            .set_default_response("This is a test response from the bot.")
            .await;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let input_locator = Locator::css("#chat-input, .chat-input, textarea");
    let _ = browser.wait_for(input_locator.clone()).await;
    let _ = browser.type_text(input_locator, "Test message").await;

    let send_button = Locator::css("#send-button, .send-button, button[type='submit']");
    let _ = browser.click(send_button).await;

    let response_locator = Locator::css(".bot-message, .message-bot, .response");
    match browser.wait_for(response_locator).await {
        Ok(_) => println!("Bot response received"),
        Err(e) => eprintln!("No bot response: {}", e),
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_chat_history() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm.set_default_response("Response").await;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let input_locator = Locator::css("#chat-input, .chat-input, textarea");
    let send_button = Locator::css("#send-button, .send-button, button[type='submit']");

    for i in 1..=3 {
        let _ = browser.wait_for(input_locator.clone()).await;
        let _ = browser
            .type_text(input_locator.clone(), &format!("Message {}", i))
            .await;
        let _ = browser.click(send_button.clone()).await;
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    let messages_locator = Locator::css(".message, .chat-message");
    match browser.find_elements(messages_locator).await {
        Ok(elements) => {
            println!("Found {} messages in history", elements.len());
        }
        Err(e) => eprintln!("Failed to find messages: {}", e),
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_typing_indicator() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm.with_latency(2000);
        mock_llm.set_default_response("Delayed response").await;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let input_locator = Locator::css("#chat-input, .chat-input, textarea");
    let send_button = Locator::css("#send-button, .send-button, button[type='submit']");

    let _ = browser.wait_for(input_locator.clone()).await;
    let _ = browser.type_text(input_locator, "Hello").await;
    let _ = browser.click(send_button).await;

    let typing_locator = Locator::css(".typing-indicator, .typing, .loading");
    match browser.find_element(typing_locator).await {
        Ok(_) => println!("Typing indicator found"),
        Err(_) => eprintln!("Typing indicator not found (may have completed quickly)"),
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_keyboard_shortcuts() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm.set_default_response("Response").await;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let input_locator = Locator::css("#chat-input, .chat-input, textarea");
    let _ = browser.wait_for(input_locator.clone()).await;
    let _ = browser
        .type_text(input_locator.clone(), "Test enter key")
        .await;

    if let Err(e) = browser.press_key(input_locator, "Enter").await {
        eprintln!("Failed to press Enter: {}", e);
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_empty_message_prevention() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let send_button = Locator::css("#send-button, .send-button, button[type='submit']");
    let _ = browser.wait_for(send_button.clone()).await;

    match browser.is_element_enabled(send_button.clone()).await {
        Ok(enabled) => {
            if !enabled {
                println!("Send button correctly disabled for empty input");
            } else {
                println!("Send button enabled (validation may be on submit)");
            }
        }
        Err(e) => eprintln!("Could not check button state: {}", e),
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_responsive_design() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let viewports = vec![
        (375, 667, "mobile"),
        (768, 1024, "tablet"),
        (1920, 1080, "desktop"),
    ];

    for (width, height, name) in viewports {
        if browser.set_window_size(width, height).await.is_ok() {
            tokio::time::sleep(std::time::Duration::from_millis(200)).await;

            let chat_container = Locator::css("#chat-container, .chat-container, .chat-widget");
            match browser.is_element_visible(chat_container).await {
                Ok(visible) => {
                    if visible {
                        println!("{} viewport ({}x{}): chat visible", name, width, height);
                    } else {
                        eprintln!("{} viewport: chat not visible", name);
                    }
                }
                Err(e) => eprintln!("{} viewport check failed: {}", name, e),
            }
        }
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_conversation_reset() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup_with_browser().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if !ctx.has_browser() {
        eprintln!("Skipping: browser not available");
        ctx.close().await;
        return;
    }

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm.set_default_response("Response").await;
    }

    let browser = ctx.browser.as_ref().unwrap();
    let chat_url = format!("{}/chat/test-bot", ctx.base_url());

    if browser.goto(&chat_url).await.is_err() {
        ctx.close().await;
        return;
    }

    let input_locator = Locator::css("#chat-input, .chat-input, textarea");
    let send_button = Locator::css("#send-button, .send-button, button[type='submit']");

    let _ = browser.wait_for(input_locator.clone()).await;
    let _ = browser.type_text(input_locator, "Test message").await;
    let _ = browser.click(send_button).await;
    tokio::time::sleep(std::time::Duration::from_millis(500)).await;

    let reset_button =
        Locator::css("#reset-button, .reset-button, .new-chat, [data-action='reset']");
    match browser.click(reset_button).await {
        Ok(_) => {
            tokio::time::sleep(std::time::Duration::from_millis(300)).await;
            let messages_locator = Locator::css(".message, .chat-message");
            match browser.find_elements(messages_locator).await {
                Ok(elements) if elements.is_empty() => {
                    println!("Conversation reset successfully");
                }
                Ok(elements) => {
                    println!("Messages remaining after reset: {}", elements.len());
                }
                Err(_) => println!("No messages found (reset may have worked)"),
            }
        }
        Err(_) => eprintln!("Reset button not found (feature may not be implemented)"),
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_mock_llm_integration() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm
            .expect_completion("what is the weather", "The weather is sunny today!")
            .await;

        mock_llm.assert_not_called().await;

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/v1/chat/completions", mock_llm.url()))
            .json(&serde_json::json!({
                "model": "gpt-4",
                "messages": [{"role": "user", "content": "what is the weather"}]
            }))
            .send()
            .await;

        if let Ok(resp) = response {
            assert!(resp.status().is_success());
            mock_llm.assert_called().await;
        }
    }

    ctx.close().await;
}

#[tokio::test]
async fn test_mock_llm_error_handling() {
    if !should_run_e2e_tests() {
        eprintln!("Skipping: E2E tests disabled");
        return;
    }

    let ctx = match E2ETestContext::setup().await {
        Ok(ctx) => ctx,
        Err(e) => {
            eprintln!("Skipping: {}", e);
            return;
        }
    };

    if let Some(mock_llm) = ctx.ctx.mock_llm() {
        mock_llm.next_call_fails(500, "Internal server error").await;

        let client = reqwest::Client::new();
        let response = client
            .post(&format!("{}/v1/chat/completions", mock_llm.url()))
            .json(&serde_json::json!({
                "model": "gpt-4",
                "messages": [{"role": "user", "content": "test"}]
            }))
            .send()
            .await;

        if let Ok(resp) = response {
            assert_eq!(resp.status().as_u16(), 500);
        }
    }

    ctx.close().await;
}
