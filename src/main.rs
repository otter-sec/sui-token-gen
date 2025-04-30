//! # Sui Token Generator CLI Tool
//!
//! The Sui Token Generator CLI is a command-line tool designed for developers to:
//! - Create custom Sui token contracts with flexible parameters.
//! - Verify existing token contracts via local files, Git repositories, or blockchain addresses.
//!
//! The tool operates on a client-server architecture, communicating with an RPC server for token creation and verification tasks.
//!
use clap::{Parser, Subcommand};
use commands::{create, verify};
use errors::TokenGenErrors;
use handlers::handle_error;
pub use utils::constants;
use utils::{
    client::rpc_client::{initiate_client, TokenGenClient}, constants::DEFAULT_ENVIRONMENT, helpers::validate_rpc_url
};

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
/// - **Token Verification**: Validate existing token contracts via local files, repository URLs, or blockchain addresses.
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

#[derive(Debug, Clone, Parser)]
pub struct CreateTokenParams {
    /// Optional RPC URL for this command.
    #[arg(short, long)]
    rpc: Option<String>,

    /// Token name.
    #[arg(short, long)]
    name: Option<String>,

    /// Token symbol.
    #[arg(short, long)]
    symbol: Option<String>,

    /// Token decimals.
    #[arg(long)]
    decimals: Option<u8>,

    /// Token description.
    #[arg(short, long)]
    description: Option<String>,

    /// Whether metadata is frozen.
    #[arg(short, long)]
    is_frozen: Option<bool>,

    /// Token creation environment.
    #[arg(short, long)]
    environment: Option<String>,
}

/// Enum for supported subcommands.
#[derive(Subcommand, Debug)]
enum Commands {
    /// Create a new Sui token contract with customizable parameters.
    #[command(about = "Creates a new token contract.")]
    Create(CreateTokenParams),

    /// Verify an existing Sui token contract from a repository, local file, or blockchain address.
    #[command(about = "Verifies an existing contract from a repo, local file, or token address.")]
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

        /// Token contract address on the blockchain.
        #[arg(short, long)]
        address: Option<String>,

        /// Blockchain environment (mainnet, devnet, testnet).
        #[arg(short, long)]
        environment: Option<String>,
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
/// * `Err(TokenGenErrors)` - Command execution failed with a specific error
///
/// # Details
/// Handles three main verification methods:
/// * `Verify` with `--path`: Validates a contract from a local file.
/// * `Verify` with `--url`: Validates a contract from a repository.
/// * `Verify` with `--address` and `--environment`: Verifies a token contract by its blockchain address.
async fn run_cli(cli: Cli) -> Result<()> {
    match &cli.command {
        Commands::Create(params) => {
            let rpc_url = params
                .rpc
                .clone()
                .unwrap_or_else(|| constants::ADDRESS.to_string());

            let rpc_url = validate_rpc_url(&rpc_url)?;

            let client: TokenGenClient = initiate_client(&rpc_url)
                .await
                .map_err(|_| TokenGenErrors::FailedToConnectRpc)?;

            create::create_token(client, params).await?;
        }
        Commands::Verify {
            rpc,
            path,
            url,
            address,
            environment,
        } => {
            // Use the provided RPC URL or fall back to the default
            let rpc_url = rpc
                .clone()
                .unwrap_or_else(|| constants::ADDRESS.to_string());

            // Validate the RPC URL
            let rpc_url = validate_rpc_url(&rpc_url)?;

            let client: TokenGenClient = initiate_client(&rpc_url)
                .await
                .map_err(|_| TokenGenErrors::FailedToConnectRpc)?;

            // Ensure at least one verification parameter is provided
            if path.is_none() && url.is_none() && address.is_none() {
                return Err(TokenGenErrors::InvalidInput(
                    "Error: Either --path, --url, or --address must be provided.".to_string(),
                ));
            }

            // Verify by local file path
            if let Some(path) = path {
                verify::verify_token_from_path(path, client.clone()).await?;
            }

            // Verify by repository URL
            if let Some(url) = url {
                verify::verify_token_using_url(url, client.clone()).await?;
            }

            // Verify by token address and environment
            // Verify by token address with a default environment of DEFAULT_ENVIRONMENT
            if let Some(address) = address {
                let env = environment.clone().unwrap_or_else(|| DEFAULT_ENVIRONMENT.to_string());
                verify::verify_token_address(address, &env, client).await?;
            }
        }
    }
    Ok(())
}
