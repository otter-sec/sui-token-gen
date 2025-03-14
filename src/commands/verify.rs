use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    handlers::{handle_success, SuccessType},
    utils::client::rpc_client::TokenGenClient,
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
    let verify_data = verify_path(path)?;

    // Send the contract content to the RPC client for verification.
    client
        .verify_content(context::current(), verify_data.content)
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;

    // Log success message if verification is successful.
    handle_success(SuccessType::TokenVerified {
        path: Some(path.to_string()),
        url: None,
        address: None,
        environment: None,
        file_name: Some(verify_data.file_name),
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
    let verification_result = client
        .verify_url(context::current(), url.to_string())
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;

    // Log success message if verification is successful.
    handle_success(SuccessType::TokenVerified {
        path: None,
        url: Some(url.to_string()),
        address: None,
        environment: None,
        file_name: Some(verification_result),
    });

    Ok(())
}

/**
 * Verifies a token contract using its blockchain address and environment.
 *
 * This function performs the following steps:
 * 1. Validates the provided blockchain address format.
 * 2. Sends the address and environment to the RPC client for verification.
 * 3. Logs the success if verification is successful, or returns an appropriate error if verification fails.
 *
 * # Parameters
 * - `address`: A string slice representing the token contract's blockchain address.
 * - `environment`: A string slice representing the blockchain environment (`mainnet`, `devnet`, `testnet`).
 * - `client`: An instance of `TokenGenClient` used to interact with the verification RPC service.
 *
 * # Returns
 * - `Ok(())` if the token address is successfully verified.
 * - `Err(TokenGenErrors)` if any validation or verification step fails.
 */
pub async fn verify_token_address(
    address: &str,
    environment: &str,
    client: TokenGenClient,
) -> Result<()> {
    // Send the address and environment to the RPC client for verification.
    client
        .verify_address(
            context::current(),
            address.to_string(),
            environment.to_string(),
        )
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;
    // Log success message if verification is successful.
    handle_success(SuccessType::TokenVerified {
        path: None,
        url: None,
        address: Some(address.to_string()),
        environment: Some(environment.to_string()),
        file_name: None,
    });

    Ok(())
}
