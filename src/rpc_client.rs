use anyhow::Result;
use std::{io::Error, net::SocketAddr};
use tarpc::{client, tokio_serde::formats::Json};

#[tarpc::service]
pub trait TokenGen {
    async fn create(
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> Result<(String, String), String>;

    async fn verify_url(url: String) -> String;
    async fn verify_content(content: String) -> String;
}

// Initializing RPC client
pub async fn initiate_client() -> Result<TokenGenClient, Error> {
    // RPC server address
    const ADDRESS: &str = "[::1]:5000";

    // Parse address
    let server_addr: SocketAddr = ADDRESS.parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid address format")
    })?;

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client: TokenGenClient =
        TokenGenClient::new(client::Config::default(), transport.await?).spawn();

    Ok(client)
}
