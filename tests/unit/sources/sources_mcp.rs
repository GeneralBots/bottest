//! Unit tests migrated from src/sources/mcp.rs
//! These tests were originally in botserver and have been migrated to bottest.

#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]
// Original: use super::*; - tests used internal functions from botserver

    #[test]

    
    fn test_server_type_icons() {
        assert_eq!(get_server_type_icon("filesystem"), "📁");
        assert_eq!(get_server_type_icon("database"), "🗄️");
        assert_eq!(get_server_type_icon("github"), "🐙");
        assert_eq!(get_server_type_icon("unknown"), "🔌");
    }

    #[test]

    
    fn test_risk_level_class() {
        assert_eq!(get_risk_level_class(&ToolRiskLevel::Safe), "risk-safe");
        assert_eq!(
            get_risk_level_class(&ToolRiskLevel::Critical),
            "risk-critical"
        );
    }

    #[test]

    
    fn test_risk_level_name() {
        assert_eq!(get_risk_level_name(&ToolRiskLevel::Safe), "Safe");
        assert_eq!(get_risk_level_name(&ToolRiskLevel::High), "High");
    }