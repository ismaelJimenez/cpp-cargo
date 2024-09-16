use clap::{Parser, Subcommand};
use std::error::Error;

#[derive(clap::ValueEnum, Clone, Debug)]
enum Command {
    New,
    Run,
}

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new project
    New {
        /// The folder name for the new project
        folder_name: String,
    },
    /// Run the project
    Run,
}

fn main() -> Result<(), Box<dyn Error>> {
    let args = Args::parse();

    match args.command {
        Commands::New { folder_name } => {
            println!(
                "\x1b[1;32mCreated\x1b[0m binary (application) `{}` package",
                folder_name
            );
        }
        Commands::Run => {
            println!("Running the project");
        }
    }

    Ok(())
}
