use std::fs;
use suitokengentest::{
    errors::TokenGenErrors,
    test_utils::{cleanup_test_environment, setup_test_environment},
    Result,
};
use tarpc::context;

/// Test the complete flow of token creation and verification
#[tokio::test]
async fn test_create_and_verify_flow() -> Result<()> {
    // Set up test environment
    let client = setup_test_environment().await?;

    // Test token creation
    let (token_content, move_toml) = client
        .create(
            context::current(),
            8,
            "Test Token".into(),
            "TST".into(),
            "Test Description".into(),
            false
        )
        .await?;

    // Write token to test file
    let test_dir = "test_token_dir";
    fs::create_dir_all(test_dir)?;
    fs::write(format!("{}/sources/token.move", test_dir), token_content)?;
    fs::write(format!("{}/Move.toml", test_dir), move_toml.clone())?;

    // Verify token content
    assert!(client.verify_content(context::current(), token_content).await.is_ok());

    // Verify TOML structure
    assert!(move_toml.contains("name = \"Test Token\""));
    assert!(move_toml.contains("version = \"0.1.0\""));
    assert!(move_toml.contains("symbol = \"TST\""));

    // Clean up
    fs::remove_dir_all(test_dir)?;
    cleanup_test_environment();
    Ok(())
}

/// Test error handling in the complete flow
#[tokio::test]
async fn test_error_handling_flow() -> Result<()> {
    // Set up test environment
    let client = setup_test_environment().await?;

    // Test invalid token creation
    let result = client
        .create(
            context::current(),
            0,
            "Invalid@Token".into(),
            "T$T".into(),
            "Test".into(),
            false
        )
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals)));

    // Test invalid content verification
    let result = client.verify_content(context::current(), "invalid content".into()).await;
    assert!(result.is_err());

    // Test invalid URL verification
    let result = client.verify_url(context::current(), "not_a_url".into()).await;
    assert!(result.is_err());

    // Clean up
    cleanup_test_environment();
    Ok(())
}

/// Test the URL verification flow
#[tokio::test]
async fn test_url_verification_flow() -> Result<()> {
    // Set up test environment
    let client = setup_test_environment().await?;

    // Create a valid token first
    let (token_content, _) = client
        .create(
            context::current(),
            8,
            "Test Token".into(),
            "TST".into(),
            "Test Description".into(),
            false
        )
        .await?;

    // Test verification with invalid URL
    let result = client.verify_url(context::current(), "https://not-a-real-repo.git".into()).await;
    assert!(result.is_err());

    // Test verification with malformed URL
    let result = client.verify_url(context::current(), "not-even-a-url".into()).await;
    assert!(result.is_err());

    // Clean up
    cleanup_test_environment();
    Ok(())
}
