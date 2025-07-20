use crate::utils::Repository;
use clap::{Parser, Subcommand};
use std::env;
mod add;
mod commit;
mod init;
mod log;
mod pull;
mod utils;

#[derive(Parser, Debug)]
#[command(
    name = "rsvcs",
    version = "0.0.1",
    about = "Super simple backup utility with version control elements written in Rust"
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
    Pull {
        #[command(subcommand)]
        pull_command: PullCommands,
    },
}

#[derive(Subcommand, Debug)]
enum LogCommands {
    Print,
}

#[derive(Subcommand, Debug)]
enum PullCommands {
    Latest,
    #[command(name = "hash")]
    Hash {
        hash: String,
    },
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
                    match repo.print_log() {
                        Ok(()) => {}
                        Err(e) => eprintln!("Error: {}", e),
                    };
                }
            }
        }

        Commands::Pull { pull_command } => {
            let repo = match Repository::open() {
                Ok(repo) => repo,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    return;
                }
            };
            match pull_command {
                PullCommands::Latest => {
                    if args.verbose {
                        println!("Pulling latest commit")
                    }
                    match repo.pull_latest() {
                        Ok(()) => println!("Successfully pulled latest commit"),
                        Err(e) => eprintln!("Error: {}", e),
                    }
                }
                PullCommands::Hash { hash } => {
                    if args.verbose {
                        println!("Pulling commit with hash {}", &hash)
                    }
                    match repo.pull_hash(&hash) {
                        Ok(()) => println!("Successfully pulled commit {}", hash),
                        Err(e) => println!("Error: {}", e),
                    }
                }
            }
        }
    }
}
