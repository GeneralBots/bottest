
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::HashMap;

#[must_use] 
pub fn sample_config() -> HashMap<String, String> {
    let mut config = HashMap::new();
    config.insert("llm-model".to_string(), "gpt-4".to_string());
    config.insert("llm-temperature".to_string(), "0.7".to_string());
    config.insert("llm-max-tokens".to_string(), "1000".to_string());
    config.insert("kb-enabled".to_string(), "true".to_string());
    config.insert("kb-threshold".to_string(), "0.75".to_string());
    config.insert("attendance-enabled".to_string(), "true".to_string());
    config.insert("attendance-queue-size".to_string(), "50".to_string());
    config
}

#[must_use] 
pub fn sample_bot_config() -> Value {
    json!({
        "name": "test-bot",
        "description": "Test bot for automated testing",
        "llm": {
            "provider": "openai",
            "model": "gpt-4",
            "temperature": 0.7,
            "max_tokens": 1000,
            "system_prompt": "You are a helpful assistant."
        },
        "kb": {
            "enabled": true,
            "threshold": 0.75,
            "max_results": 5
        },
        "channels": {
            "whatsapp": {
                "enabled": true,
                "phone_number_id": "123456789"
            },
            "teams": {
                "enabled": true,
                "bot_id": "test-bot-id"
            },
            "web": {
                "enabled": true
            }
        }
    })
}

