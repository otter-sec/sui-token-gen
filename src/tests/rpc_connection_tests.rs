use crate::{
    errors::TokenGenErrors,
    rpc_client::{create_timeout_context, initiate_client, TokenGenClient},
};

/// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client(address: &str) -> crate::Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}

#[tokio::test]
async fn rpc_client_connection_failure() -> crate::Result<()> {
    // Invalid address
    let invalid_address: &str = "127.0.0.1:5001";

    // Test invalid address scenario
    let result = setup_test_client(invalid_address).await;
    assert!(result.is_err());
    assert!(matches!(result, Err(TokenGenErrors::InvalidInput(_))));
    Ok(())
}

#[tokio::test]
async fn setup_test_client_error_handling() -> crate::Result<()> {
    let client = setup_test_client("127.0.0.1:5000").await?;
    assert!(client
        .verify_content(create_timeout_context(), "invalid content".to_string())
        .await
        .map_err(|e| TokenGenErrors::RpcError(e))?
        .is_err());
    Ok(())
}
