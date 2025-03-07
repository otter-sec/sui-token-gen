use tarpc::context;

use super::common::test_utils::setup_test_client;
use crate::{commands::verify::verify_token_using_url, constants::ADDRESS, Result};

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
