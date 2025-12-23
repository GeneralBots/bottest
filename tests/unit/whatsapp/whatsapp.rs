//! Unit tests migrated from src/whatsapp/mod.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_extract_text_message() {
        let message = WhatsAppMessage {
            id: "msg123".to_string(),
            from: "+1234567890".to_string(),
            timestamp: "1234567890".to_string(),
            message_type: "text".to_string(),
            text: Some(WhatsAppText {
                body: "Hello, world!".to_string(),
            }),
            image: None,
            audio: None,
            video: None,
            document: None,
            location: None,
            interactive: None,
            button: None,
        };

        let content = extract_message_content(&message);
        assert_eq!(content, "Hello, world!");
    }

    #[test]

    
    fn test_extract_interactive_button() {
        let message = WhatsAppMessage {
            id: "msg123".to_string(),
            from: "+1234567890".to_string(),
            timestamp: "1234567890".to_string(),
            message_type: "interactive".to_string(),
            text: None,
            image: None,
            audio: None,
            video: None,
            document: None,
            location: None,
            interactive: Some(WhatsAppInteractive {
                interactive_type: "button_reply".to_string(),
                button_reply: Some(WhatsAppButtonReply {
                    id: "btn1".to_string(),
                    title: "Yes".to_string(),
                }),
                list_reply: None,
            }),
            button: None,
        };

        let content = extract_message_content(&message);
        assert_eq!(content, "Yes");
    }