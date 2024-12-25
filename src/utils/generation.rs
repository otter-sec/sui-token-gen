use crate::{
    utils::helpers::sanitize_name,
    variables::{SUB_FOLDER, TEST_FOLDER},
    Result,
};
use std::fs;

/**
 * Creates a contract file within a specific subfolder of the base folder.
 *
 * This function performs the following:
 * 1. Sanitizes the provided contract name to ensure it contains only alphanumeric characters.
 * 2. Constructs the path to the target file location (base folder + subfolder).
 * 3. Writes the provided token template content into the target file.
 *
 * # Parameters
 * - `name`: The name of the token, used to generate a sanitized file name.
 * - `base_folder`: The root folder where the contract file will be created.
 * - `token_template`: The content of the token contract to be written to the file.
 * - `sub_folder`: The subfolder inside the base folder where the file will be created.
 *
 * # Returns
 * - `Ok(())` if the file is created successfully.
 * - `Err` if there is an issue with file creation.
 */
pub fn create_contract_file(
    name: &str,
    base_folder: &str,
    token_template: &str,
    sub_folder: &str,
) -> Result<()> {
    // Sanitize the name to create a safe file name (alphanumeric only).
    let slug: String = sanitize_name(name);

    // Construct the path for the contract file.
    let sources_folder: String = format!("{}/{}", base_folder, sub_folder);
    let file_name: String = format!("{}/{}.move", sources_folder, slug.to_lowercase());

    // Write the token template content to the file.
    fs::write(&file_name, token_template)?;
    Ok(())
}

/**
 * Creates the base folder for the contract and its subfolders.
 *
 * This function sets up the required directory structure for the token contract:
 * - A main folder for the contract (base folder).
 * - Subfolders such as `sources` and `tests` for organizing contract files.
 *
 * # Parameters
 * - `base_folder`: The root folder where subfolders will be created.
 *
 * # Returns
 * - `Ok(())` if all directories are created successfully.
 * - `Err` if any directory creation fails.
 */
pub fn create_base_folder(base_folder: &str) -> Result<()> {
    // Create the main `sources` and `tests` subdirectories.
    create_dir(base_folder, SUB_FOLDER)?;
    create_dir(base_folder, TEST_FOLDER)?;
    Ok(())
}

/**
 * Creates a `Move.toml` file from the RPC response.
 *
 * This function generates a `Move.toml` file, which is essential for defining the Move package structure.
 *
 * # Parameters
 * - `package_name`: The name of the package, used to determine the output file location.
 * - `toml_content`: The content of the `Move.toml` file to be written.
 *
 * # Returns
 * - `Ok(())` if the file is created successfully.
 * - `Err` if there is an issue with file creation.
 */
pub fn create_move_toml(package_name: &str, toml_content: &str) -> Result<()> {
    // Construct the file path for `Move.toml`.
    let file_path: String = format!("{}/Move.toml", package_name);

    // Write the provided TOML content to the file.
    fs::write(&file_path, toml_content)?;
    Ok(())
}

/**
 * Creates a directory within the base folder.
 *
 * This utility function constructs the full directory path and creates it.
 * It ensures the directory structure is created recursively if it does not already exist.
 *
 * # Parameters
 * - `base_folder`: The root folder where the directory will be created.
 * - `sub_folder`: The subfolder to be created within the base folder.
 *
 * # Returns
 * - `Ok(())` if the directory is created successfully.
 * - `Err` if there is an issue with directory creation.
 */
pub fn create_dir(base_folder: &str, sub_folder: &str) -> Result<()> {
    // Construct the directory path.
    let dir: String = format!("{}/{}", base_folder, sub_folder);

    // Create the directory and all necessary parent directories.
    fs::create_dir_all(&dir)?;
    Ok(())
}
