use anyhow::Result;
use git2::Repository;
use std::{env, fs, path::Path};
use url::Url;

use crate::utils::{
    errors::TokenGenErrors,
    generation::generate_token,
    helpers::{filter_token_content, get_token_info, is_valid_repository_url, sanitize_repo_name},
    variables::{TokenDetails, SUB_FOLDER},
};

/*
   Check url is valid and clone into local folder
   Take all .move files in that folder
   Read the file content and extract token details
   Genarate new token with that data
   Compare that newly created contract with user given contract
*/
pub async fn verify_token_using_url(url: &str) -> Result<(), TokenGenErrors> {
    let repo_name = validate_url(url)?;

    let clone_path = Path::new(&repo_name);

    // Ensure the cloned contract is in a good state
    check_cloned_contract(clone_path)?;

    // Get the current directory
    let current_dir = env::current_dir()
        .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read current dir: {}", e)))?;

    // Clone the repository
    Repository::clone(url, clone_path).map_err(|e| TokenGenErrors::GitError(e.to_string()))?;

    let sources_path: String = format!("{}/{}/{}", current_dir.display(), repo_name, SUB_FOLDER);

    // Convert to a Path reference
    let sources_path_ref: &Path = Path::new(&sources_path);

    // Ensure the cloned repository contains the required folder
    if !sources_path_ref.exists() || !sources_path_ref.is_dir() {
        return Err(TokenGenErrors::InvalidPath(
            "Cloned repo not found".to_string(),
        ));
    }

    // Read the directory entries
    let entries =
        fs::read_dir(sources_path_ref).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;

    // Find the first `.move` file
    let mut current_content = String::new();
    for entry in entries {
        let entry = entry.map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|e| e == "move") {
            // Read the `.move` file content
            current_content =
                read_file(&path).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
            break; // Exit the loop after finding the first .move file
        }
    }

    // Return an error if no `.move` file was found
    if current_content.is_empty() {
        return Err(TokenGenErrors::InvalidPathNoMoveFiles);
    }

    compare_contract_content(current_content, Some(clone_path))?;

    // Clean the repo
    check_cloned_contract(clone_path)?;
    Ok(())
}

pub fn read_file(file_path: &Path) -> io::Result<String> {
    Ok(fs::read_to_string(file_path)?)
}

pub fn compare_contract_content(
    current_content: String,
    clone_path: Option<&Path>,
) -> Result<(), TokenGenErrors> {
    // Filtering file content
    let cleaned_current_content: String = filter_token_content(&current_content);

    // Extracting token details from that file
    let details: TokenDetails = get_token_info(&cleaned_current_content);

    // Generating new token with these extracted details
    let expected_content: String = generate_token(
        details.decimals,
        details.symbol.clone(),
        details.name.to_string(),
        details.description.clone(),
        details.is_frozen,
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

fn validate_url(url: &str) -> Result<String, TokenGenErrors> {
    // Parse the URL to check if it is valid
    Url::parse(url).map_err(|_| TokenGenErrors::InvalidUrl)?;

    is_valid_repository_url(url)?;

    // Extract the repository name from the URL
    let name = url
        .trim_end_matches(".git")
        .rsplit('/')
        .next()
        .ok_or_else(|| {
            TokenGenErrors::InvalidRepo
        })?;

    Ok(sanitize_repo_name(name))
}

fn check_cloned_contract(path: &Path) -> Result<(), TokenGenErrors> {
    if path.exists() && path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
    }
    Ok(())
}
