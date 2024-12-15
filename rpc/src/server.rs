use clap::Parser;
use futures::{future, prelude::*};
use regex::Regex;
use service::{init_tracing, TokenGen, TokenGenErrors};
use std::net::{IpAddr, Ipv6Addr, SocketAddr};
use tarpc::{
    context,
    server::{self, Channel},
    tokio_serde::formats::Json,
};

mod utils;
use utils::{generation, helpers::sanitize_name, verify_helper};

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
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
        environment: String,
    ) -> anyhow::Result<(String, String, String), TokenGenErrors> {
        // Log the address when a request is handled
        self.log_address().await;

        // Validate decimals: must be a number greater than 0 and less than 100
        if decimals <= 0 || decimals >= 100 {
            return Err(TokenGenErrors::InvalidDecimals);
        }

        // Validate symbol: must only contain alphanumeric characters and be no more than 5 characters long
        let symbol_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !symbol_regex.is_match(&symbol) {
            return Err(TokenGenErrors::InvalidSymbol);
        }
        if symbol.len() > 6 {
            return Err(TokenGenErrors::InvalidSymbol);
        }

        // Validate name: must only contain alphanumeric characters, spaces, commas, and dots
        let name_regex = Regex::new(r"^[a-zA-Z0-9\s,\.]+$").unwrap();
        if !name_regex.is_match(&name) {
            return Err(TokenGenErrors::InvalidName);
        }

        // Validate description: optional but must only contain alphanumeric characters, spaces, commas, and dots
        if !description.is_empty() && !name_regex.is_match(&description) {
            return Err(TokenGenErrors::InvalidDescription);
        }

        // Validate environment: must be one of "mainnet", "devnet", or "testnet"
        let valid_environments = ["mainnet", "devnet", "testnet"];
        let environment = if valid_environments.contains(&environment.as_str()) {
            environment
        } else {
            "devnet".to_string() // Default to "devnet" if invalid
        };

        // Proceed with the token generation logic
        let base_folder: String = sanitize_name(&name);
        let token_content: String = generation::generate_token(
            decimals,
            symbol.clone(),
            &name,
            description.clone(),
            is_frozen,
            false,
        );
        let test_token_content: String = generation::generate_token(
            decimals,
            symbol.clone(),
            &name,
            description.clone(),
            is_frozen,
            true,
        );

        let move_toml_content = generation::generate_move_toml(&base_folder, environment);

        Ok((token_content, move_toml_content, test_token_content)) // Return both toml file and contract as strings
    }

    async fn verify_url(
        self,
        _: context::Context,
        url: String,
    ) -> anyhow::Result<(), TokenGenErrors> {
        verify_helper::verify_token_using_url(&url)
            .await
            .map_err(|e| TokenGenErrors::VerifyResultError(e.to_string()))
    }

    async fn verify_content(
        self,
        _: context::Context,
        content: String,
    ) -> anyhow::Result<(), TokenGenErrors> {
        verify_helper::compare_contract_content(content, None)
            .map_err(|e| TokenGenErrors::VerifyResultError(e.to_string()))
    }
}

async fn spawn(fut: impl Future<Output = ()> + Send + 'static) {
    tokio::spawn(fut);
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let flags = Flags::parse();
    init_tracing("Sui-token-get rpc")?;

    let server_addr = (IpAddr::V6(Ipv6Addr::LOCALHOST), flags.port);
    let mut listener = tarpc::serde_transport::tcp::listen(&server_addr, Json::default).await?;
    tracing::info!("Listening on port {}", listener.local_addr().port());
    listener.config_mut().max_frame_length(usize::MAX);
    listener
        .filter_map(|r| future::ready(r.ok()))
        .map(server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = TokenServer::new(channel.transport().peer_addr().unwrap());
            channel.execute(server.serve()).for_each(spawn)
        })
        .buffer_unordered(10)
        .for_each(|_| async {})
        .await;

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
