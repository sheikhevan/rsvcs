use clap::Parser;
use std::env;
use std::fs::File;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "rsvcs",
    version = "0.0.1",
    about = "simple version control system written in Rust"
)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn get_default_config_path() -> String {
    let home = env::var("HOME").unwrap_or_else(|_| ".".to_string());

    let config_dir = PathBuf::from(home).join(".config").join("rsvcs");

    let yml_path = config_dir.join("config.yml");
    let yaml_path = config_dir.join("config.yaml");

    if yml_path.exists() {
        yml_path.to_string_lossy().to_string()
    } else {
        yaml_path.to_string_lossy().to_string()
    }
}

fn main() {
    let args = Cli::parse();

    let config = match args.config.as_deref() {
        Some(s) => s,
        None => match  {
            
        },
    };

    if args.verbose {
        println!("verbose mode enabled!");
    }
}
