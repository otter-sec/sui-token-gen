use git2::Repository;
use std::env;
use std::fs;
use std::io::{self};
use std::path::Path;
use url::Url;

use crate::utils::helpers::is_valid_github_url;
use crate::utils::verify_helper::verify_contract;

use crate::variables::{SUB_FOLDER, SUI_GITREPO_DIR};

pub async fn verify_token_from_path(path: &str) -> io::Result<()> {
    let path = Path::new(path);

    assert!(
        path.exists(),
        "The provided path for the contract or .move file is invalid."
    );

    if path.is_dir() {
        //Check for sources sub folder in contract
        let sources_folder = path.join(SUB_FOLDER);
        if sources_folder.exists() && sources_folder.is_dir() {
            //Call verify function
            verify_contract(&sources_folder).await?;
        } else {
            //Call verify function
            verify_contract(path).await?;
        }
    } else {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "The file is not a .move file or directory",
        ));
    }
    Ok(())
}

pub async fn verify_token_using_url(url: &str) -> io::Result<()> {

    // Parse the URL to check if it is valid
    assert!(
        Url::parse(url).is_ok(),
        "The provided URL is not a valid URL."
    );

    assert!(
        is_valid_github_url(url),
        "The provided URL is not a valid GitHub URL."
    );

    let clone_path = Path::new(SUI_GITREPO_DIR.trim_start_matches("./"));
    match Repository::clone(url, clone_path) {
        Ok(_) => {
            let current_dir = env::current_dir().expect("Failed to get current directory");
            let templates_path: String = format!(
                "{}/{}/{}",
                current_dir.display(),
                SUI_GITREPO_DIR.trim_start_matches("./"),
                SUB_FOLDER
            );
            println!("templates_path: {:?}", templates_path);
            // Convert to a Path reference
            let templates_path_ref: &Path = Path::new(&templates_path);

            // Check if the folder exists
            if templates_path_ref.exists() && templates_path_ref.is_dir() {
                //Call verify function
                verify_contract(templates_path_ref).await?;
            } else {
                println!("Contract does not exist: {:?}", templates_path_ref);
            }
            check_cloned_contract(Path::new(SUI_GITREPO_DIR));
        }
        Err(e) => eprintln!("Error while cloning: {}", e),
    }
    Ok(())
}

fn check_cloned_contract(path: &Path) {
    if path.exists() && path.is_dir() {
        let _ = fs::remove_dir_all(path);
    }
}
