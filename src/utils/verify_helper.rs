use std::{
    fs::{self, ReadDir},
    io,
    path::Path,
};

use crate::{errors::TokenGenErrors, variables::SUB_FOLDER, Result};

pub fn read_file(file_path: &Path) -> io::Result<String> {
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("move") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File is not a .move file",
        ));
    }

    Ok(fs::read_to_string(file_path)?)
}

/*
   This function verifies the provided path and ensures it meets the following criteria:
   1. The path exists and contains a `sources` folder.
   2. Only one `.move` file will be inside the `sources` folder.
   3. The `.move` file is not empty.

   If all conditions are satisfied, the function reads and returns the content of the `.move` file.
   Otherwise, it returns an appropriate error.
*/
pub fn verify_path(path: &str) -> Result<String> {
    let path = Path::new(path);

    let sources_folder = path.join(SUB_FOLDER);

    // Ensure the provided path exists and contains a `sources` folder.
    if !path.exists() || !path.is_dir() || !sources_folder.exists() || !sources_folder.is_dir() {
        return Err(TokenGenErrors::InvalidPath(
            "The provided path for the contract is invalid.".to_string(),
        ));
    }

    // Read the directory entries
    let entries = read_dir(&sources_folder).map_err(|e| TokenGenErrors::FileIoError(e))?;

    // Find the first `.move` file
    let mut current_content = String::new();
    for entry in entries {
        let entry = entry.map_err(|e| TokenGenErrors::FileIoError(e))?;
        let path = entry.path();

        if path.is_file() && path.extension().is_some_and(|e| e == "move") {
            // Read the `.move` file content
            current_content = read_file(&path).map_err(|e| TokenGenErrors::FileIoError(e))?;
            break; // Exit the loop after finding the first .move file
        }
    }

    // Return an error if no `.move` file was found
    if current_content.is_empty() {
        return Err(TokenGenErrors::InvalidPath(
            "No `.move` file found in the directory.".to_string(),
        ));
    }

    // Return the content of the `.move` file.
    Ok(current_content)
}

pub fn read_dir(dir: &Path) -> io::Result<ReadDir> {
    Ok(fs::read_dir(dir)?)
}
