


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

use rhai::Dynamic;



    #[test]


    fn test_calculate_lead_score_empty() {
        let lead_data = Map::new();
        let score = calculate_lead_score(&lead_data, None);
        assert_eq!(score, 0);
    }

    #[test]


    fn test_calculate_lead_score_basic() {
        let mut lead_data = Map::new();
        lead_data.insert("job_title".into(), Dynamic::from("CEO"));
        lead_data.insert("company_size".into(), Dynamic::from(500_i64));
        lead_data.insert("email".into(), Dynamic::from("ceo@company.com"));

        let score = calculate_lead_score(&lead_data, None);
        assert!(score > 30);
    }

    #[test]


    fn test_calculate_lead_score_with_title() {
        let mut lead_data = Map::new();
        lead_data.insert("job_title".into(), Dynamic::from("CTO"));

        let score = calculate_lead_score(&lead_data, None);
        assert!(score >= 30);
    }

    #[test]


    fn test_determine_priority() {
        assert_eq!(determine_priority(95), "CRITICAL");
        assert_eq!(determine_priority(75), "HIGH");
        assert_eq!(determine_priority(55), "MEDIUM");
        assert_eq!(determine_priority(35), "LOW");
        assert_eq!(determine_priority(10), "MINIMAL");
    }

    #[test]


    fn test_score_clamping() {
        let mut lead_data = Map::new();
        lead_data.insert("budget".into(), Dynamic::from(1000000_i64));

        let score = calculate_lead_score(&lead_data, None);
        assert!(
            score <= 100,
            "Score should be clamped to 100, got {}",
            score
        );
    }