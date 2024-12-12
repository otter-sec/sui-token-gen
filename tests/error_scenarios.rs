use std::time::Duration;
use tarpc::context;
use suitokengentest::{
    errors::TokenGenErrors,
    test_utils::{cleanup_test_environment, mocks::MockGitRepo, setup_test_environment},
    Result,
};
use tempfile::tempdir;

#[tokio::test]
async fn test_invalid_token_parameters() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test invalid decimals
    let result = client.create(
        context::current(),
        0,
        "Test".into(),
        "TST".into(),
        "Test".into(),
        false
    ).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals)));

    // Test invalid symbol
    let result = client.create(
        context::current(),
        8,
        "Test".into(),
        "T$T".into(),
        "Test".into(),
        false
    ).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidSymbol(_))));

    // Test empty name
    let result = client.create(
        context::current(),
        8,
        "".into(),
        "TST".into(),
        "Test".into(),
        false
    ).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidName(_))));

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_rpc_connection_failure() -> Result<()> {
    // Set up test environment with intentionally wrong port
    let client = setup_test_environment().await?;

    // Wait a moment to ensure server is ready
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Try to create a token (should fail due to connection issues)
    let result = client
        .create(8, "Test".into(), "TST".into(), "Description".into(), false)
        .await;

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, TokenGenErrors::GeneralError(_)));
    }

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_invalid_git_repo() -> Result<()> {
    // Set up test environment
    let client = setup_test_environment().await?;

    // Create a temporary directory for the mock repo
    let temp_dir = tempdir()?;
    let mock_repo = MockGitRepo::new(temp_dir.path().to_path_buf());

    // Initialize repo with invalid Move contract
    mock_repo.add_test_files(vec![
        ("Move.toml", "invalid toml content"),
        ("sources/token.move", "invalid move content"),
    ])?;

    // Try to verify the invalid repo
    let result = client
        .verify_url(temp_dir.path().to_str().unwrap().into())
        .await;

    assert!(result.is_err());
    if let Err(e) = result {
        assert!(matches!(e, TokenGenErrors::GitError(_)));
    }

    // Clean up
    mock_repo.cleanup()?;
    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_invalid_move_content() -> Result<()> {
    // Set up test environment
    let client = setup_test_environment().await?;

    // Try to verify invalid Move content
    let result = client.verify_content(
        context::current(),
        "invalid move module syntax".into()
    ).await;

    assert!(result.is_err());
    assert!(matches!(result, Err(TokenGenErrors::InvalidInput(_))));

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_invalid_url_verification() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test invalid URL format
    let result = client.verify_url(context::current(), "not-a-url".into()).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidInput(_))));

    // Test unsupported protocol
    let result = client.verify_url(context::current(), "ftp://test.com".into()).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidInput(_))));

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_server_shutdown_handling() -> Result<()> {
    let client = setup_test_environment().await?;

    // Initial request should succeed
    let result = client.verify_url(context::current(), "https://github.com/test/repo".into()).await;
    assert!(result.is_ok());

    // Shutdown server
    cleanup_test_environment();

    // Request after shutdown should fail
    let result = client.verify_url(context::current(), "https://github.com/test/repo".into()).await;
    assert!(matches!(result, Err(TokenGenErrors::RpcError(_))));

    Ok(())
}

#[tokio::test]
async fn test_timeout_handling() -> Result<()> {
    let client = setup_test_environment().await?;

    // Create context with timeout
    let mut ctx = context::current();
    ctx.deadline = Some(std::time::SystemTime::now() + Duration::from_millis(1));

    // Sleep in server to trigger timeout
    tokio::time::sleep(Duration::from_millis(10)).await;

    let result = client.verify_url(ctx, "https://github.com/test/repo".into()).await;
    assert!(matches!(result, Err(TokenGenErrors::RpcError(_))));

    cleanup_test_environment();
    Ok(())
}
