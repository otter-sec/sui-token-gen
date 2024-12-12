use crate::{
    errors::TokenGenErrors,
    rpc_client::{initiate_client, TokenGenClient},
    Result,
};
use tarpc::context;

/// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client() -> Result<TokenGenClient> {
    initiate_client()
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_setup_test_client_error_handling() -> Result<()> {
        let client = setup_test_client().await?;
        assert!(client.verify_content(context::current(), "invalid content".to_string()).await.is_err());
        Ok(())
    }
}
