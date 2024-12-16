use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::{initiate_client, TokenGenClient},
    variables::ADDRESS,
    Result,
};

// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client(address: &str) -> Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}

#[tokio::test]
async fn rpc_client_connection_failure() -> Result<()> {
    // Invalid address
    let invalid_address: &str = "127.0.0.1:5001";

    // Test invalid address scenario
    let result = setup_test_client(invalid_address).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(TokenGenErrors::InvalidInput(_))));
    Ok(())
}

#[tokio::test]
async fn setup_test_client_error_handling() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;
    assert!(client
        .verify_content(context::current(), "invalid content".to_string())
        .await
        .map_err(|e| TokenGenErrors::RpcError(e))?
        .is_err());
    Ok(())
}

#[tokio::test]
async fn test_error_propagation() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test RPC error propagation
    let result = client
        .verify_content(context::current(), "invalid content".to_string())
        .await
        .map_err(TokenGenErrors::from);
    assert!(matches!(result, Err(TokenGenErrors::RpcError(_))));

    // Test verification error propagation
    let result = client
        .verify_url(context::current(), "https://invalid-url".to_string())
        .await
        .map_err(TokenGenErrors::from);
    assert!(matches!(result, Err(TokenGenErrors::InvalidUrl(_))));

    Ok(())
}
