//! Unit tests migrated from src/security/antivirus.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_classify_threat() {
        assert_eq!(
            AntivirusManager::classify_threat("Win.Trojan.Generic"),
            "Trojan"
        );
        assert_eq!(
            AntivirusManager::classify_threat("Ransomware.WannaCry"),
            "Ransomware"
        );
        assert_eq!(
            AntivirusManager::classify_threat("PUP.Optional.Adware"),
            "PUP"
        );
        assert_eq!(
            AntivirusManager::classify_threat("Unknown.Malware"),
            "Malware"
        );
    }

    #[test]

    
    fn test_assess_severity() {
        assert_eq!(
            AntivirusManager::assess_severity("Ransomware.Test"),
            ThreatSeverity::Critical
        );
        assert_eq!(
            AntivirusManager::assess_severity("Trojan.Generic"),
            ThreatSeverity::High
        );
        assert_eq!(
            AntivirusManager::assess_severity("Virus.Test"),
            ThreatSeverity::Medium
        );
        assert_eq!(
            AntivirusManager::assess_severity("PUP.Adware"),
            ThreatSeverity::Low
        );
    }

    #[tokio::test]
    async fn test_antivirus_manager_creation() {
        let config = AntivirusConfig::default();
        let manager = AntivirusManager::new(config);
        assert!(manager.is_ok());
    }