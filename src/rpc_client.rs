use std::time::Duration;

use tarpc::{client, context, tokio_serde::formats::Json};

use crate::errors::TokenGenErrors;

#[tarpc::service]
pub trait TokenGen {
    async fn create(
        decimals: u8,
        name: String,
        symbol: String,
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
    let max_retries = 3;
    let mut retry_count = 0;
    let mut last_error = None;

    while retry_count < max_retries {
        match try_connect(server_addr).await {
            Ok(client) => return Ok(client),
            Err(e) => {
                last_error = Some(e);
                retry_count += 1;
                if retry_count < max_retries {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                }
            }
        }
    }

    Err(last_error.unwrap_or_else(|| {
        TokenGenErrors::RpcError("Failed to connect to RPC server after max retries".into())
    }))
}

async fn try_connect(server_addr: &str) -> Result<TokenGenClient, TokenGenErrors> {
    let mut config = tarpc::serde_transport::tcp::Config::default();
    config.max_frame_length = Some(64 * 1024 * 1024);
    config.max_response_time = Some(Duration::from_secs(60));

    let transport = tarpc::serde_transport::tcp::connect_with_config(server_addr, Json::default, config)
        .await
        .map_err(|e| TokenGenErrors::RpcError(format!("Failed to connect to RPC server: {}", e)))?;

    let client = TokenGenClient::new(client::Config::default(), transport).spawn();

    // Test connection with a longer timeout for initial connection
    let mut ctx = context::current();
    ctx.deadline = (tokio::time::Instant::now() + Duration::from_secs(10)).into();

    // Verify connection with a ping and store result
    let _ = client
        .verify_content(ctx, String::new())
        .await
        .map_err(|e| TokenGenErrors::RpcError(format!("Failed to verify connection: {}", e)))?;

    Ok(client)
}
