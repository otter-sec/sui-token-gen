use std::path::Path;
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    utils::{helpers::log_success_message, verify_helper::verify_contract},
    variables::SUB_FOLDER,
    Result,
};

pub async fn verify_token_from_path(path: &str, client: TokenGenClient) -> Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        let error = TokenGenErrors::InvalidPath("The provided path for the contract is invalid.".to_string());
        error.log();
        return Err(error);
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
        let error = TokenGenErrors::InvalidPath("The path is not a directory.".to_string());
        error.log();
        return Err(error);
    }
    log_success_message("Verified successfully");
    Ok(())
}

pub async fn verify_token_using_url(url: &str, client: TokenGenClient) -> Result<()> {
    client
        .verify_url(context::current(), url.to_string())
        .await
        .map_err(|e| {
            let error = TokenGenErrors::RpcError(e);
            error.log();
            error
        })?
        .map_err(|e| {
            let error = TokenGenErrors::VerificationError(e.to_string());
            error.log();
            error
        })?;
    log_success_message("Verified successfully");
    Ok(())
}
