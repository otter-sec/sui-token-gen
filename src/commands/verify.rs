use std::path::Path;
use std::io::{self};
use url::Url;
use git2::Repository;
use std::env;
use std::fs;


use crate::utils::helpers::validate_input;
use crate::utils::verify_helper::verify_contract;

use crate::variables::{SUB_FOLDER, SUI_GITREPO_DIR};

pub async fn verify_token(path_or_url: &str) {
    /*
        status: valid input or not
        error_message: if invalid return error message
        input_type: input type (FILE or URL)
    */
    let (status, error_message, input_type): (bool, String, Option<&str>) = validate_input(path_or_url);

    if !status {
        eprintln!("{}", error_message);
        return;
    }

    if let Some(input_type) = input_type {
        let _ = fetch_data(path_or_url, input_type).await;
    }
}

//Based on input type fetching contract data
pub async fn fetch_data(path_or_url: &str, input_type: &str) -> io::Result<()> {
    match input_type {
        "FILE" => {
            let path = Path::new(path_or_url);

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
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "The file is not a .move file or directory"));
            }
        }
        "URL" => {
            match Url::parse(path_or_url) {
                Ok(_) => {
                    let clone_path = Path::new(SUI_GITREPO_DIR.trim_start_matches("./"));
                    match Repository::clone(path_or_url, clone_path) {
                        Ok(_) => {
                            let current_dir = env::current_dir().expect("Failed to get current directory");
                            let templates_path: String = format!("{}/{}/{}", current_dir.display(),SUI_GITREPO_DIR.trim_start_matches("./"), SUB_FOLDER);
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
                        },
                        Err(e) => eprintln!("Error while cloning: {}", e),
                    }
                },
                Err(_) => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Invalid URL")),
            }
        }
        _ => return Err(io::Error::new(io::ErrorKind::InvalidInput, "Unknown input type")),
    }

    Ok(())
}

fn check_cloned_contract(path: &Path) {
    if path.exists() && path.is_dir() {
        let _ = fs::remove_dir_all(path);
    }
}
