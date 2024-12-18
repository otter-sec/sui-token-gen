use clap::{Parser, Subcommand};

use error_handler::handle_error;
use errors::TokenGenErrors;
use rpc_client::{initiate_client, TokenGenClient};
use variables::ADDRESS;

mod commands;
mod error_handler;
mod errors;
mod rpc_client;
mod success_handler;
mod utils;
mod variables;

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
        // Path to the file
        #[arg(short, long)]
        path: Option<String>,

        // URL to fuzz
        #[arg(short, long)]
        url: Option<String>,
    },
}

// Define Return type for main function as Result<T, TokenGenErrors>
pub type Result<T> = std::result::Result<T, TokenGenErrors>;

#[tokio::main]
async fn main() {
    let cli = Cli::parse();
    handle_error(run_cli(cli).await);
}

async fn run_cli(cli: Cli) -> Result<()> {
    let client: TokenGenClient = initiate_client(ADDRESS)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))?;

    match &cli.command {
        Commands::Create => {
            commands::create::create_token(client).await?;
        }
        Commands::Verify { path, url } => {
            if path.is_none() && url.is_none() {
                return Err(TokenGenErrors::InvalidInput(
                    "Error: Either --path or --url must be provided.".to_string(),
                ));
            }

            if let Some(path) = path {
                commands::verify::verify_token_from_path(path, client.clone()).await?;
            }

            if let Some(url) = url {
                commands::verify::verify_token_using_url(url, client).await?;
            }
        }
    }
    Ok(())
}
