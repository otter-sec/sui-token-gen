use clap::{Parser, Subcommand};
use tokio;

mod commands;
mod utils;

#[derive(Parser, Debug)]
#[command(
    author = "Osec", 
    version = "1.0.0", 
    about = "Create and verify Sui tokens contracts",
    long_about = "Sui Token Generator is a CLI tool that helps you create and verify tokens contracts."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Creates a new token contract.")]
    Create,
    #[command(about = "Verifies an existing contract from repo or local.")]
    Verify {
        path: String,
    },
}

#[tokio::main]
async fn main() {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create => {
            commands::create::create_token().await;
        }
        Commands::Verify { path } => {
            commands::verify::verify_token(path).await;
        }
    }
}
