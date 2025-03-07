use super::common::test_utils::setup_test_client;
use crate::{commands::verify::verify_token_using_url, constants::ADDRESS, Result};
use tarpc::context;

// Test case to verify URL validation errors when verifying token via URL
// It checks that only valid GitHub URLs are accepted, and tests non-GitHub, malformed, and non-existent URLs.
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
    assert!(result.is_err());

    // Test a path that is not a directory
    let not_dir_path = "/etc/hosts";
    let result = client
        .verify_content(context::current(), not_dir_path.to_string())
        .await?;
    assert!(result.is_err());

    Ok(())
}
