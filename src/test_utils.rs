use crate::{
    errors::TokenGenErrors,
    rpc_client::{initiate_client, TokenGenClient},
    Result,
};

/// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client() -> Result<TokenGenClient> {
    initiate_client()
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}
