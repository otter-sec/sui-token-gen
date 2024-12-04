use std::fs;
use std::io::{self};
use std::path::Path;


use crate::utils::helpers::{filter_token_content, get_token_info};
use crate::utils::generation::generate_token;


pub fn read_file(file_path: &Path) -> io::Result<String> {
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("move") {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "File is not a .move file"));
    }

    let content = fs::read_to_string(file_path)?;
    Ok(content)
}

/*
    Check dir is directory or not
    Take all .move files in that folder
    Read the file content and extract token details
    Genarate new token with that data
    Compare that newly created contract with user given contract
 */
pub async fn verify_contract(dir: &Path) -> io::Result<()> {
    if !dir.is_dir() {
        return Err(io::Error::new(io::ErrorKind::InvalidInput, "Path is not a directory"));
    }
    let entries = fs::read_dir(dir)
        .map_err(|e| io::Error::new(io::ErrorKind::Other, e.to_string()))?;


    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().map(|e| e == "move").unwrap_or(false) {
                    //Reading file
                    match read_file(&path) {
                        Ok(current_content) => {
                            //Filtering file content
                            let cleaned_current_content = filter_token_content(&current_content);

                            //Extracting token details from that file
                            let details: (u8, String, String, String, bool) = get_token_info(&current_content);

                            //Generating new token with these extracted details
                            let expected_content = generate_token(details.0, details.1, &details.2, details.3, details.4);

                            //Filtering newly created token content
                            let cleaned_expected_content = filter_token_content(&expected_content);

                            // println!("{:?}", cleaned_expected_content);
                            // println!("{:?}", cleaned_current_content);

                            //Comparing both expected contract and user contract
                            if cleaned_current_content == cleaned_expected_content {
                                println!("The contents are the same.");
                            } else {
                                println!("The contents are different.");
                            }
                        }
                        Err(e) => {
                            eprintln!("Error reading .move file: {}", e);
                        }
                    }
                }
            }
            Err(e) => eprintln!("Error reading entry in directory: {}", e),
        }
    }
    Ok(())
}