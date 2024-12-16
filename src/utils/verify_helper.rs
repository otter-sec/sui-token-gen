use std::{fs, io, path::Path};

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
   2. A `.move` file with the same name as the project exists inside the `sources` folder.
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

    // Extract the project name from the provided path.
    let project_name = path
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| TokenGenErrors::InvalidPath("Invalid project name.".to_string()))?;

    // Construct the expected file name for the `.move` file based on the project name.
    let contract_file_name = format!("{}.move", project_name);
    let contract_path = sources_folder.join(&contract_file_name);

    // Check if the `.move` file exists and is a valid file.
    if !contract_path.exists() || !contract_path.is_file() {
        return Err(TokenGenErrors::InvalidPath(format!(
            "The provided path doesn't have a `.move` file named '{}'.",
            contract_file_name
        )));
    }

    // Read the content of the `.move` file.
    let current_content: String =
        read_file(&contract_path).map_err(|e| TokenGenErrors::FileIoError(e))?;

    // Ensure the `.move` file is not empty.
    if current_content.is_empty() {
        return Err(TokenGenErrors::InvalidPath(
            "The contract file is empty.".to_string(),
        ));
    }

    // Return the content of the `.move` file.
    Ok(current_content)
}
