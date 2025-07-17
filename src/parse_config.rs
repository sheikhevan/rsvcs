use std::env;
use std::fs::File;
use std::io::{ErrorKind, Read, Result};
// use toml;

fn get_config_path(cli_config_path: Option<String>) -> String {
    let default_config_path = format!(
        "{}/.config/rsvcs/config.toml",
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );

    let config_path = match cli_config_path.as_deref() {
        Some(s) => s.to_string(),
        None => default_config_path,
    };

    config_path
}

fn open_config(path: &str) -> Result<File> {
    let config = match File::open(path) {
        Ok(file) => Ok(file),
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(_) => {
                    println!("Config file not found, creating an empty one at: {}", path);
                    File::open(path)
                }
                Err(error) => panic!("There was an error creating the config: {error:?}"),
            },
            _ => panic!("An error occurred: {e:?}"),
        },
    };
    config
}

pub fn parse_config(cli_config_path: Option<String>) {
    let config_path = get_config_path(cli_config_path);

    let mut config_result = match open_config(&config_path) {
        Ok(path) => path,
        Err(e) => panic!("There was an error opening the config: {e:?}"),
    };

    let mut contents = String::new();

    config_result
        .read_to_string(&mut contents)
        .expect("Failed to read config");

    println!("{}", contents)
}
