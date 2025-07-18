use crate::utils::Repository;
use clap::{Parser, Subcommand};
use std::env;
mod add;
mod init;
mod utils;

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
            let repo = Repository::new(env::current_dir().unwrap());
            match repo.init() {
                Ok(()) => println!("Repo successfully initialized!"),
                Err(e) => eprintln!("Repo initialzation failed: {e}"),
            }
        }
        Commands::Add { files } => {
            let repo = match Repository::open() {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            for file in files {
                if args.verbose {
                    println!("Adding files: {:?}", file);
                }
                if let Err(e) = repo.add(&file) {
                    eprintln!("Error adding {}: {}", file, e);
                }
            }
        }
        Commands::Commit { message } => {
            let repo = match Repository::open() {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            }
            let staging_status = match repo.is_staging_empty() {
                Ok(true) => true,
                Err(e) => panic!("There was an error accessing `staging`: {}", e),
            }
            println!("Commiting with message {:?}", message);
        }
    }
}
