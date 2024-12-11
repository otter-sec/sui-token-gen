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
    ) -> anyhow::Result<(String, String), TokenGenErrors> {
        // Log the address when a request is handled
        self.log_address().await;

        // Validate decimals: must be a number greater than 0
        if decimals <= 0 {
            return Err(TokenGenErrors::InvalidDecimals);
        }

        // Validate symbol: must only contain alphanumeric characters and be no more than 5 characters long
        let symbol_regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();
        if !symbol_regex.is_match(&symbol) {
            return Err(TokenGenErrors::InvalidSymbol);
        }
        if symbol.len() > 5 {
            return Err(TokenGenErrors::InvalidSymbol);
        }

        // Validate name: must only contain alphanumeric characters and spaces
        let name_regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();
        if !name_regex.is_match(&name) {
            return Err(TokenGenErrors::InvalidName);
        }

        // Validate description: optional but must only contain alphanumeric characters and spaces
        if !description.is_empty() && !name_regex.is_match(&description) {
            return Err(TokenGenErrors::InvalidDescription);
        }

        // Proceed with the token generation logic
        let base_folder: String = sanitize_name(name.to_owned());
        let token_content: String = generation::generate_token(
            decimals,
            symbol.clone(),
            &name,
            description.clone(),
            is_frozen,
        );
        let move_toml_content = generation::generate_move_toml(&base_folder);

        Ok((token_content, move_toml_content)) // Return both toml file and contract as strings
    }

    async fn verify_url(
        self,
        _: context::Context,
        url: String,
    ) -> anyhow::Result<(), TokenGenErrors> {
        let response = verify_helper::verify_token_using_url(&url).await;
        if let Err(rpc_err) = response {
            return Err(TokenGenErrors::VerifyResultError(rpc_err.to_string()));
        }
        Ok(())
    }

    async fn verify_content(
        self,
        _: context::Context,
        content: String,
    ) -> anyhow::Result<(), TokenGenErrors> {
        let response = verify_helper::compare_contract_content(content);
        if let Err(rpc_err) = response {
            return Err(TokenGenErrors::VerifyResultError(rpc_err.to_string()));
        }
        Ok(())
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
