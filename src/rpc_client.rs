use anyhow::Result;
use std::{io::Error, net::SocketAddr};
use tarpc::{client, tokio_serde::formats::Json};

use crate::errors::RpcResponseErrors;

#[tarpc::service]
pub trait TokenGen {
    async fn create(
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), RpcResponseErrors>;

    async fn verify_url(url: String) -> Result<(), RpcResponseErrors>;
    async fn verify_content(content: String) -> Result<(), RpcResponseErrors>;
}

// Initializing RPC client
pub async fn initiate_client(address: &str) -> Result<TokenGenClient, Error> {
    // Parse address
    let server_addr: SocketAddr = address.parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid address format")
    })?;

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client: TokenGenClient =
        TokenGenClient::new(client::Config::default(), transport.await?).spawn();

    Ok(client)
}
