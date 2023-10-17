use serde::{Deserialize, Serialize};
use std::fs;

#[derive(Serialize, Deserialize)]
pub struct PostConfig {
    pub class: String,
    pub descendant: String,
    pub attr: String,
}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub pages: Vec<String>,
    pub post: PostConfig,
    pub image: PostConfig,
    pub webhook_url: String,
    pub author: Author,
    pub character: Character,
}

#[derive(Serialize, Deserialize)]
pub struct PageConfig {
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct Author {
    pub class: String,
    pub descendant: String,
}

#[derive(Serialize, Deserialize)]
pub struct Character {
    pub class: String,
    pub descendant: String,
}

pub fn load_config_from_file(file_path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let contents = fs::read_to_string(file_path)?;
    let config: Config = serde_json::from_str(&contents).map_err(|e| {
        println!("Deserialization error: {}", e);
        Box::new(e) as Box<dyn std::error::Error>
    })?;
    Ok(config)
}
