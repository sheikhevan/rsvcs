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
    let path = Path::new(&args.config);

    match path.exists() && matches!(path.extension(), Some(ext) if ext == "yaml" || ext == "yml") {
        true => {
            println!("Config exists!");
        }
        false => {
            println!("Config does not exist or is not a YAML");
            println!(
                "Creating default config located at {}",
                default_config_path()
            );
            let _ = File::create(default_config_path());
        }
    }
}
