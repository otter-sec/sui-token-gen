use anyhow::Result;
use std::{io::Error, net::SocketAddr};
use tarpc::{client, service, tokio_serde::formats::Json};

use super::responses::RpcResponseErrors;

/// `TokenGen` trait defines the RPC interface for token generation operations.
#[service]
pub trait TokenGen {
    /// `create` is an asynchronous method that creates a token with the specified parameters.
    ///
    /// # Arguments
    /// - `decimals`: The number of decimal places for the token.
    /// - `name`: The name of the token.
    /// - `symbol`: The symbol for the token.
    /// - `description`: A description for the token.
    /// - `is_frozen`: Whether the token is frozen or not.
    /// - `environment`: The environment in which the token is deployed (e.g., `mainnet`, `devnet`, `testnet`).
    ///
    /// # Returns
    /// - A tuple containing:
    ///   - The created token's contract address.
    ///   - The transaction ID.
    ///   - The metadata associated with the token.
    ///
    /// # Errors
    /// - Returns `RpcResponseErrors` in case of RPC failures or invalid input parameters.
    #[allow(clippy::too_many_arguments)]
    async fn create(
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), RpcResponseErrors>;

    /// `verify_url` is an asynchronous method that verifies the validity of a provided URL.
    ///
    /// # Arguments
    /// - `url`: The URL to be verified.
    ///
    /// # Returns
    /// - `Ok(File name)` returns verified file name if the URL is valid.
    ///
    /// # Errors
    /// - Returns `RpcResponseErrors` if the URL is invalid or verification fails.
    async fn verify_url(url: String) -> Result<String, RpcResponseErrors>;

    /// `verify_content` is an asynchronous method that verifies the provided content.
    ///
    /// # Arguments
    /// - `content`: The content to be verified.
    ///
    /// # Returns
    /// - `Ok(())` if the content is valid.
    ///
    /// # Errors
    /// - Returns `RpcResponseErrors` if the content is invalid or verification fails.
    async fn verify_content(content: String, toml: String) -> Result<(), RpcResponseErrors>;

    /// `verify_address` is an asynchronous method that validates a given blockchain address in a specific environment.
    ///
    /// # Arguments
    /// - `address`: The blockchain address to be verified.
    /// - `environment`: The environment where the address should be verified (e.g., `mainnet`, `devnet`, `testnet`).
    ///
    /// # Returns
    /// - `Ok(())` if the address is valid in the specified environment.
    ///
    /// # Errors
    /// - Returns `RpcResponseErrors` if the address is invalid or verification fails.
    async fn verify_address(address: String, environment: String) -> Result<(), RpcResponseErrors>;
}

/// Initializes the RPC client by connecting to the provided server address.
pub async fn initiate_client(address: &str) -> Result<TokenGenClient, Error> {
    // Parse the address string to `SocketAddr` to ensure it is in a valid format.
    let server_addr: SocketAddr = address.parse().map_err(|_| {
        std::io::Error::new(std::io::ErrorKind::InvalidInput, "Invalid address format")
    })?;

    // Establish a TCP connection to the server, using JSON for serialization.
    let mut transport = tarpc::serde_transport::tcp::connect(server_addr, Json::default);

    // Configure the transport to handle large messages by increasing the maximum frame length.
    transport.config_mut().max_frame_length(usize::MAX);

    // Configure the client settings to support a larger number of in-flight requests and a bigger request buffer.
    let mut client_config = client::Config::default();
    client_config.max_in_flight_requests = 1024; // Allows up to 1024 concurrent requests
    client_config.pending_request_buffer = 1024; // Defines a buffer for pending requests

    // Create the RPC client using the transport and client configuration.
    let client: TokenGenClient = TokenGenClient::new(client_config, transport.await?).spawn();

    // Return the initialized client
    Ok(client)
}
