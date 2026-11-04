use std::collections::HashMap;
use std::fs;
use std::error::Error;

pub struct ConfigurationManager {

    configurations: HashMap<String, serde_json::Value>

}

impl ConfigurationManager {

    pub fn new(config_file: Option<&str>) -> Result<Self, Box<dyn Error>> {
       
        let file: &str =config_file.unwrap_or("src/configs/DefaultConfigsFile.json");

        println!("Loading Server Configurations...");

        let contents = fs::read_to_string(file)?;

        let configurations: HashMap<String, serde_json::Value> = serde_json::from_str(&contents)?;

        let mut instance = Self {
            configurations,
        };

        instance.clean_configurations();
        instance.apply_configurations_defaults();
        
        println!("Done Loading Server Configurations...");

        Ok(instance)
    }

    pub fn apply_configurations_defaults(&mut self) {
        self.configurations
            .entry("host".to_string())
            .or_insert(serde_json::json!("127.0.0.1"));

        self.configurations
            .entry("port".to_string())
            .or_insert(serde_json::json!("0"));
    }

    pub fn get_value(&self, key: &str) -> Option<&serde_json::Value> {
        self.configurations.get(key)
    }

    pub fn clean_configurations(&mut self){

        let allowed_configs = ["host", "port"];

        let invalid_keys: Vec<String> = self.configurations
        .keys()
        .filter(|key| !allowed_configs.contains(&key.as_str()))
        .cloned()
        .collect();

        for key in invalid_keys {
            self.configurations.remove(&key);
        }
    }


}