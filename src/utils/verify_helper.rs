use std::{
    fs::{self, ReadDir},
    io,
    path::Path,
};

use crate::{constants::SUB_FOLDER, errors::TokenGenErrors, Result};

/**
 * Reads the content of a file at the given path.
 *
 * # Parameters
 * - `file_path`: The path to the file to be read.
 *
 * # Returns
 * - `Ok(String)`: The content of the file as a string.
 * - `Err(io::Error)`: If the file cannot be read (e.g., it doesn't exist or lacks permissions).
 */
pub fn read_file(file_path: &Path) -> io::Result<String> {
    fs::read_to_string(file_path)
}

/**
 * Verifies a directory path and validates its structure for a Move project.
 *
 * This function performs the following checks:
 * 1. Ensures that the provided path exists, is a directory, and contains a `sources` folder.
 * 2. Checks the `sources` folder for `.move` files, ensuring exactly one `.move` file exists.
 * 3. Validates that the `.move` file is not empty.
 *
 * If all criteria are met, the content of the `.move` file is returned. Otherwise, an appropriate
 * error is returned.
 *
 * # Parameters
 * - `path`: A string slice representing the directory path to verify.
 *
 * # Returns
 * - `Ok(String)`: The content of the `.move` file if all conditions are satisfied.
 * - `Err(TokenGenErrors)`: If the path or file structure is invalid.
 *
 * # Errors
 * - `TokenGenErrors::InvalidPathNotDirectory`: If the path or `sources` folder is missing or not a directory.
 * - `TokenGenErrors::InvalidPathNoMoveFiles`: If no valid `.move` file is found in the `sources` folder.
 */
pub fn verify_path(path: &str) -> Result<String> {
    let path = Path::new(path);

    // Construct the path to the `sources` folder.
    let sources_folder = path.join(SUB_FOLDER);

    // Validate that the provided path and `sources` folder exist and are directories.
    if !path.exists() || !path.is_dir() || !sources_folder.exists() || !sources_folder.is_dir() {
        return Err(TokenGenErrors::InvalidPathNotDirectory);
    }

    // Read entries from the `sources` folder.
    let entries = read_dir(&sources_folder)?;

    // Look for the first `.move` file in the `sources` folder.
    let mut current_content = String::new();
    for entry in entries {
        let entry = entry?;
        let path = entry.path();

        // Check if the file has a `.move` extension.
        if path.is_file() && path.extension().is_some_and(|e| e == "move") {
            // Read the content of the `.move` file.
            current_content = read_file(&path)?;
            break; // Stop searching after the first valid `.move` file is found.
        }
    }

    // Return an error if no `.move` file was found or the file is empty.
    if current_content.is_empty() {
        return Err(TokenGenErrors::InvalidPathNoMoveFiles);
    }

    // Return the content of the `.move` file.
    Ok(current_content)
}

/**
 * Reads the contents of a directory and returns its entries.
 *
 * This function provides an abstraction for reading a directory, returning a `ReadDir` iterator
 * for the directory's entries.
 *
 * # Parameters
 * - `dir`: A reference to the path of the directory to read.
 *
 * # Returns
 * - `Ok(ReadDir)`: An iterator over the directory's entries.
 * - `Err(io::Error)`: If the directory cannot be read (e.g., it doesn't exist or lacks permissions).
 */
pub fn read_dir(dir: &Path) -> io::Result<ReadDir> {
    fs::read_dir(dir)
}
