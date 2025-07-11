use std::fs;
use yaml_rust2::{self, YamlLoader};

#[derive(Debug, Clone)]
pub struct Config {
    pub storage: String,
    pub items: Vec<String>,
}

pub fn parse_yaml(path: &str) -> Result<Config, Box<dyn std::error::Error>> {
    let config_string = fs::read_to_string(path)?;
    let configs = YamlLoader::load_from_str(&config_string)?;
    let config = &configs[0];

    let storage = config["storage"]
        .as_str()
        .ok_or("Missing storage field")?
        .to_string();

    let items = config["items"]
        .as_vec()
        .ok_or("Missing items field")?
        .iter()
        .filter_map(|item| item.as_str())
        .map(|s| s.to_string())
        .collect();

    Ok(Config { storage, items })
}
