use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    success_handler::{handle_success, SuccessType},
    utils::{helpers::is_valid_repository_url, verify_helper::verify_path},
    Result,
};

/*
   Verifies a token using a local file path.
   - Validates the path, ensures it contains a `.move` contract file.
   - Sends the contract content to the client for verification.
   - Logs success if verification passes or returns an error if it fails.

   Parameters:
   - `path`: Local path to the contract.
   - `client`: RPC client for verification.

   Returns:
   - `Ok(())` on success or a `TokenGenErrors` on failure.
*/
pub async fn verify_token_from_path(path: &str, client: TokenGenClient) -> Result<()> {
    let current_content = verify_path(&path)?;
    client
        .verify_content(context::current(), current_content)
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;

    handle_success(SuccessType::TokenVerified {
        path: Some(path.to_string()),
        url: None,
    });
    Ok(())
}

/*
   Verifies a token using a URL.
   - Sends the URL to the client for verification.
   - Logs success if verification passes or returns an error if it fails.

   Parameters:
   - `url`: URL pointing to the contract.
   - `client`: RPC client for verification.

   Returns:
   - `Ok(())` on success or a `TokenGenErrors` on failure.
*/
pub async fn verify_token_using_url(url: &str, client: TokenGenClient) -> Result<()> {
    // Verify it's a valid GitHub repository URL
    is_valid_repository_url(url)?;

    client
        .verify_url(context::current(), url.to_string())
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;

    handle_success(SuccessType::TokenVerified {
        path: None,
        url: Some(url.to_string()),
    });
    Ok(())
}
