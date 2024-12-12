use std::path::Path;
use tarpc::context;

use crate::{
    TokenGenErrors,
    Result,
    rpc::TokenGen,
    utils::verify_helper::{verify_contract, verify_token_using_url},
};

pub async fn verify_token_from_path(path: &Path, client: &impl TokenGen) -> Result<()> {
    if !path.exists() {
        return Err(TokenGenErrors::InvalidPath(format!(
            "Path does not exist: {}",
            path.display()
        )));
    }

    if path.is_file() {
        let content = std::fs::read_to_string(path)?;
        let ctx = context::current();
        client.verify_content(ctx, content).await?;
    } else if path.is_dir() {
        verify_contract(path, client).await?;
    } else {
        return Err(TokenGenErrors::InvalidPath(format!(
            "Path is neither a file nor a directory: {}",
            path.display()
        )));
    }

    Ok(())
}

pub async fn verify_token_using_url(url: &str, client: &impl TokenGen) -> Result<()> {
    let ctx = context::current();
    client.verify_url(ctx, url.to_string()).await?;
    Ok(())
}
