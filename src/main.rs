//! Sui Token Generator CLI Tool
//!
//! This module serves as the entry point for the Sui Token Generator CLI tool.
//! It provides functionality to:
//! - Create new Sui token contracts with customizable parameters
//! - Verify existing token contracts from local files or Git repositories
//!
//! The tool uses a client-server architecture with RPC communication for token
//! operations and verification.

use clap::{Parser, Subcommand};

use commands::{create, verify};
use error_handler::handle_error;
use errors::TokenGenErrors;
use rpc_client::{initiate_client, TokenGenClient};
use variables::ADDRESS;

mod commands;
mod error_handler;
mod errors;
mod rpc_client;
mod success_handler;
#[cfg(test)]
pub mod tests;
mod utils;
mod variables;

/// Main entry point for the Sui Token Generator CLI tool.
///
/// This tool provides functionality to create and verify Sui token contracts.
/// It supports subcommands for token creation and verification, and interacts
/// with an RPC server to execute these operations.
#[derive(Parser, Debug)]
#[command(
    author = "Osec",
    version = "1.0.0",
    about = "Create and verify Sui Coin contracts",
    long_about = "Sui Token Generator is a CLI tool that helps you create and verify tokens contracts."
)]
struct Cli {
    /// Subcommands for the CLI tool (Create or Verify).
    #[command(subcommand)]
    command: Commands,
}

/// Enum representing the subcommands available in the CLI tool.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Creates a new Sui token contract.
    #[command(about = "Creates a new token contract.")]
    Create,
    /// Verifies an existing Sui token contract from a repository or local file.
    #[command(about = "Verifies an existing contract from repo or local.")]
    Verify {
        /// Path to the local contract file to verify.
        #[arg(short, long)]
        path: Option<String>,

        /// URL of the repository (GitHub/GitLab) containing the contract to verify.
        #[arg(short, long)]
        url: Option<String>,
    },
}

/// Custom result type for the application, using `TokenGenErrors` for error handling.
pub type Result<T> = std::result::Result<T, TokenGenErrors>;

/// Main asynchronous entry point for the application.
///
/// Parses command-line arguments, executes the requested subcommand,
/// and handles any resulting errors.
#[tokio::main]
async fn main() {
    // Parse CLI arguments and construct the CLI instance.
    let cli = Cli::parse();

    // Execute the CLI logic and handle errors, if any.
    handle_error(run_cli(cli).await);
}

/// Core logic to execute the selected subcommand.
///
/// # Arguments
/// * `cli` - Parsed CLI arguments containing the selected subcommand.
///
/// # Returns
/// * `Ok(())` if the operation succeeds.
/// * `Err(TokenGenErrors)` if any error occurs during the execution.
async fn run_cli(cli: Cli) -> Result<()> {
    // Initialize the RPC client by connecting to the configured address.
    let client: TokenGenClient = initiate_client(ADDRESS)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))?;

    // Match and execute the selected subcommand.
    match &cli.command {
        // Handle the "Create" subcommand to create a new token contract.
        Commands::Create => {
            create::create_token(client).await?;
        }

        // Handle the "Verify" subcommand to verify an existing token contract.
        Commands::Verify { path, url } => {
            // Ensure that at least one of `--path` or `--url` is provided.
            if path.is_none() && url.is_none() {
                return Err(TokenGenErrors::InvalidInput(
                    "Error: Either --path or --url must be provided.".to_string(),
                ));
            }

            // Verify the contract from the provided local path, if specified.
            if let Some(path) = path {
                verify::verify_token_from_path(path, client.clone()).await?;
            }

            // Verify the contract from the provided repository URL, if specified.
            if let Some(url) = url {
                verify::verify_token_using_url(url, client).await?;
            }
        }
    }
    Ok(())
}
