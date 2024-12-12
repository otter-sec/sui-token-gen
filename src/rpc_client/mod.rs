use std::net::SocketAddr;
use tarpc::{client, context};
use tarpc::tokio_serde::formats::Json;

use crate::errors::TokenGenErrors;
use crate::Result;

#[tarpc::service]
pub trait TokenGen: Send + Sync + 'static {
    /// Verify a token contract from a URL
    async fn verify_url(ctx: context::Context, url: String) -> Result<()>;
    /// Verify token contract content
    async fn verify_content(ctx: context::Context, content: String) -> Result<()>;
    /// Create a new token contract
    async fn create(
        ctx: context::Context,
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> Result<(String, String)>;
}

pub async fn connect_client(addr: SocketAddr) -> Result<client::NewClient<dyn TokenGen>> {
    let transport = tarpc::serde_transport::tcp::connect(addr, Json::default)
        .await
        .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?;

    Ok(client::new(client::Config::default(), transport).spawn())
}
