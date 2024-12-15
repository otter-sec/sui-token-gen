use anyhow::Result;
use std::{io::Error, net::SocketAddr, time::Duration};
use tarpc::{client, context, tokio_serde::formats::Json};

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

// Helper function to create a context with timeout
fn create_timeout_context() -> context::Context {
    let mut ctx = context::current();
    ctx.deadline = Some(tokio::time::Instant::now() + Duration::from_secs(30));
    ctx
}

// Initializing RPC client
pub async fn initiate_client(address: &str) -> Result<TokenGenClient, Error> {
    // Parse address
    let server_addr: SocketAddr = address.parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid address format")
    })?;

    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    // Configure client with increased buffer sizes for better reliability
    let mut client_config = client::Config::default();
    client_config.max_in_flight_requests = 1024;
    client_config.pending_request_buffer = 1024;

    // Add retry logic for connection establishment
    let mut retry_count = 0;
    const MAX_RETRIES: u32 = 3;
    const RETRY_DELAY: Duration = Duration::from_secs(1);

    loop {
        match transport.await {
            Ok(transport) => {
                let client = TokenGenClient::new(client_config.clone(), transport).spawn();
                return Ok(client);
            }
            Err(e) => {
                retry_count += 1;
                if retry_count >= MAX_RETRIES {
                    return Err(Error::new(
                        std::io::ErrorKind::ConnectionRefused,
                        format!("Failed to connect after {} retries: {}", MAX_RETRIES, e),
                    ));
                }
                tokio::time::sleep(RETRY_DELAY).await;
                transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);
                transport.config_mut().max_frame_length(usize::MAX);
            }
        }
    }
}
