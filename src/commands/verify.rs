use git2::Repository;
use std::{env, fs, path::Path};
use url::Url;

use crate::{
    errors::TokenGenErrors,
    utils::{
        helpers::{is_running_test, is_valid_github_url},
        verify_helper::verify_contract,
    },
    variables::{SUB_FOLDER, SUI_GITREPO_DIR},
    Result,
};

pub async fn verify_token_from_path(path: &str) -> Result<()> {
    let path = Path::new(path);

    if !path.exists() {
        return Err(TokenGenErrors::InvalidPath(
            "The provided path for the contract or .move file is invalid.".to_string(),
        ));
    }

    if path.is_dir() {
        // Check for sources sub-folder in contract
        let sources_folder = path.join(SUB_FOLDER);
        if sources_folder.exists() && sources_folder.is_dir() {
            // Call verify function
            verify_contract(&sources_folder).await?;
        } else {
            // Call verify function
            verify_contract(path).await?;
        }
    } else {
        return Err(TokenGenErrors::InvalidPath(
            "The file is not a .move file or directory".to_string(),
        ));
    }
    Ok(())
}

pub async fn verify_token_using_url(url: &str) -> Result<()> {
    // Parse the URL to check if it is valid
    Url::parse(url).map_err(|_| {
        TokenGenErrors::InvalidUrl("The provided URL is not a valid URL.".to_string())
    })?;

    if !is_valid_github_url(url) {
        return Err(TokenGenErrors::InvalidUrl(
            "The provided URL is not a valid GitHub URL.".to_string(),
        ));
    }

    let clone_path = Path::new(SUI_GITREPO_DIR.trim_start_matches("./"));
    match Repository::clone(url, clone_path) {
        Ok(_) => {
            let current_dir = env::current_dir().map_err(TokenGenErrors::FileIoError)?;
            let templates_path: String = format!(
                "{}/{}/{}",
                current_dir.display(),
                SUI_GITREPO_DIR.trim_start_matches("./"),
                SUB_FOLDER
            );

            // Convert to a Path reference
            let templates_path_ref: &Path = Path::new(&templates_path);

            // Check if the folder exists
            if templates_path_ref.exists() && templates_path_ref.is_dir() {
                // Call verify function
                verify_contract(templates_path_ref).await?;
            } else {
                return Err(TokenGenErrors::InvalidPath(
                    "Cloned repo not found".to_string(),
                ));
            }
            check_cloned_contract(Path::new(SUI_GITREPO_DIR.trim_start_matches("./")))?;
        }
        Err(e) => {
            return Err(TokenGenErrors::GitError(e));
        }
    }
    Ok(())
}

fn check_cloned_contract(path: &Path) -> Result<()> {
    if path.exists() && path.is_dir() && !is_running_test() {
        if let Err(e) = fs::remove_dir_all(path) {
            return Err(TokenGenErrors::FileIoError(e));
        }
    }
    Ok(())
}
