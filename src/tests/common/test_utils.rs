use crate::{
    errors::TokenGenErrors,
    utils::client::rpc_client::{initiate_client, TokenGenClient},
    Result,
};

// Helper function to set up a test client with consistent error handling
// This function initializes an RPC client by calling the initiate_client function and returns the client.
// If an error occurs during client initialization, it maps the error to a TokenGenError.
pub async fn setup_test_client(address: &str) -> Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|_| TokenGenErrors::FailedToConnectRpc)
} 