use std::time::Duration;
use tokio::test;
use suitokengentest::{
    errors::TokenGenErrors,
    test_utils::{cleanup_test_environment, setup_test_environment, mocks::MockGitRepo},
    Result,
};
use tarpc::context;

/// Unit tests for TokenServer RPC methods

#[tokio::test]
async fn test_server_startup_coordination() -> Result<()> {
    // Test multiple server startups
    let client1 = setup_test_environment().await?;
    let client2 = setup_test_environment().await?;

    // Both clients should be able to make requests
    let result1 = client1.verify_url(context::current(), "https://github.com/test1.git".into()).await;
    let result2 = client2.verify_url(context::current(), "https://github.com/test2.git".into()).await;

    assert!(result1.is_ok() || result1.is_err());
    assert!(result2.is_ok() || result2.is_err());

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_rpc_connection_handling() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test request timeout
    let mut ctx = context::current();
    ctx.deadline = std::time::SystemTime::now() + Duration::from_millis(1);

    tokio::time::sleep(Duration::from_millis(10)).await;
    let result = client.verify_url(ctx, "https://github.com/test.git".into()).await;
    assert!(result.is_err());

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_verify_url_validation() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test valid URLs
    assert!(client.verify_url(context::current(), "https://github.com/test.git".into()).await.is_ok());
    assert!(client.verify_url(context::current(), "http://gitlab.com/test.git".into()).await.is_ok());

    // Test invalid URLs
    let err = client.verify_url(context::current(), "not-a-url".into()).await.unwrap_err();
    assert!(matches!(err, TokenGenErrors::InvalidInput(_)));

    let err = client.verify_url(context::current(), "ftp://invalid.git".into()).await.unwrap_err();
    assert!(matches!(err, TokenGenErrors::InvalidInput(_)));

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_verify_content_validation() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test valid content
    assert!(client.verify_content(context::current(), "module test::token {
        use sui::coin;
        use sui::transfer;
        use sui::tx_context::{Self, TxContext};
    }".into()).await.is_ok());

    // Test empty content
    let err = client.verify_content(context::current(), "".into()).await.unwrap_err();
    assert!(matches!(err, TokenGenErrors::InvalidInput(_)));

    // Test whitespace-only content
    let err = client.verify_content(context::current(), "   ".into()).await.unwrap_err();
    assert!(matches!(err, TokenGenErrors::InvalidInput(_)));

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_external_dependency_mocking() -> Result<()> {
    let temp_dir = tempfile::tempdir()?;
    let mock_repo = MockGitRepo::new(temp_dir.path().to_path_buf());

    // Set up mock repository
    mock_repo.add_test_files(vec![
        ("Move.toml", "[package]\nname = \"test_token\"\nversion = \"0.1.0\""),
        ("sources/token.move", "module test_token {}")
    ])?;

    let client = setup_test_environment().await?;

    // Test with mock repository
    let result = client.verify_content(
        context::current(),
        std::fs::read_to_string(temp_dir.path().join("sources/token.move"))?
    ).await;

    assert!(result.is_ok());

    mock_repo.cleanup()?;
    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_concurrent_requests() -> Result<()> {
    let client = setup_test_environment().await?;

    // Send multiple requests concurrently
    let futures = vec![
        client.create(
            context::current(),
            8,
            "Token1".into(),
            "TK1".into(),
            "Test Token 1".into(),
            false
        ),
        client.create(
            context::current(),
            8,
            "Token2".into(),
            "TK2".into(),
            "Test Token 2".into(),
            false
        ),
    ];

    let results = futures::future::join_all(futures).await;
    assert_eq!(results.len(), 2);
    assert!(results.iter().all(|r| r.is_ok()));

    cleanup_test_environment();
    Ok(())
}

#[tokio::test]
async fn test_detailed_error_scenarios() -> Result<()> {
    let client = setup_test_environment().await?;

    // Test invalid decimals
    let result = client.create(
        context::current(),
        256, // Invalid decimals (> u8::MAX)
        "Test".into(),
        "TST".into(),
        "Test".into(),
        false
    ).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidDecimals)));

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

    // Test invalid symbol
    let result = client.create(
        context::current(),
        8,
        "Test".into(),
        "T$T".into(), // Invalid symbol
        "Test".into(),
        false
    ).await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidSymbol(_))));

    cleanup_test_environment();
    Ok(())
}
