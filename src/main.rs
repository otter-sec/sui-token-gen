//! # Sui Token Generator CLI Tool
//!
//! The Sui Token Generator CLI is a command-line tool designed for developers to:
//! - Create custom Sui token contracts with flexible parameters.
//! - Verify existing token contracts via local files or Git repositories.
//!
//! The tool operates on a client-server architecture, communicating with an RPC server for token creation and verification tasks.
//!
use clap::{Parser, Subcommand};
use commands::{create, verify};
use errors::TokenGenErrors;
use handlers::handle_error;
use utils::{
    client::rpc_client::{initiate_client, TokenGenClient},
    helpers::validate_rpc_url,
};
pub use utils::constants;

mod commands;
mod errors;
mod handlers;
#[cfg(test)]
pub mod tests;
mod utils;


/// Result type for the application, using custom error handling.
pub type Result<T> = std::result::Result<T, TokenGenErrors>;

/// # Sui Token Generator CLI Tool
///
/// A command-line interface (CLI) tool to create and verify Sui token contracts.
///
/// ## Features:
/// - **Token Creation**: Easily generate Sui token contracts with customizable parameters.
/// - **Token Verification**: Validate existing token contracts via local files or repository URLs.
///
/// Use `--help` for detailed explanations of available commands and options.
#[derive(Parser, Debug)]
#[command(
    author = "Osec",
    version = "1.0.0",
    about = "Create and verify Sui Coin contracts",
    long_about = "Sui Token Generator is a CLI tool for developers to create and verify token contracts effortlessly."
)]
struct Cli {
    /// Available subcommands for the CLI tool.
    #[command(subcommand)]
    command: Commands,
}

/// Enum for supported subcommands.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new Sui token contract with customizable parameters.
    #[command(about = "Creates a new token contract.")]
    Create {
        /// Optional RPC URL for this command.
        #[arg(short, long)]
        rpc: Option<String>,
    },

    /// Verify an existing Sui token contract from a repository or local file.
    #[command(about = "Verifies an existing contract from a repo or local file.")]
    Verify {
        /// Optional RPC URL for this command.
        #[arg(short, long)]
        rpc: Option<String>,

        /// Path to the local contract file.
        #[arg(short, long)]
        path: Option<String>,

        /// URL of the repository containing the contract.
        #[arg(short, long)]
        url: Option<String>,
    },
}

/// Main asynchronous entry point.
///
/// Parses command-line arguments, executes the selected subcommand, and handles errors.
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    handle_error(run_cli(cli).await);
}

/// Executes the selected CLI subcommand based on user input.
///
/// # Arguments
/// * `cli` - The parsed CLI arguments containing the subcommand and its options
///
/// # Returns
/// * `Ok(())` - Command executed successfully
/// * `Err(TokenGenErrors)` - Command execution failed with specific error
///
/// # Details
/// Handles two main commands:
/// * `Create` - Generates a new Sui token contract
/// * `Verify` - Validates existing token contracts from local path or URL
async fn run_cli(cli: Cli) -> Result<()> {
    match &cli.command {
        Commands::Create { rpc } => {
            // Use the provided RPC URL or fall back to the default
            let rpc_url = rpc.clone().unwrap_or_else(|| constants::ADDRESS.to_string());

            // Validate the RPC URL
            validate_rpc_url(&rpc_url)?;

            let client: TokenGenClient = initiate_client(&rpc_url).await.map_err(|_| {
                TokenGenErrors::InvalidInput("Failed to connect to the RPC service".to_string())
            })?;
            create::create_token(client).await?;
        }
        Commands::Verify { rpc, path, url } => {
            // Use the provided RPC URL or fall back to the default
            let rpc_url = rpc.clone().unwrap_or_else(|| constants::ADDRESS.to_string());

            // Validate the RPC URL
            validate_rpc_url(&rpc_url)?;

            let client: TokenGenClient = initiate_client(&rpc_url).await.map_err(|_| {
                TokenGenErrors::InvalidInput("Failed to connect to the RPC service".to_string())
            })?;

            if path.is_none() && url.is_none() {
                return Err(TokenGenErrors::InvalidInput(
                    "Error: Either --path or --url must be provided.".to_string(),
                ));
            }
            if let Some(path) = path {
                verify::verify_token_from_path(path, client.clone()).await?;
            }
            if let Some(url) = url {
                verify::verify_token_using_url(url, client).await?;
            }
        }
    }
    Ok(())
}
