use serde::Deserialize;
use std::fs;

#[derive(Deserialize)]
pub struct Config {
    pub url: String,
    pub webhook_url: String,
    pub antitags: Vec<String>,
    pub tags: Vec<String>,
}

impl Config {
    pub fn load() -> Config {
        let config_content = fs::read_to_string("config.json").expect("Error reading config file");
        serde_json::from_str(&config_content).expect("Error parsing config JSON")
    }
}
