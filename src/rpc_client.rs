use std::time::Duration;

use tarpc::{
    client::{self, Channel},
    context,
    serde_transport::tcp::connect,
    tokio_serde::formats::Json,
};
use tracing;

use crate::errors::TokenGenErrors;

#[tarpc::service]
pub trait TokenGen {
    async fn create(
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        is_frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), TokenGenErrors>;

    async fn verify_url(url: String) -> Result<(), TokenGenErrors>;
    async fn verify_content(content: String) -> Result<(), TokenGenErrors>;
}

// Helper function to create a context with timeout and keep-alive
pub fn create_timeout_context() -> context::Context {
    let mut ctx = context::current();
    ctx.deadline = (tokio::time::Instant::now() + Duration::from_secs(30)).into();
    ctx
}

pub async fn initiate_client(server_addr: &str) -> Result<TokenGenClient, TokenGenErrors> {
    tracing::info!("Attempting to connect to RPC server at {}", server_addr);

    let transport = connect(server_addr, Json::default)
        .await
        .map_err(|e| TokenGenErrors::RpcError(format!("Failed to connect to RPC server: {}", e)))?;

    let client_config = client::Config::default()
        .request_timeout(Some(Duration::from_secs(30)));

    let client = TokenGenClient::new(client_config, transport).spawn();
    tracing::info!("RPC client initialized successfully");

    Ok(client)
}
