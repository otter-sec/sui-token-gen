use tarpc::context;

use crate::{
    commands::verify::verify_token_using_url,
    errors::TokenGenErrors,
    rpc_client::{initiate_client, TokenGenClient},
    variables::ADDRESS,
    Result,
};

// Helper function to set up a test client with consistent error handling
// It initializes the client and returns a Result, mapping any errors to TokenGenErrors::InvalidInput
pub async fn setup_test_client(address: &str) -> Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|_| TokenGenErrors::FailedToConnectRpc)
}

// Test case to verify token creation works across different environments (devnet, testnet, mainnet)
// It attempts to create a token in each environment and asserts that the token creation is successful for all environments.
#[tokio::test]
async fn environment_specific_token_creation() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Loop through different environments and test token creation
    for env in ["devnet", "testnet", "mainnet"] {
        let result = client
            .create(
                context::current(),
                6, // 6 decimal places
                "TestToken".to_string(),
                "TEST".to_string(),
                "Test Description".to_string(),
                false,
                env.to_string(), // Environment for deployment
            )
            .await;

        // Assert that token creation succeeds for each environment
        assert!(
            result.is_ok(),
            "Token creation failed for environment: {}",
            env
        );
    }
    Ok(())
}

// Test case to verify error handling when verifying token using a URL
// It checks the behavior when invalid or malformed URLs are passed to the verification function.
#[tokio::test]
async fn verify_token_rpc_error_mapping() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test with an invalid URL
    let invalid_url = "https://invalid-url-that-does-not-exist";
    let result = verify_token_using_url(invalid_url, client.to_owned()).await;
    assert!(result.is_err()); // Assert that the result is an error

    // Test with a malformed URL
    let malformed_url = "not-a-url";
    let result = verify_token_using_url(malformed_url, client).await;
    assert!(result.is_err()); // Assert that the result is an error

    Ok(())
}

// Test case for ensuring that errors are properly propagated when invalid parameters are provided
// It checks different types of invalid input (such as invalid decimals, empty token name, and invalid environment).
#[tokio::test]
async fn error_propagation_flow() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test invalid decimal places (255 is too high)
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
    assert!(result.is_err()); // Assert that an error occurs due to invalid decimals

    // Test empty token name (should be invalid)
    let result = client
        .create(
            context::current(),
            6,
            "".to_string(), // Empty token name
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await?;
    assert!(result.is_err()); // Assert that an error occurs due to empty name

    // Test invalid environment (environment should fall back to 'devnet' if invalid)
    let result = client
        .create(
            context::current(),
            6,
            "TestToken".to_string(),
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "invalid_env".to_string(), // Invalid environment
        )
        .await?;
    assert!(result.is_ok()); // Assert that the result is ok even for an invalid environment (defaulting to 'devnet')

    Ok(())
}

// Test case to verify URL validation errors when verifying token via URL
// It checks that only valid GitHub URLs are accepted, and tests non-GitHub, malformed, and non-existent URLs.
#[tokio::test]
async fn test_url_validation_errors() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test non-GitHub URL (GitLab URL should be invalid)
    let non_github_url = "https://gitlab.com/some/repo";
    let result = verify_token_using_url(non_github_url, client.to_owned()).await;
    assert!(result.is_err()); // Assert that the result is an error for non-GitHub URL

    // Test malformed URL (should be invalid)
    let malformed_url = "not-a-url";
    let result = verify_token_using_url(malformed_url, client.to_owned()).await;
    assert!(result.is_err()); // Assert that the result is an error for malformed URL

    // Test non-existent GitHub URL (should return error)
    let invalid_url = "https://github.com/invalid/repo";
    let result = verify_token_using_url(invalid_url, client).await;
    assert!(result.is_err()); // Assert that the result is an error for non-existent URL

    Ok(())
}

// Test case to check for path validation errors
// It tests scenarios where a non-existent path or a path that is not a directory is provided for verification.
#[tokio::test]
async fn test_path_validation_errors() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test non-existent path
    let non_existent_path = "/path/does/not/exist";
    let result = client
        .verify_content(context::current(), non_existent_path.to_string())
        .await?;
    assert!(result.is_err()); // Assert that the result is an error for a non-existent path

    // Test a path that is not a directory (e.g., /etc/hosts file)
    let not_dir_path = "/etc/hosts";
    let result = client
        .verify_content(context::current(), not_dir_path.to_string())
        .await?;
    assert!(result.is_err()); // Assert that the result is an error for a path that is not a directory

    Ok(())
}
