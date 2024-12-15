use std::{
    fs::{self, ReadDir},
    io,
    path::Path,
};
use tarpc::context;

use crate::{errors::TokenGenErrors, rpc_client::TokenGenClient, Result};

pub fn read_file(file_path: &Path) -> io::Result<String> {
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("move") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File is not a .move file",
        ));
    }

    Ok(fs::read_to_string(file_path)?)
}

pub fn read_dir(dir: &Path) -> io::Result<ReadDir> {
    Ok(fs::read_dir(dir)?)
}

/*
   Check dir is directory or not
   Take all .move files in that folder
   Call verify_content function from RPC
*/
pub async fn verify_contract(dir: &Path, client: TokenGenClient) -> Result<()> {
    // Ensure the path is a directory
    if !dir.is_dir() {
        return Err(TokenGenErrors::InvalidPath(
            "Provided path is not a directory.".to_string(),
        ));
    }

    // Read the directory entries
    let entries = read_dir(dir).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;

    // Find the first `.move` file
    let mut current_content = String::new();
    for entry in entries {
        let entry = entry.map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|e| e == "move") {
            // Read the `.move` file content
            current_content = read_file(&path).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
            break; // Exit the loop after finding the first .move file
        }
    }

    // Return an error if no `.move` file was found
    if current_content.is_empty() {
        return Err(TokenGenErrors::InvalidPath(
            "No `.move` file found in the directory.".to_string(),
        ));
    }

    // Verify the content using the client
    client
        .verify_content(context::current(), current_content)
        .await
        .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?
        .map_err(|e| TokenGenErrors::VerificationError(e.to_string()))?;
    Ok(())
}
