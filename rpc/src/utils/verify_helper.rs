use anyhow::Result;
use git2::Repository;
use std::{env, fs, path::Path};
use url::Url;

use crate::utils::{
    errors::TokenGenErrors,
    generation::generate_token,
    helpers::{filter_token_content, get_token_info, is_valid_github_url},
    variables::SUB_FOLDER,
};

pub async fn verify_token_using_url(url: &str) -> Result<(), TokenGenErrors> {
    // Parse the URL to check if it is valid
    Url::parse(url).map_err(|_| {
        TokenGenErrors::InvalidUrl("The provided URL is not a valid URL.".to_string())
    })?;

    if !is_valid_github_url(url) {
        return Err(TokenGenErrors::InvalidUrl(
            "The provided URL is not a valid GitHub URL.".to_string(),
        ));
    }

    // Extract the repository name from the URL
    let repo_name = url
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .ok_or_else(|| {
            TokenGenErrors::InvalidUrl("Failed to extract repository name.".to_string())
        })?;

    let clone_path = Path::new(repo_name);

    // Ensure the cloned contract is in a good state
    check_cloned_contract(clone_path)?;

    // Clone the repository
    Repository::clone(url, clone_path).map_err(|e| TokenGenErrors::GitError(e.to_string()))?;

    // Get the current directory
    let current_dir = env::current_dir()
        .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read current dir: {}", e)))?;

    let templates_path: String = format!("{}/{}/{}", current_dir.display(), repo_name, SUB_FOLDER);

    // Convert to a Path reference
    let templates_path_ref: &Path = Path::new(&templates_path);

    // Ensure the cloned repository contains the required folder
    if templates_path_ref.exists() && templates_path_ref.is_dir() {
        // Call verify function
        verify_contract(templates_path_ref, clone_path).await?;

        // Ensure the cloned contract is clean after verification
        check_cloned_contract(clone_path)?;
    } else {
        return Err(TokenGenErrors::InvalidPath(
            "Cloned repo not found".to_string(),
        ));
    }

    Ok(())
}

fn check_cloned_contract(path: &Path) -> Result<(), TokenGenErrors> {
    if path.exists() && path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
    }
    Ok(())
}

pub fn read_file(file_path: &Path) -> Result<String, TokenGenErrors> {
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("move") {
        return Err(TokenGenErrors::FileIoError(
            "File is not a .move file".to_string(),
        ));
    }

    let content =
        fs::read_to_string(file_path).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
    Ok(content)
}

/*
   Check dir is directory or not
   Take all .move files in that folder
   Read the file content and extract token details
   Genarate new token with that data
   Compare that newly created contract with user given contract
*/
pub async fn verify_contract(dir: &Path, clone_path: &Path) -> Result<(), TokenGenErrors> {
    // Ensure the given path is a directory
    if !dir.is_dir() {
        return Err(TokenGenErrors::InvalidPath(
            "Path is not a directory".to_string(),
        ));
    }

    // Read all entries in the directory
    let entries = fs::read_dir(dir)
        .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read directory: {}", e)))?;

    // Iterate over the entries
    for entry in entries {
        let entry = entry.map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
        let path = entry.path();

        // Check if the entry is a file with a `.move` extension
        if path.is_file() && path.extension().is_some_and(|e| e == "move") {
            // Read the contract file content
            let current_content = read_file(&path)?;

            // Compare contract content, propagate errors if any
            compare_contract_content(current_content, Some(clone_path))?;
        }
    }

    Ok(()) // Success: No modifications detected
}

pub fn compare_contract_content(
    current_content: String,
    clone_path: Option<&Path>,
) -> Result<(), TokenGenErrors> {
    // Filtering file content
    let cleaned_current_content: String = filter_token_content(&current_content);

    // Extracting token details from that file
    let details: (u8, String, String, String, bool) = get_token_info(&current_content);

    // Generating new token with these extracted details
    let expected_content: String = generate_token(
        details.0,
        details.1,
        &details.2,
        details.3.to_owned(),
        details.4,
        false,
    );

    // Filtering newly created token content
    let cleaned_expected_content: String = filter_token_content(&expected_content);

    // Comparing both expected contract and user contract
    if cleaned_current_content != cleaned_expected_content {
        // Ensure the cloned contract is clean after verification
        // only if clone_path exists
        if let Some(path) = clone_path {
            check_cloned_contract(path)?;
        }

        return Err(TokenGenErrors::VerifyResultError(
            "Content mismatch detected".to_string(),
        ));
    }

    Ok(()) // Return success if the contract is not modified
}
