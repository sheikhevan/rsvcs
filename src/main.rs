use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value = "~/.config/rsbackup/config")]
    config: String,
}
fn main() {
    let args = Args::parse();

    println!("Config file located at -> {}", args.config);
}
