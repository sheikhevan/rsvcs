use clap::Parser;
use std::env;
mod parse_config;

#[derive(Parser, Debug)]
#[command(
    name = "rsvcs",
    version = "0.0.1",
    about = "Simple version control system written in Rust"
)]
struct Cli {
    #[arg(short, long)]
    config: Option<String>,

    #[arg(short, long)]
    verbose: bool,
}

fn main() {
    let args = Cli::parse();

    let default_config_path = format!(
        "{}/.config/rsvcs/config.toml",
        env::var("HOME").unwrap_or_else(|_| ".".to_string())
    );

    let config_path = match args.config.as_deref() {
        Some(s) => s.to_string(),
        None => default_config_path,
    };

    parse_config::parse_config(&config_path);

    if args.verbose {
        println!("Verbose mode enabled!");
        println!("Using config file: {}", config_path);
    }
}
