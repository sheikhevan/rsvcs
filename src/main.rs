use clap::Parser;
use std::{env, fs::File, path::Path};
mod parse_config;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = default_config_path())]
    config: String,
}

fn default_config_path() -> String {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());
    format!("{}/.config/rsvcs/config.yaml", home)
}

fn main() {
    let args = Args::parse();
    let path = Path::new(&args.config);

    match path.exists() && matches!(path.extension(), Some(ext) if ext == "yaml" || ext == "yml") {
        true => {
            println!("Config exists!");
            match parse_config::parse_yaml(&args.config) {
                Ok((config)) => {
                    println!("Storage: {}", config.storage);
                    println!("Items: {:?}", config.items);
                }
                Err(e) => println!("Error parsing config: {}", e),
            }
        }
        false => {
            println!("Config does not exist or is not a YAML");
            println!("Creating default config at {}", default_config_path());
            let _ = File::create(default_config_path());
        }
    }
}
