use relay_web_server::configs::configuration_manager::ConfigurationManager;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_configurations(){
        
        let json = r#"{
            "host": "127.0.0.1",
            "port": "8080"
        }"#;

        let file = tempfile::NamedTempFile::new().unwrap();

        std::fs::write(file.path(), json).unwrap();

        let result = ConfigurationManager::new(
            Some(file.path().to_str().unwrap())
        );

        assert!(result.is_ok());

    }

    #[test]
    fn fail_to_parse_configurations(){
        
        let json = r#"{
            "host": "127.0.0.1"
            "port": "8080"
        }"#;

        let file = tempfile::NamedTempFile::new().unwrap();

        std::fs::write(file.path(), json).unwrap();

        let result = ConfigurationManager::new(
            Some(file.path().to_str().unwrap())
        );

        assert!(result.is_err());

    }

    #[test]
    fn test_cleaning_functionality(){
        
        let json = r#"{
            "host": "127.0.0.1",
            "port": "8080",
            "ShoudBeDeleted": true
        }"#;

        let file = tempfile::NamedTempFile::new().unwrap();

        std::fs::write(file.path(), json).unwrap();

        let result = ConfigurationManager::new(
            Some(file.path().to_str().unwrap())
        );

        let mut instance:ConfigurationManager = result.unwrap();

        instance.clean_configurations();

        instance.get_value("ShoudBeDeleted");

        assert!(instance.get_value("ShoudBeDeleted").is_none());
        assert!(instance.get_value("host").is_some());
        assert!(instance.get_value("port").is_some());


    }

    #[test]
    fn test_defaults_configurations_functionality(){
        
        let manager = ConfigurationManager::new(None).unwrap();

        assert!(manager.get_value("host").is_some());
        assert!(manager.get_value("port").is_some());

    }
    
    #[test]
    fn test_default_configuration_values() {

        let manager = ConfigurationManager::new(None).unwrap();

        assert_eq!(
            manager.get_value("host").unwrap(),
            &serde_json::json!("127.0.0.1")
        );

    }

}