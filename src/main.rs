use clap::Parser;
use std::{env, fs::File, path::Path};

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

    match Path::new(&args.config).exists() {
        true => {
            println!("Config file located at -> {}", args.config);
        }
        false => {
            println!("specified config file does not exist.");
            let mut file = File::create(&args.config);
        }
    }
}
