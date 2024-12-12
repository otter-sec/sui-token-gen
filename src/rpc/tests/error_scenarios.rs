use super::{setup_test_environment, cleanup_test_environment};
use crate::utils::errors::TokenGenErrors;
use tarpc::context;
use anyhow::Result;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_server_shutdown_handling() -> Result<()> {
    let client = setup_test_environment().await?;

    // Initial request should succeed
    let ctx = context::current();
    let result = client
        .verify_url(ctx.clone(), "https://github.com/test/repo.git".to_string())
        .await;
    assert!(result.is_ok());

    // Shutdown server
    cleanup_test_environment().await;

    // Request after shutdown should fail
    let result = client
        .verify_url(ctx, "https://github.com/test/repo.git".to_string())
        .await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_timeout_handling() -> Result<()> {
    let client = setup_test_environment().await?;

    // Create context with short timeout
    let mut ctx = context::current();
    ctx.deadline = Instant::now() + Duration::from_millis(1);

    // Sleep to trigger timeout
    tokio::time::sleep(Duration::from_millis(10)).await;

    let result = client
        .verify_url(ctx, "https://github.com/test/repo.git".to_string())
        .await;
    assert!(result.is_err());

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_invalid_token_parameters() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    // Test invalid decimals
    let result = client
        .create(
            ctx.clone(),
            255,
            "Test Token".to_string(),
            "TST".to_string(),
            "Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals(_))));

    // Test empty name
    let result = client
        .create(
            ctx.clone(),
            8,
            "".to_string(),
            "TST".to_string(),
            "Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidName(_))));

    // Test invalid symbol
    let result = client
        .create(
            ctx.clone(),
            8,
            "Test Token".to_string(),
            "TOOLONGSYMBOL".to_string(),
            "Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidSymbol(_))));

    // Test invalid environment
    let result = client
        .create(
            ctx,
            8,
            "Test Token".to_string(),
            "TST".to_string(),
            "Description".to_string(),
            false,
            "invalid_network".to_string(),
        )
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidEnvironment(_))));

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_network_failures() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    // Test invalid repository URL
    let result = client
        .verify_url(ctx.clone(), "https://invalid-domain/repo.git".to_string())
        .await;
    assert!(result.is_err());

    // Test non-existent repository
    let result = client
        .verify_url(
            ctx.clone(),
            "https://github.com/nonexistent/repository.git".to_string(),
        )
        .await;
    assert!(result.is_err());

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_concurrent_error_handling() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    let mut handles = vec![];

    // Spawn multiple invalid requests concurrently
    for i in 0..5 {
        let client = client.clone();
        let ctx = ctx.clone();
        handles.push(tokio::spawn(async move {
            client
                .create(
                    ctx,
                    255, // Invalid decimals
                    format!("Token {}", i),
                    "TST".to_string(),
                    "Description".to_string(),
                    false,
                    "devnet".to_string(),
                )
                .await
        }));
    }

    // All requests should fail with InvalidDecimals error
    for handle in handles {
        let result = handle.await?;
        assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals(_))));
    }

    cleanup_test_environment().await;
    Ok(())
}
