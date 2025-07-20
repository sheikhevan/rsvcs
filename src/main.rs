use crate::utils::Repository;
use clap::{Parser, Subcommand};
use std::env;
mod add;
mod commit;
mod init;
mod log;
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
    Add {
        files: Vec<String>,
    },
    Commit {
        message: String,
    },
    Log {
        #[command(subcommand)]
        log_command: LogCommands,
    },
}

#[derive(Subcommand, Debug)]
enum LogCommands {
    Print,
}

fn main() {
    let args = Cli::parse();

    match args.command {
        Commands::Init => {
            if args.verbose {
                println!("Initializing Repository")
            }
            let repo = Repository::new(env::current_dir().unwrap());
            match repo.init() {
                Ok(()) => println!("Repository successfully initialized!"),
                Err(e) => eprintln!("Repository initialization failed: {e}"),
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
            println!("Operation completed successfully")
        }

        Commands::Commit { message } => {
            let mut repo = match Repository::open() {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };

            match repo.is_staging_empty() {
                Ok(true) => {
                    println!("Staging directory is empty. Add files using 'rsvcs add'");
                }
                Ok(false) => {
                    if let Err(e) = repo.commit(&message) {
                        eprintln!("Error: {}", e);
                    }
                }
                Err(e) => {
                    eprintln!("Error checking staging directory: {}", e);
                }
            }
        }

        Commands::Log { log_command } => {
            let repo = match Repository::open() {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            match log_command {
                LogCommands::Print => {
                    let _ = match repo.print_log() {
                        Ok(()) => {}
                        Err(e) => eprintln!("Error: {}", e),
                    };
                }
            }
        }
    }
}
