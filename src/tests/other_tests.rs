use tarpc::context;

use crate::{
    commands::verify::verify_token_using_url,
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
async fn environment_specific_token_creation() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;
    for env in ["devnet", "testnet", "mainnet"] {
        let result = client
            .create(
                context::current(),
                6,
                "TestToken".to_string(),
                "TEST".to_string(),
                "Test Description".to_string(),
                false,
                env.to_string(),
            )
            .await;
        assert!(
            result.is_ok(),
            "Token creation failed for environment: {}",
            env
        );
    }
    Ok(())
}

#[tokio::test]
async fn verify_token_rpc_error_mapping() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test invalid URL scenario
    let invalid_url = "https://invalid-url-that-does-not-exist";
    let result = verify_token_using_url(invalid_url, client.to_owned()).await;
    assert!(result.is_err());

    // Test malformed URL scenario
    let malformed_url = "not-a-url";
    let result = verify_token_using_url(malformed_url, client).await;
    assert!(result.is_err());
    Ok(())
}

#[tokio::test]
async fn error_propagation_flow() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test invalid decimals
    let result = client
        .create(
            context::current(),
            255, // Invalid decimals
            "TestToken".to_string(),
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await?;
    assert!(result.is_err());

    // Test empty name
    let result = client
        .create(
            context::current(),
            6,
            "".to_string(), // Empty name
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await?;
    assert!(result.is_err());

    // Test invalid environment should be succeed. i.e if environment invalid 'devnet' as default
    let result = client
        .create(
            context::current(),
            6,
            "TestToken".to_string(),
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "invalid_env".to_string(),
        )
        .await?;
    assert!(result.is_ok());

    Ok(())
}

#[tokio::test]
async fn test_url_validation_errors() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test non-GitHub URL
    let non_github_url = "https://gitlab.com/some/repo";
    let result = verify_token_using_url(non_github_url, client.to_owned()).await;
    assert!(result.is_err());

    // Test malformed URL
    let malformed_url = "not-a-url";
    let result = verify_token_using_url(malformed_url, client.to_owned()).await;
    assert!(result.is_err());

    // Test non-existent GitHub URL
    let invalid_url = "https://github.com/invalid/repo";
    let result = verify_token_using_url(invalid_url, client).await;
    assert!(result.is_err());

    Ok(())
}

#[tokio::test]
async fn test_path_validation_errors() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test non-existent path
    let non_existent_path = "/path/does/not/exist";
    let result = client
        .verify_content(context::current(), non_existent_path.to_string())
        .await?;
    assert!(result.is_err());

    // Test path that's not a directory
    let not_dir_path = "/etc/hosts";
    let result = client
        .verify_content(context::current(), not_dir_path.to_string())
        .await?;
    assert!(result.is_err());

    Ok(())
}
