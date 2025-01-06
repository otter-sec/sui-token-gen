use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    utils::client::rpc_client::TokenGenClient,
    handlers::{handle_success, SuccessType},
    utils::{helpers::is_valid_repository_url, verify_helper::verify_path},
    Result,
};

/**
 * Verifies a token contract using a local file path.
 *
 * This function performs the following steps:
 * 1. Validates the provided file path to ensure it contains a valid `.move` contract file.
 * 2. Reads the contract content from the specified path.
 * 3. Sends the contract content to the RPC client for verification.
 * 4. Logs the success if verification is successful, or returns an appropriate error if verification fails.
 *
 * # Parameters
 * - `path`: A string slice that represents the local file path to the token contract.
 * - `client`: An instance of `TokenGenClient` used to interact with the verification RPC service.
 *
 * # Returns
 * - `Ok(())` if the contract is successfully verified.
 * - `Err(TokenGenErrors)` if any validation or verification step fails.
 */
pub async fn verify_token_from_path(path: &str, client: TokenGenClient) -> Result<()> {
    // Validate the file path and ensure it contains valid contract content.
    let current_content = verify_path(path)?;

    // Send the contract content to the RPC client for verification.
    client
        .verify_content(context::current(), current_content)
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;

    // Log success message if verification is successful.
    handle_success(SuccessType::TokenVerified {
        path: Some(path.to_string()),
        url: None,
    });

    Ok(())
}

/**
 * Verifies a token contract using a URL pointing to a Git repository.
 *
 * This function performs the following steps:
 * 1. Validates the provided URL to ensure it is a valid Git repository URL.
 * 2. Sends the URL to the RPC client for verification.
 * 3. Logs the success if verification is successful, or returns an appropriate error if verification fails.
 *
 * # Parameters
 * - `url`: A string slice that represents the URL pointing to the token contract (e.g., GitHub or GitLab repository).
 * - `client`: An instance of `TokenGenClient` used to interact with the verification RPC service.
 *
 * # Returns
 * - `Ok(())` if the contract is successfully verified.
 * - `Err(TokenGenErrors)` if any validation or verification step fails.
 */
pub async fn verify_token_using_url(url: &str, client: TokenGenClient) -> Result<()> {
    // Validate the URL to ensure it is a valid Git repository URL.
    is_valid_repository_url(url)?;

    // Send the URL to the RPC client for verification.
    client
        .verify_url(context::current(), url.to_string())
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;

    // Log success message if verification is successful.
    handle_success(SuccessType::TokenVerified {
        path: None,
        url: Some(url.to_string()),
    });

    Ok(())
}
