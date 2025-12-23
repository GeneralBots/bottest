//! Unit tests migrated from src/basic/keywords/card.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_card_style_from_string() {
        assert!(matches!(CardStyle::from("minimal"), CardStyle::Minimal));
        assert!(matches!(CardStyle::from("VIBRANT"), CardStyle::Vibrant));
        assert!(matches!(CardStyle::from("dark"), CardStyle::Dark));
        assert!(matches!(CardStyle::from("unknown"), CardStyle::Modern));
    }

    #[test]

    
    fn test_card_dimensions_for_style() {
        let story_dims = CardDimensions::for_style(&CardStyle::Story);
        assert_eq!(story_dims.width, 1080);
        assert_eq!(story_dims.height, 1920);

        let square_dims = CardDimensions::for_style(&CardStyle::Modern);
        assert_eq!(square_dims.width, 1080);
        assert_eq!(square_dims.height, 1080);
    }

    #[test]

    
    fn test_card_config_default() {
        let config = CardConfig::default();
        assert!(matches!(config.style, CardStyle::Modern));
        assert!(config.include_hashtags);
        assert!(config.include_caption);
        assert!(config.brand_watermark.is_none());
    }