use std::panic::resume_unwind;
use clap::{Parser, Subcommand};
use minigit::Repository;

#[derive(Parser)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Init,
    Add { name: String },
    Remove { name: String },
    Commit { name: String },
    Checkout { version: u32 },
    History { lines: Option<u32> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing minigit...");
            let repository = Repository::new();
            repository.serialize();
        }
        Commands::Add { name } => {
            println!("Adding file {name}");
        }
        Commands::Remove { name } => {
            println!("Removing file {name}");
        }
        Commands::Commit { name } => {
            println!("Commiting file {name}");
        }
        Commands::Checkout { version } => {
            println!("Restoring to version {version}");
        }
        Commands::History { lines } => {
            match lines {
                Some(lines) => println!("Printing last {lines} lines of history"),
                None => println!("Printing history")
            }
        }
    }
}
