use clap::{Parser, Subcommand};
use errors::TokenGenErrors;

mod commands;
mod errors;
mod rpc_client;
mod utils;
mod variables;

use rpc_client::{initiate_client, TokenGenClient};
use variables::ADDRESS;

#[cfg(test)]
pub mod tests;

#[derive(Parser, Debug)]
#[command(
    author = "Osec",
    version = "1.0.0",
    about = "Create and verify Sui Coin contracts",
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
        /// Path to the file
        #[arg(short, long)]
        path: Option<String>,

        /// URL to fuzz
        #[arg(short, long)]
        url: Option<String>,
    },
}

// Define Return type for main function as Result<T, TokenGenErrors>
pub type Result<T> = std::result::Result<T, TokenGenErrors>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create => {
            // Initialize the RPC client for create command
            let client: TokenGenClient = match initiate_client(ADDRESS).await {
                Ok(client) => client,
                Err(e) => {
                    let error = TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e));
                    error.log();
                    std::process::exit(1);
                }
            };

            if let Err(error) = commands::create::create_token(client).await {
                error.log();
                std::process::exit(1);
            }
        }
        Commands::Verify { path, url } => {
            if path.is_none() && url.is_none() {
                let error = TokenGenErrors::InvalidInput(
                    "Error: Either --path or --url must be provided.".to_string(),
                );
                error.log();
                std::process::exit(1);
            }

            // Check path validity before initializing client
            if let Some(path) = path {
                if !std::path::Path::new(path).exists() {
                    let error = TokenGenErrors::InvalidPath("The provided path for the contract is invalid.".to_string());
                    error.log();
                    std::process::exit(1);
                }
            }

            // Initialize the RPC client for verify command
            let client: TokenGenClient = match initiate_client(ADDRESS).await {
                Ok(client) => client,
                Err(e) => {
                    let error = TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e));
                    error.log();
                    std::process::exit(1);
                }
            };

            if let Some(path) = path {
                if let Err(error) = commands::verify::verify_token_from_path(path, client.clone()).await {
                    error.log();
                    std::process::exit(1);
                }
            }

            if let Some(url) = url {
                if let Err(error) = commands::verify::verify_token_using_url(url, client).await {
                    error.log();
                    std::process::exit(1);
                }
            }
        }
    }
    Ok(())
}
