use std::{fs, path};

use crate::{
    utils::helpers::sanitize_name,
    variables::{SUB_FOLDER, TEST_FOLDER},
    Result,
};

pub fn create_contract_file(
    name: &str,
    base_folder: &str,
    token_template: &str,
    sub_folder: &str,
) -> Result<()> {
    // Filtering alphanumeric characters only
    let slug: String = sanitize_name(&name.to_string());

    // Create move contract file in base_folder/sources folder
    let sources_folder: String = format!("{}/{}", base_folder, sub_folder);
    let file_name: String = format!("{}/{}.move", sources_folder, slug.to_lowercase());

    fs::write(&file_name, token_template)?;
    Ok(())
}

// Creating contract base folder and sources folder
pub fn create_base_folder(base_folder: &str) -> Result<()> {
    create_dir(base_folder, SUB_FOLDER)?;
    create_dir(base_folder, TEST_FOLDER)?;
    Ok(())
}

// Creating move.toml file from RPC response
pub fn create_move_toml(package_name: &str, toml_content: &str) -> Result<()> {
    let file_path: String = format!("{}/Move.toml", package_name);
    fs::write(&file_path, toml_content)?;
    Ok(())
}

pub fn create_dir(base_folder: &str, sub_folder: &str) -> Result<()> {
    let dir: String = format!("{}/{}", base_folder, sub_folder);
    fs::create_dir_all(&dir)?;
    Ok(())
}

pub fn remove_dir(base_folder: &str) -> Result<()> {
    if path::Path::new(&base_folder).exists() {
        fs::remove_dir_all(&base_folder)?;
    }
    Ok(())
}
