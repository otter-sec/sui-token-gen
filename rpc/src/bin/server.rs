use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
};

use anyhow::Result;
use clap::Parser;
use futures::{future, prelude::*};
use service::{
    init_tracing,
    utils::{
        verify_helper,
        errors::TokenGenErrors,
    },
    TokenGen,
};
use tarpc::{
    context,
    server::{BaseChannel, Channel},
    tokio_serde::formats::Json,
};
use tempfile::tempdir;

#[derive(Parser)]
struct Flags {
    /// The port to run on.
    #[clap(long, default_value = "5000")]
    port: u16,
}

#[derive(Clone)]
struct TokenServer;

#[tarpc::server]
impl TokenGen for TokenServer {
    async fn create(
        self,
        _: context::Context,
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        is_frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), TokenGenErrors> {
        // Validate environment: must be one of "mainnet", "devnet", or "testnet"
        let valid_environments = ["mainnet", "devnet", "testnet"];
        let validated_environment = if valid_environments.contains(&environment.as_str()) {
            environment
        } else {
            "devnet".to_string() // Default to "devnet" if invalid
        };

        // Use validated environment for further processing
        tracing::info!("Using environment: {}", validated_environment);

        // Get project root directory (two levels up from rpc/src/bin)
        let current_dir = std::env::current_dir()
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to get current directory: {}", e)))?;

        let project_root = current_dir
            .parent() // up from bin
            .and_then(|p| p.parent()) // up from src
            .and_then(|p| p.parent()) // up from rpc
            .ok_or_else(|| TokenGenErrors::FileIoError("Failed to find project root".into()))?;

        // Read template files from project root
        let template_dir = project_root.join("src").join("templates");
        tracing::info!("Template directory: {:?}", template_dir);

        let token_template = fs::read_to_string(template_dir.join("move/token.move.template"))
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read token template: {}", e)))?;

        let toml_template = fs::read_to_string(template_dir.join("toml/Move.toml.template"))
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read toml template: {}", e)))?;

        // Replace placeholders in token template
        let token_content = token_template
            .replace("{{name}}", &name)
            .replace("{{symbol}}", &symbol)
            .replace("{{description}}", &description)
            .replace("{{decimals}}", &decimals.to_string())
            .replace("{{is_frozen}}", &is_frozen.to_string());

        // Replace placeholders in toml template
        let toml_content = toml_template
            .replace("{{name}}", &name)
            .replace("{{symbol}}", &symbol)
            .replace("{{environment}}", &validated_environment);

        // Create temporary directory for token files and return it with contents
        let temp_dir = tempdir().map_err(|e| {
            TokenGenErrors::FileIoError(format!("Failed to create temporary directory: {}", e))
        })?;

        Ok((token_content.clone(), toml_content, temp_dir.path().to_string_lossy().to_string()))
    }

    async fn verify_url(
        self,
        _: context::Context,
        url: String,
    ) -> Result<(), TokenGenErrors> {
        match verify_helper::verify_token_using_url(&url).await {
            Ok(_) => Ok(()),
            Err(e) => Err(TokenGenErrors::VerificationError(e.to_string())),
        }
    }

    async fn verify_content(
        self,
        _: context::Context,
        content: String,
    ) -> Result<(), TokenGenErrors> {
        // Create a temporary directory for verification
        let temp_dir = tempdir().map_err(|e| {
            TokenGenErrors::FileIoError(format!("Failed to create temporary directory: {}", e))
        })?;

        // Write content to a temporary file
        let temp_file = temp_dir.path().join("temp.move");
        fs::write(&temp_file, &content).map_err(|e| {
            TokenGenErrors::FileIoError(format!("Failed to write temporary file: {}", e))
        })?;

        // Verify the contract
        match verify_helper::verify_contract(temp_dir.path(), temp_dir.path()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(TokenGenErrors::VerificationError(e.to_string())),
        }
    }
}

impl TokenServer {
    fn new() -> Self {
        Self
    }

    fn log_address(&self, addr: SocketAddr) {
        println!("Server running on {}", addr);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Parse command line arguments.
    let flags = Flags::parse();
    let server_addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), flags.port);

    // Initialize logging
    init_tracing("sui-token-gen-rpc")?;

    // Start the server and print out its socket address.
    let listener = tarpc::serde_transport::tcp::listen(server_addr, Json::default).await?;
    let listener = listener.filter_map(|r| future::ready(r.ok()));
    println!("Server running on {}", server_addr);

    let server = TokenServer::new();
    server.log_address(server_addr);

    listener
        .map(BaseChannel::with_defaults)
        .map(|channel| {
            let server = server.clone();
            channel.execute(server.serve())
        })
        .for_each(|_| async {})
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn test_server() -> TokenServer {
        TokenServer::new()
    }

    // PLACEHOLDER: test implementation functions
}
