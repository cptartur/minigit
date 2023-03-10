use clap::{Parser, Subcommand};
use minigit::repository::Repository;

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
    Commit { message: String },
    Checkout { version: u32 },
    History { lines: Option<u32> },
}

fn main() {
    let cli = Cli::parse();

    match cli.command {
        Commands::Init => {
            println!("Initializing minigit...");
            let repository = Repository::create().unwrap();
            repository.save();
        }
        Commands::Add { name } => {
            println!("Adding file {name}");
            let mut repository = Repository::load().unwrap();
            repository.add(&name, None);
            repository.save();
        }
        Commands::Remove { name } => {
            println!("Removing file {name}");
            let mut repository = Repository::load().unwrap();
            repository.remove(&name);
            repository.save();
        }
        Commands::Commit { message } => {
            println!("Committing with message {message}");
            let mut repository = Repository::load().unwrap();
            repository.commit(Some(&message));
            repository.save();
        }
        Commands::Checkout { version } => {
            println!("Restoring to version {version}");
            let repository = Repository::load().unwrap();
            repository.checkout(version);
        }
        Commands::History { lines } => {
            let repository = Repository::load().unwrap();
            repository.history(lines);
        }
    }
}
