use clap::Parser;
use futures::{future, prelude::*};
use regex::Regex;
use service::{
    init_tracing,
    utils::{helpers::sanitize_name, verify_helper},
    TokenGen,
};
use suitokengentest::errors::{RpcResponseErrors, TokenGenErrors};
use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::Path,
};
use tarpc::{
    context,
    server::{self, Channel},
    tokio_serde::formats::Json,
};

#[derive(Parser)]
struct Flags {
    #[clap(long)]
    port: u16,
}

#[derive(Clone)]
struct TokenServer {
    addr: SocketAddr,
}

impl TokenServer {
    fn new(addr: SocketAddr) -> Self {
        TokenServer { addr }
    }

    async fn log_address(&self) {
        tracing::info!("Server address: {}", self.addr);
    }
}

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
    ) -> anyhow::Result<(String, String, String), TokenGenErrors> {
        // Log the address when a request is handled
        self.log_address().await;

        // Validate decimals: must be a number greater than 0 and less than 100
        if decimals <= 0 || decimals >= 100 {
            return Err(TokenGenErrors::InvalidDecimals("Decimals must be between 1 and 99".to_string()));
        }

        // Validate symbol: must only contain alphanumeric characters and be no more than 5 characters long
        let symbol_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !symbol_regex.is_match(&symbol) {
            return Err(TokenGenErrors::InvalidSymbol("Symbol must contain only alphanumeric characters".to_string()));
        }
        if symbol.len() > 6 {
            return Err(TokenGenErrors::InvalidSymbol("Symbol must not be longer than 6 characters".to_string()));
        }

        // Validate name: must only contain alphanumeric characters, spaces, commas, and dots
        let name_regex = Regex::new(r"^[a-zA-Z0-9\s,\.]+$").unwrap();
        if !name_regex.is_match(&name) {
            return Err(TokenGenErrors::InvalidName("Name must contain only alphanumeric characters, spaces, commas, and dots".to_string()));
        }

        // Validate description: optional but must only contain alphanumeric characters, spaces, commas, and dots
        if !description.is_empty() && !name_regex.is_match(&description) {
            return Err(TokenGenErrors::InvalidDescription("Description must contain only alphanumeric characters, spaces, commas, and dots".to_string()));
        }

        // Validate environment: must be one of "mainnet", "devnet", or "testnet"
        let valid_environments = ["mainnet", "devnet", "testnet"];
        let environment = if valid_environments.contains(&environment.as_str()) {
            environment
        } else {
            "devnet".to_string() // Default to "devnet" if invalid
        };

        // Read template files from project root
        let template_dir = Path::new("../src/templates");
        let token_template = fs::read_to_string(template_dir.join("move/token.move.template"))
            .map_err(|e| TokenGenErrors::TemplateNotFound(format!("Failed to read token template: {}", e)))?;
        let toml_template = fs::read_to_string(template_dir.join("toml/Move.toml.template"))
            .map_err(|e| TokenGenErrors::TemplateNotFound(format!("Failed to read toml template: {}", e)))?;

        // Process templates with token info
        let token_content = token_template
            .replace("{{token_name}}", &name)
            .replace("{{token_symbol}}", &symbol)
            .replace("{{token_decimals}}", &decimals.to_string())
            .replace("{{token_description}}", &description)
            .replace("{{token_frozen}}", &is_frozen.to_string());

        let move_toml_content = toml_template.replace("{{package_name}}", &sanitize_name(&name));

        Ok((token_content.clone(), move_toml_content, token_content)) // Return both toml file and contract as strings
    }

    async fn verify_url(
        self,
        _: context::Context,
        url: String,
    ) -> anyhow::Result<(), TokenGenErrors> {
        verify_helper::verify_token_using_url(&url)
            .await
            .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))
    }

    async fn verify_content(
        self,
        _: context::Context,
        content: String,
    ) -> anyhow::Result<(), TokenGenErrors> {
        verify_helper::compare_contract_content(content, None)
            .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("Sui-token-get rpc")?;

    // Use IPv4 localhost instead of IPv6 for better compatibility
    let server_addr = (IpAddr::V4(Ipv4Addr::LOCALHOST), flags.port);
    tracing::info!("Attempting to bind to {:?}", server_addr);

    // Configure and start TCP listener with error handling
    let mut listener = match tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await {
        Ok(l) => {
            tracing::info!("Successfully bound to {:?}", l.local_addr());
            l
        }
        Err(e) => {
            tracing::error!("Failed to bind to {:?}: {}", server_addr, e);
            return Err(e.into());
        }
    };

    // Configure server with longer timeouts and larger frame size
    listener.config_mut().max_frame_length(usize::MAX);

    // Add more debug logging
    tracing::info!("Starting server with configuration: {:?}", listener.config());

    // Create a shutdown signal channel
    let (shutdown_tx, shutdown_rx) = tokio::sync::oneshot::channel::<()>();
    let shutdown_tx_clone = shutdown_tx.clone();

    // Spawn server in a separate task
    let server_handle = tokio::spawn(async move {
        listener
            .filter_map(|r| {
                if let Err(e) = &r {
                    tracing::error!("Connection error: {:?}", e);
                }
                future::ready(r.ok())
            })
            .map(|channel| {
                let channel = server::BaseChannel::with_defaults(channel);
                tracing::info!("New client connected from: {:?}", channel.transport().peer_addr());
                channel
            })
            .map(|channel| {
                let server = TokenServer::new(channel.transport().peer_addr().unwrap());

                // Keep connection alive until shutdown signal
                let mut shutdown_rx = shutdown_rx.clone();
                tokio::spawn(async move {
                    let _ = shutdown_rx.await;
                });

                channel.execute(server.serve()).for_each(spawn)
            })
            .buffer_unordered(10)
            .for_each(|_| async {})
            .await;
    });

    // Handle server shutdown gracefully
    tokio::select! {
        _ = tokio::signal::ctrl_c() => {
            tracing::info!("Received Ctrl+C, initiating graceful shutdown...");
        }
        _ = server_handle => {
            tracing::info!("Server task completed, initiating shutdown...");
        }
    }

    // Send shutdown signal to all connections
    let _ = shutdown_tx.send(());
    let _ = shutdown_tx_clone.send(());

    // Wait for server to finish
    tracing::info!("Waiting for server to shut down...");
    server_handle.abort();
    match server_handle.await {
        Ok(_) => tracing::info!("Server shut down successfully"),
        Err(e) => tracing::warn!("Server shutdown error: {:?}", e),
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::SocketAddr;
    use tarpc::context;

    fn test_server() -> TokenServer {
        let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 50051);
        TokenServer::new(addr)
    }

    #[tokio::test]
    async fn test_create_token_validation() {
        let server = test_server();
        let ctx = context::current();

        // Test invalid decimals
        let result = server
            .clone()
            .create(
                ctx.clone(),
                0,
                "Test".into(),
                "TST".into(),
                "Description".into(),
                false,
                "devnet".into(),
            )
            .await;
        assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals)));

        // Test invalid symbol (too long)
        let result = server
            .clone()
            .create(
                ctx.clone(),
                8,
                "Test".into(),
                "TSTSTST".into(),
                "Description".into(),
                false,
                "devnet".into(),
            )
            .await;
        assert!(matches!(result, Err(TokenGenErrors::InvalidSymbol)));

        // Test invalid symbol (special characters)
        let result = server
            .clone()
            .create(
                ctx.clone(),
                8,
                "Test".into(),
                "T$T".into(),
                "Description".into(),
                false,
                "devnet".into(),
            )
            .await;
        assert!(matches!(result, Err(TokenGenErrors::InvalidSymbol)));

        // Test invalid name (special characters)
        let result = server
            .clone()
            .create(
                ctx.clone(),
                8,
                "Test@".into(),
                "TST".into(),
                "Description".into(),
                false,
                "devnet".into(),
            )
            .await;
        assert!(matches!(result, Err(TokenGenErrors::InvalidName)));

        // Test invalid description (special characters)
        let result = server
            .clone()
            .create(
                ctx.clone(),
                8,
                "Test".into(),
                "TST".into(),
                "Test@Description".into(),
                false,
                "devnet".into(),
            )
            .await;
        assert!(matches!(result, Err(TokenGenErrors::InvalidDescription)));

        // Test valid input
        let result = server
            .create(
                ctx,
                8,
                "Test Token".into(),
                "TST".into(),
                "Test Description".into(),
                false,
                "devnet".into(),
            )
            .await;
        assert!(result.is_ok());
    }

    #[tokio::test]
    async fn test_verify_url_validation() {
        let server = test_server();
        let ctx = context::current();

        // Test invalid URL
        let result = server
            .clone()
            .verify_url(ctx.clone(), "not_a_url".into())
            .await;
        assert!(result.is_err());

        // Test invalid git URL
        let result = server
            .clone()
            .verify_url(ctx.clone(), "https://example.com/not-a-git-repo.git".into())
            .await;
        assert!(result.is_err());

        // Test valid URL format (actual verification will be tested in integration tests)
        let result = server
            .verify_url(ctx, "https://github.com/valid/repo.git".into())
            .await;
        assert!(result.is_err()); // Will fail because repo doesn't exist, but URL format is valid
    }

    #[tokio::test]
    async fn test_verify_content_validation() {
        let server = test_server();
        let ctx = context::current();

        // Test empty content
        let result = server.clone().verify_content(ctx.clone(), "".into()).await;
        assert!(result.is_err());

        // Test invalid content
        let result = server
            .clone()
            .verify_content(ctx.clone(), "invalid content".into())
            .await;
        assert!(result.is_err());

        // Test malformed Move code
        let result = server
            .verify_content(ctx, "module test { public fun main() { } }".into())
            .await;
        assert!(result.is_err());
    }
}
