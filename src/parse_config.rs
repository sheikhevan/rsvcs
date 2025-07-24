use std::{
    error::Error,
    fs::File,
    io::{Read, Write},
};

use crate::utils::Repository;
use serde::{Deserialize, Serialize};
use toml::{self, to_string};

#[derive(Debug, Deserialize, Serialize)]
pub struct Config {
    pub general: General,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct General {
    pub verbose: bool,
    pub ignore: Vec<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            general: General {
                verbose: false,
                ignore: Vec::new(),
            },
        }
    }
}

impl Repository {
    pub fn write_default_config(&self) -> Result<(), Box<dyn Error>> {
        let default_config = Config {
            general: General {
                verbose: false,
                ignore: Vec::new(),
            },
        };
        let toml_string = to_string(&default_config)?;
        let mut config = File::create(self.rsvcs.join("config.toml"))?;
        config.write_all(toml_string.as_bytes())?;
        Ok(())
    }
    pub fn read_config(&self) -> Result<Config, Box<dyn Error>> {
        let config_path = self.rsvcs.join("config.toml");
        let mut config = File::open(config_path)?;
        let mut contents = String::new();
        config
            .read_to_string(&mut contents)
            .expect("Failed to read config");
        let config: Config = toml::from_str(&contents)?;
        Ok(config)
    }
}
