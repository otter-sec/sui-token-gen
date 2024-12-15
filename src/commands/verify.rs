use std::path::Path;
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    success_handler::{handle_success, SuccessType},
    utils::verify_helper::verify_contract,
    variables::SUB_FOLDER,
    Result,
};

pub async fn verify_token_from_path(path: &str, client: TokenGenClient) -> Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(TokenGenErrors::InvalidPath("The provided path for the contract is invalid.".to_string()));
    }

    if path.is_dir() {
        // Check for sources sub-folder in contract
        let sources_folder = path.join(SUB_FOLDER);
        if sources_folder.exists() && sources_folder.is_dir() {
            // Call verify function
            verify_contract(&sources_folder, client).await?;
        } else {
            // Call verify function
            verify_contract(path, client).await?;
        }
    } else {
        return Err(TokenGenErrors::InvalidPath("The path is not a directory.".to_string()));
    }
    handle_success(SuccessType::TokenVerified { path: Some(path.to_string_lossy().to_string()), url: None });
    Ok(())
}

pub async fn verify_token_using_url(url: &str, client: TokenGenClient) -> Result<()> {
    client
        .verify_url(context::current(), url.to_string())
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;
    handle_success(SuccessType::TokenVerified { path: None, url: Some(url.to_string()) });
    Ok(())
}
