//! Unit tests migrated from src/basic/keywords/crm/attendance.rs
//! These tests were originally in botserver and have been migrated to bottest.

use botserver::basic::keywords::crm::attendance::create_fallback_tips;
use rhai::Map;

#[test]
fn test_fallback_tips_urgent() {
    let tips = create_fallback_tips("This is URGENT! Help now!");
    let result = tips.try_cast::<Map>().unwrap();
    assert!(result.get("success").unwrap().as_bool().unwrap());
}

#[test]
fn test_fallback_tips_question() {
    let tips = create_fallback_tips("Can you help me with this?");
    let result = tips.try_cast::<Map>().unwrap();
    assert!(result.get("success").unwrap().as_bool().unwrap());
}

#[test]
fn test_polish_message() {
    let polished = polish_text("thx 4 ur msg", "professional");
    assert!(!polished.contains("thx"));
    assert!(polished.contains("your"));
}

#[test]
fn test_polish_message_capitalization() {
    let polished = polish_text("hello there", "professional");
    assert!(polished.starts_with('H'));
    assert!(polished.ends_with('.'));
}

fn polish_text(message: &str, _tone: &str) -> String {
    let mut polished = message.to_string();
    polished = polished
        .replace("thx", "Thank you")
        .replace("u ", "you ")
        .replace(" u", " you")
        .replace("ur ", "your ")
        .replace("ill ", "I'll ")
        .replace("dont ", "don't ")
        .replace("cant ", "can't ")
        .replace("wont ", "won't ")
        .replace("im ", "I'm ")
        .replace("ive ", "I've ");
    if let Some(first_char) = polished.chars().next() {
        polished = first_char.to_uppercase().to_string() + &polished[1..];
    }
    if !polished.ends_with('.') && !polished.ends_with('!') && !polished.ends_with('?') {
        polished.push('.');
    }
    polished
}

#[test]
fn test_sentiment_positive() {
    let result = analyze_text_sentiment("Thank you so much! This is great!");
    assert_eq!(result, "positive");
}

#[test]
fn test_sentiment_negative() {
    let result = analyze_text_sentiment("This is terrible! I'm so frustrated!");
    assert_eq!(result, "negative");
}

#[test]
fn test_sentiment_neutral() {
    let result = analyze_text_sentiment("The meeting is at 3pm.");
    assert_eq!(result, "neutral");
}

fn analyze_text_sentiment(message: &str) -> &'static str {
    let msg_lower = message.to_lowercase();
    let positive_words = [
        "thank", "great", "perfect", "awesome", "excellent", "good", "happy", "love",
    ];
    let negative_words = [
        "angry", "frustrated", "terrible", "awful", "horrible", "hate", "disappointed", "problem",
        "issue",
    ];
    let positive_count = positive_words
        .iter()
        .filter(|w| msg_lower.contains(*w))
        .count();
    let negative_count = negative_words
        .iter()
        .filter(|w| msg_lower.contains(*w))
        .count();
    if positive_count > negative_count {
        "positive"
    } else if negative_count > positive_count {
        "negative"
    } else {
        "neutral"
    }
}

#[test]
fn test_smart_replies_count() {
    let replies = generate_smart_replies();
    assert_eq!(replies.len(), 3);
}

#[test]
fn test_smart_replies_content() {
    let replies = generate_smart_replies();
    assert!(replies.iter().any(|r| r.contains("Thank you")));
    assert!(replies.iter().any(|r| r.contains("understand")));
}

fn generate_smart_replies() -> Vec<String> {
    vec![
        "Thank you for reaching out! I'd be happy to help you with that.".to_string(),
        "I understand your concern. Let me look into this for you right away.".to_string(),
        "Is there anything else I can help you with today?".to_string(),
    ]
}
