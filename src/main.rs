use clap::Parser;
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

    let config = parse_config::parse_config(args.config);

    if config.general.verbose || args.verbose {
        println!("Verbose mode enabled!");
    }
}
