


#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]


    #[test]


    fn test_default_config() {
        let config = WizardConfig::default();
        assert_eq!(config.llm_provider, LlmProvider::None);
        assert!(!config.components.is_empty());
    }

    #[test]


    fn test_slug_generation() {
        let mut config = WizardConfig::default();
        config.organization.name = "My Test Company".to_string();
        config.organization.slug = config
            .organization
            .name
            .to_lowercase()
            .replace(' ', "-")
            .chars()
            .filter(|c| c.is_alphanumeric() || *c == '-')
            .collect();

        assert_eq!(config.organization.slug, "my-test-company");
    }