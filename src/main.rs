//! # Sui Token Generator CLI Tool
//!
//! The Sui Token Generator CLI is a command-line tool designed for developers to:
//! - Create custom Sui token contracts with flexible parameters.
//! - Verify existing token contracts via local files or Git repositories.
//!
//! The tool operates on a client-server architecture, communicating with an RPC server for token creation and verification tasks.
//!
//! ## Available REST APIs
//!
//! ### 1. Create Token
//! **URL**: `/create`  
//! **Method**: `POST`  
//! **Request Body**: JSON containing token parameters.
//!
//! #### Example Usage:
//! ```bash
//! curl -X POST -H "Content-Type: application/json" \
//! -d '{
//!     "decimals": 1,
//!     "name": "My Token",
//!     "symbol": "MTK",
//!     "description": "Test token",
//!     "is_frozen": false,
//!     "environment": "devnet"
//! }' http://5.161.90.244:5001/create
//! ```
//!
//! #### Example Request Body:
//! ```json
//! {
//!   "decimals": 8,
//!   "name": "MyToken",
//!   "symbol": "MTK",
//!   "description": "A custom token.",
//!   "is_frozen": false,
//!   "environment": "devnet"
//! }
//! ```
//!
//! #### Example Response:
//! ```json
//! {
//!   "success": true,
//!   "message": "Creation successful",
//!   "data": {
//!     "token": "contract...",
//!     "move_toml": "move toml...",
//!     "test_token": "contract test..."
//!   }
//! }
//! ```
//!
//! ### 2. Verify Token from URL
//! **URL**: `/verify_url`  
//! **Method**: `POST`  
//! **Request Body**: JSON containing the repository URL.
//!
//! #### Example Usage:
//! ```bash
//! curl -X POST -H "Content-Type: application/json" \
//! -d '{
//!     "url": "https://github.com/meumar-osec/test-sui-token"
//! }' http://5.161.90.244:5001/verify_url
//! ```
//!
//! #### Example Request Body:
//! ```json
//! {
//!   "url": "https://github.com/meumar-osec/test-sui-token"
//! }
//! ```
//!
//! #### Example Response:
//! ```json
//! {
//!   "success": true,
//!   "message": "Verified successfully"
//! }
//! ```
//!
//! ### 3. Verify Token Content
//! **URL**: `/verify_content`  
//! **Method**: `POST`  
//! **Request Body**: JSON containing the contract content.
//!
//! #### Example Usage:
//! ```bash
//! curl -X POST -H "Content-Type: application/json" \
//! -d '{
//! "content": "module Mytoken::Mytoken {\n    use sui::coin::{Self, TreasuryCap};\n    public struct MYTOKEN has drop {}\n\n    /// Initialize the token with treasury and metadata\n    fun init(witness: MYTOKEN, ctx: &mut TxContext) {\n        let (treasury, metadata) = coin::create_currency(\n            witness, 8, b\"MT\", b\"My token\", b\"Tetsing\", option::none(), ctx\n        );\n        \n        transfer::public_freeze_object(metadata);\n        \n        transfer::public_transfer(treasury, ctx.sender());\n    }\n\n    public fun mint(\n\t\ttreasury_cap: &mut TreasuryCap<MYTOKEN>,\n\t\tamount: u64,\n\t\trecipient: address,\n\t\tctx: &mut TxContext,\n    ) {\n        let coin = coin::mint(treasury_cap, amount, ctx);\n        transfer::public_transfer(coin, recipient)\n    }\n}"
//! }' http://5.161.90.244:5001/verify_content
//! ```
//!
//! #### Example Request Body:
//! ```json
//! {
//!   "content": "module Mytoken::Mytoken ..."
//! }
//! ```
//!
//! #### Example Response:
//! ```json
//! {
//!   "success": true,
//!   "message": "Verified successfully"
//! }
//! ```

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
    Create,

    /// Verify an existing Sui token contract from a repository or local file.
    #[command(about = "Verifies an existing contract from a repo or local file.")]
    Verify {
        /// Path to the local contract file.
        #[arg(short, long)]
        path: Option<String>,

        /// URL of the repository containing the contract.
        #[arg(short, long)]
        url: Option<String>,
    },
}

/// Result type for the application, using custom error handling.
pub type Result<T> = std::result::Result<T, TokenGenErrors>;

/// Main asynchronous entry point.
///
/// Parses command-line arguments, executes the selected subcommand, and handles errors.
#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    handle_error(run_cli(cli).await);
}

/// Executes the selected subcommand.
///
/// # Arguments:
/// - `cli`: Parsed CLI arguments.
///
/// # Returns:
/// - `Ok(())` if successful.
/// - `Err(TokenGenErrors)` if an error occurs.
async fn run_cli(cli: Cli) -> Result<()> {
    let client: TokenGenClient = initiate_client(ADDRESS).await.map_err(|_| {
        TokenGenErrors::InvalidInput(format!("Failed to initiate a connection to the RPC service"))
    })?;

    match &cli.command {
        Commands::Create => create::create_token(client).await?,
        Commands::Verify { path, url } => {
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
