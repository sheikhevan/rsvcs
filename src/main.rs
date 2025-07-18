use clap::{Parser, Subcommand};
mod add;
mod init;

#[derive(Parser, Debug)]
#[command(
    name = "rsvcs",
    version = "0.0.1",
    about = "Simple version control system written in Rust"
)]

struct Cli {
    #[arg(short, long)]
    verbose: bool,

    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    Init,
    Add { files: Vec<String> },
    Commit { message: String },
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            println!("Initializing repository");
            match init::init() {
                Ok(()) => println!("Repository successfully initiated"),
                Err(e) => eprintln!("There was an error creating the repository: {}", e),
            };
        }
        Commands::Add { files } => {
            for file in files {
                if args.verbose {
                    println!("Adding files: {:?}", file);
                }
                add::add_to_repo(file);
            }
        }
        Commands::Commit { message } => {
            println!("Commiting with message {:?}", message);
        }
    }
}
