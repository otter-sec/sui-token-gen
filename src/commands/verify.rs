use std::path::Path;
use std::io::{self};
use url::Url;
use git2::Repository;

use crate::utils::helpers::validate_input;
use crate::utils::files::verify_contract;

use crate::variables::SUB_FOLDER;

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
                    verify_contract(&sources_folder).await?;
                } else {
                    verify_contract(path).await?;
                }
            } else {
                return Err(io::Error::new(io::ErrorKind::InvalidInput, "The file is not a .move file or directory"));
            }
        }
        "URL" => {
            match Url::parse(path_or_url) {
                Ok(_) => {
                    let clone_path = Path::new("./sui-programs");
                    match Repository::clone(path_or_url, clone_path) {
                        Ok(_) => println!("Cloned"),
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