#[must_use] 
pub fn whatsapp_text_message(from: &str, text: &str) -> Value {
    json!({
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123456789",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "15551234567",
                        "phone_number_id": "987654321"
                    },
                    "contacts": [{
                        "profile": {
                            "name": "Test User"
                        },
                        "wa_id": from
                    }],
                    "messages": [{
                        "from": from,
                        "id": format!("wamid.{}", uuid::Uuid::new_v4().to_string().replace('-', "")),
                        "timestamp": chrono::Utc::now().timestamp().to_string(),
                        "type": "text",
                        "text": {
                            "body": text
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    })
}

#[must_use] 
pub fn whatsapp_button_reply(from: &str, button_id: &str, button_text: &str) -> Value {
    json!({
        "object": "whatsapp_business_account",
        "entry": [{
            "id": "123456789",
            "changes": [{
                "value": {
                    "messaging_product": "whatsapp",
                    "metadata": {
                        "display_phone_number": "15551234567",
                        "phone_number_id": "987654321"
                    },
                    "contacts": [{
                        "profile": {
                            "name": "Test User"
                        },
                        "wa_id": from
                    }],
                    "messages": [{
                        "from": from,
                        "id": format!("wamid.{}", uuid::Uuid::new_v4().to_string().replace('-', "")),
                        "timestamp": chrono::Utc::now().timestamp().to_string(),
                        "type": "interactive",
                        "interactive": {
                            "type": "button_reply",
                            "button_reply": {
                                "id": button_id,
                                "title": button_text
                            }
                        }
                    }]
                },
                "field": "messages"
            }]
        }]
    })
}

#[must_use] 
pub fn teams_message_activity(from_id: &str, from_name: &str, text: &str) -> Value {
    json!({
        "type": "message",
        "id": uuid::Uuid::new_v4().to_string(),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "serviceUrl": "https://smba.trafficmanager.net/teams/",
        "channelId": "msteams",
        "from": {
            "id": from_id,
            "name": from_name,
            "aadObjectId": uuid::Uuid::new_v4().to_string()
        },
        "conversation": {
            "id": format!("conv-{}", uuid::Uuid::new_v4()),
            "conversationType": "personal",
            "tenantId": "test-tenant-id"
        },
        "recipient": {
            "id": "28:test-bot-id",
            "name": "TestBot"
        },
        "text": text,
        "textFormat": "plain",
        "locale": "en-US",
        "channelData": {
            "tenant": {
                "id": "test-tenant-id"
            }
        }
    })
}

#[must_use] 
pub fn openai_chat_request(messages: Vec<(&str, &str)>) -> Value {
    let msgs: Vec<Value> = messages
        .into_iter()
        .map(|(role, content)| {
            json!({
                "role": role,
                "content": content
            })
        })
        .collect();

    json!({
        "model": "gpt-4",
        "messages": msgs,
        "temperature": 0.7,
        "max_tokens": 1000
    })
}

#[must_use] 
pub fn openai_chat_response(content: &str) -> Value {
    json!({
        "id": format!("chatcmpl-{}", uuid::Uuid::new_v4()),
        "object": "chat.completion",
        "created": chrono::Utc::now().timestamp(),
        "model": "gpt-4",
        "choices": [{
            "index": 0,
            "message": {
                "role": "assistant",
                "content": content
            },
            "finish_reason": "stop"
        }],
        "usage": {
            "prompt_tokens": 50,
            "completion_tokens": 100,
            "total_tokens": 150
        }
    })
}

#[must_use] 
pub fn openai_embedding_response(dimensions: usize) -> Value {
    let embedding: Vec<f64> = (0..dimensions)
        .map(|i| (i as f64) / (dimensions as f64))
        .collect();

    json!({
        "object": "list",
        "data": [{
            "object": "embedding",
            "embedding": embedding,
            "index": 0
        }],
        "model": "text-embedding-ada-002",
        "usage": {
            "prompt_tokens": 10,
            "total_tokens": 10
        }
    })
}

#[must_use] 
pub fn sample_kb_entries() -> Vec<KBEntry> {
    vec![
        KBEntry {
            id: "kb-001".to_string(),
            title: "Product Overview".to_string(),
            content: "Our product is a comprehensive solution for business automation.".to_string(),
            category: Some("products".to_string()),
            tags: vec!["product".to_string(), "overview".to_string()],
        },
        KBEntry {
            id: "kb-002".to_string(),
            title: "Pricing Plans".to_string(),
            content: "We offer three pricing plans: Basic ($29/mo), Pro ($79/mo), and Enterprise (custom).".to_string(),
            category: Some("pricing".to_string()),
            tags: vec!["pricing".to_string(), "plans".to_string()],
        },
        KBEntry {
            id: "kb-003".to_string(),
            title: "Support Hours".to_string(),
            content: "Our support team is available 24/7 for Enterprise customers and 9-5 EST for other plans.".to_string(),
            category: Some("support".to_string()),
            tags: vec!["support".to_string(), "hours".to_string()],
        },
        KBEntry {
            id: "kb-004".to_string(),
            title: "Return Policy".to_string(),
            content: "We offer a 30-day money-back guarantee on all plans. No questions asked.".to_string(),
            category: Some("policy".to_string()),
            tags: vec!["returns".to_string(), "refund".to_string(), "policy".to_string()],
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KBEntry {
    pub id: String,
    pub title: String,
    pub content: String,
    pub category: Option<String>,
    pub tags: Vec<String>,
}

#[must_use] 
pub fn sample_products() -> Vec<Product> {
    vec![
        Product {
            sku: "SKU-001".to_string(),
            name: "Widget Pro".to_string(),
            description: "Premium quality widget with advanced features".to_string(),
            price: 99.99,
            in_stock: true,
            category: "widgets".to_string(),
        },
        Product {
            sku: "SKU-002".to_string(),
            name: "Widget Basic".to_string(),
            description: "Entry level widget for beginners".to_string(),
            price: 29.99,
            in_stock: true,
            category: "widgets".to_string(),
        },
        Product {
            sku: "SKU-003".to_string(),
            name: "Gadget X".to_string(),
            description: "Revolutionary gadget with cutting-edge technology".to_string(),
            price: 199.99,
            in_stock: false,
            category: "gadgets".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Product {
    pub sku: String,
    pub name: String,
    pub description: String,
    pub price: f64,
    pub in_stock: bool,
    pub category: String,
}

#[must_use] 
pub fn sample_faqs() -> Vec<FAQ> {
    vec![
        FAQ {
            id: 1,
            question: "How do I reset my password?".to_string(),
            answer: "You can reset your password by clicking 'Forgot Password' on the login page.".to_string(),
            category: "account".to_string(),
        },
        FAQ {
            id: 2,
            question: "What payment methods do you accept?".to_string(),
            answer: "We accept all major credit cards, PayPal, and bank transfers.".to_string(),
            category: "billing".to_string(),
        },
        FAQ {
            id: 3,
            question: "How do I contact support?".to_string(),
            answer: "You can reach our support team via email at support@example.com or through live chat.".to_string(),
            category: "support".to_string(),
        },
        FAQ {
            id: 4,
            question: "Can I cancel my subscription?".to_string(),
            answer: "Yes, you can cancel your subscription at any time from your account settings.".to_string(),
            category: "billing".to_string(),
        },
    ]
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FAQ {
    pub id: u32,
    pub question: String,
    pub answer: String,
    pub category: String,
}

pub mod errors {
    use serde_json::{json, Value};

    #[must_use] 
    pub fn validation_error(field: &str, message: &str) -> Value {
        json!({
            "error": {
                "type": "validation_error",
                "message": format!("Validation failed for field '{}'", field),
                "details": {
                    "field": field,
                    "message": message
                }
            }
        })
    }

    #[must_use] 
    pub fn not_found(resource: &str, id: &str) -> Value {
        json!({
            "error": {
                "type": "not_found",
                "message": format!("{} with id '{}' not found", resource, id)
            }
        })
    }

    #[must_use] 
    pub fn unauthorized() -> Value {
        json!({
            "error": {
                "type": "unauthorized",
                "message": "Authentication required"
            }
        })
    }

    #[must_use] 
    pub fn forbidden() -> Value {
        json!({
            "error": {
                "type": "forbidden",
                "message": "You don't have permission to access this resource"
            }
        })
    }

    #[must_use] 
    pub fn rate_limited(retry_after: u32) -> Value {
        json!({
            "error": {
                "type": "rate_limit_exceeded",
                "message": "Too many requests",
                "retry_after": retry_after
            }
        })
    }

    #[must_use] 
    pub fn internal_error() -> Value {
        json!({
            "error": {
                "type": "internal_error",
                "message": "An unexpected error occurred"
            }
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sample_config() {
        let config = sample_config();
        assert!(config.contains_key("llm-model"));
        assert_eq!(config.get("llm-model"), Some(&"gpt-4".to_string()));
    }

    #[test]
    fn test_whatsapp_text_message() {
        let payload = whatsapp_text_message("15551234567", "Hello");
        assert_eq!(payload["object"], "whatsapp_business_account");
        assert!(payload["entry"][0]["changes"][0]["value"]["messages"][0]["text"]["body"]
            .as_str()
            .unwrap()
            .contains("Hello"));
    }

    #[test]
    fn test_teams_message_activity() {
        let activity = teams_message_activity("user-1", "Test User", "Hello");
        assert_eq!(activity["type"], "message");
        assert_eq!(activity["text"], "Hello");
        assert_eq!(activity["channelId"], "msteams");
    }

    #[test]
    fn test_openai_chat_response() {
        let response = openai_chat_response("Hello, how can I help?");
        assert_eq!(response["object"], "chat.completion");
        assert_eq!(
            response["choices"][0]["message"]["content"],
            "Hello, how can I help?"
        );
    }

    #[test]
    fn test_sample_kb_entries() {
        let entries = sample_kb_entries();
        assert!(!entries.is_empty());
        assert!(entries.iter().any(|e| e.category == Some("products".to_string())));
    }

    #[test]
    fn test_sample_products() {
        let products = sample_products();
        assert_eq!(products.len(), 3);
        assert!(products.iter().any(|p| p.sku == "SKU-001"));
    }

    #[test]
    fn test_error_responses() {
        let validation = errors::validation_error("email", "Invalid email format");
        assert_eq!(validation["error"]["type"], "validation_error");

        let not_found = errors::not_found("User", "123");
        assert_eq!(not_found["error"]["type"], "not_found");
    }
}
