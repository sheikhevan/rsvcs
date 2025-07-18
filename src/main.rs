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
    verbose: bool,

    #[arg(short, long)]
    add: Option<String>,

    #[arg(short, long)]
    commit: Option<String>,
}

fn main() {
    let args = Cli::parse();
}
