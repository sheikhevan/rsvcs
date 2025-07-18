use clap::Parser;

mod backup;
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
        println!(
            "What place(s) Should the Files Go? {:?}",
            config.general.destinations
        );
        println!("Number of Copies? {:?}", config.general.number_of_copies);
        println!("Compression? {:?}", config.general.compression);
        println!("Compression Level? {:?}", config.general.compression_level);
        println!("Files/Directories to Copy? {:?}", config.targets.sources);
        println!(
            "Files/Directories to Exclude? {:?}",
            config.targets.exclusions
        );
    }

    backup::backup(&config.general.destinations, &config.targets.sources);
}
