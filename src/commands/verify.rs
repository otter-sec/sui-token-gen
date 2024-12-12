use std::path::Path;
use tarpc::context;

use crate::{
    TokenGenErrors,
    Result,
    utils::verify_helper::verify_contract,
    variables::SUB_FOLDER,
    rpc::TokenGen,
};

pub async fn verify_token_from_path(path: &str, client: &impl TokenGen) -> Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(TokenGenErrors::InvalidPath(
            "The provided path for the contract is invalid.".to_string(),
        ));
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
        return Err(TokenGenErrors::InvalidPath(
            "The path is not a directory.".to_string(),
        ));
    }
    println!("Verified successfully");
    Ok(())
}

pub async fn verify_token_using_url(url: &str, client: &impl TokenGen) -> Result<()> {
    client.verify_url(context::current(), url.to_string()).await?;
    Ok(())
}
