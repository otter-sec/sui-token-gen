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
    let max_retries = 5;
    let mut retry_count = 0;
    let mut last_error = None;

    while retry_count < max_retries {
        match try_connect(server_addr).await {
            Ok(client) => return Ok(client),
            Err(e) => {
                tracing::warn!("Connection attempt {} failed: {}", retry_count + 1, e);
                last_error = Some(e);
                retry_count += 1;
                if retry_count < max_retries {
                    let backoff = Duration::from_secs(2 * retry_count as u64);
                    tracing::info!("Waiting {:?} before retry...", backoff);
                    tokio::time::sleep(backoff).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        TokenGenErrors::RpcError("Failed to connect to RPC server after max retries".into())
    }))
}

async fn try_connect(server_addr: &str) -> Result<TokenGenClient, TokenGenErrors> {
    tracing::info!("Attempting to connect to RPC server at {}", server_addr);
    let transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default)
        .await
        .map_err(|e| TokenGenErrors::RpcError(format!("Failed to connect to RPC server: {}", e)))?;

    let client = TokenGenClient::new(client::Config::default(), transport).spawn();

    // Test connection with a longer timeout for initial connection
    let mut ctx = context::current();
    ctx.deadline = (tokio::time::Instant::now() + Duration::from_secs(30)).into();

    tracing::info!("Verifying connection...");
    // Verify connection with a ping and store result
    let _ = client
        .verify_content(ctx, String::new())
        .await
        .map_err(|e| TokenGenErrors::RpcError(format!("Failed to verify connection: {}", e)))?;
    tracing::info!("Connection verified successfully");

    Ok(client)
}
