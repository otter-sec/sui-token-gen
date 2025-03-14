use tarpc::context;

use super::common::test_utils::setup_test_client;
use crate::{constants::ADDRESS, errors::TokenGenErrors, Result};

// Test case to simulate a failed client connection due to an invalid address.
// This tests the scenario where the address provided for the client setup is incorrect (e.g., wrong port or unavailable address).
#[tokio::test]
async fn rpc_client_connection_failure() -> Result<()> {
    // Set an invalid address (wrong port or address not available)
    let invalid_address: &str = "127.0.0.1:5002";

    // Attempt to set up the client with the invalid address
    let result = setup_test_client(invalid_address).await;

    // Assert that the result is an error, and it matches the specific error type
    assert!(result.is_err()); // Ensure that the result is an error
    assert!(matches!(result, Err(TokenGenErrors::FailedToConnectRpc))); // Ensure that the error is of type InvalidInput

    Ok(())
}

// Test case to verify the error handling when interacting with the test client after successful setup.
// It simulates a failed RPC request (e.g., when trying to verify content that doesn't exist).
#[tokio::test]
async fn setup_test_client_error_handling() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test error handling during content verification with invalid input
    let result = client
        .verify_content(context::current(), "invalid content".to_string(), "invalid toml".to_string()) // Simulate invalid content
        .await
        .map_err(TokenGenErrors::RpcError)?; // Map the RPC error to TokenGenErrors

    // Assert that the result is an error
    assert!(result.is_err()); // Ensure that the error occurs as expected

    Ok(())
}

// Test case to simulate the propagation of errors from the RPC client.
// It tests the scenario where errors occur during RPC calls (like verifying content or URL verification) and ensures that the errors are propagated properly.
#[tokio::test]
async fn test_error_propagation() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test RPC error propagation during content verification with invalid input
    let result = client
        .verify_content(context::current(), "invalid content".to_string(), "invalid toml".to_string()) // Simulate invalid content
        .await?;
    assert!(result.is_err()); // Ensure that the error occurs as expected

    // Test URL verification error propagation with an invalid URL
    let result = client
        .verify_url(context::current(), "https://invalid-url".to_string()) // Simulate an invalid URL
        .await?;
    assert!(result.is_err()); // Ensure that the error occurs as expected

    Ok(())
}
