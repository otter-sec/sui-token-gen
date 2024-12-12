use super::{setup_test_environment, cleanup_test_environment};
use crate::utils::errors::TokenGenErrors;
use tarpc::context;
use anyhow::Result;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_client_connection() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test basic connectivity
    let ctx = context::current();
    assert!(client.verify_url(ctx.clone(), "https://github.com/test/repo.git".to_string()).await.is_ok());

    // Test client retry mechanism
    let retry_ctx = context::current();
    retry_ctx.deadline = Instant::now() + Duration::from_secs(5);

    // Simulate slow response
    let result = client
        .verify_url(retry_ctx, "https://github.com/test/repo.git".to_string())
        .await;
    assert!(result.is_ok());

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_client_error_handling() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    // Test invalid token parameters
    let result = client
        .create(
            ctx.clone(),
            255, // Invalid decimals
            "".to_string(), // Empty name
            "TOOLONG".to_string(), // Symbol too long
            "Description".to_string(),
            false,
            "invalid_network".to_string(),
        )
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals(_))));

    // Test network timeout
    let timeout_ctx = context::current();
    timeout_ctx.deadline = Instant::now();
    let result = client
        .verify_url(timeout_ctx, "https://github.com/test/repo.git".to_string())
        .await;
    assert!(result.is_err());

    // Test invalid URL format
    let result = client
        .verify_url(ctx.clone(), "not-a-url".to_string())
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidUrl(_))));

    // Test invalid content verification
    let result = client
        .verify_content(ctx, "invalid move code".to_string())
        .await;
    assert!(result.is_err());

    cleanup_test_environment().await;
    Ok(())
}
