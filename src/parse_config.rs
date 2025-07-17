// use std::env;
use std::fs::File;
use std::io::{ErrorKind, Read};
// use toml;

fn open_config(path: &str) -> File {
    let config = match File::open(path) {
        Ok(file) => file,
        Err(e) => match e.kind() {
            ErrorKind::NotFound => match File::create(path) {
                Ok(new_file) => {
                    println!("Config file not found, creating an empty one.");
                    new_file
                }
                Err(error) => panic!("There was an error creating the config: {error:?}"),
            },
            _ => panic!("An error occurred: {e:?}"),
        },
    };
    config
}

pub fn parse_config(config_path: &str) {
    let mut file = open_config(config_path);
    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Failed to read config");

    println!("{}", contents)
}
