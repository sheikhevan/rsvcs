use clap::Parser;
use std::path::Path;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "/home/evan/.config/rsvcs/config.yaml")]
    config: String,
}
fn main() {
    let args = Args::parse();

    match Path::new(&args.config).exists() {
        true => {
            println!("Config file located at -> {}", args.config);
        }
        false => {
            println!("Config file does not exist!");
        }
    }
}
