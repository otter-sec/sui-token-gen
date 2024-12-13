use crate::{
    utils::helpers::sanitize_name,
    variables::{SUB_FOLDER, TEST_FOLDER},
    Result,
};
use std::fs;
pub fn create_contract_file(
    name: String,
    base_folder: String,
    token_template: String,
    sub_folder: &str,
) -> Result<()> {
    // Filtering alphanumeric characters only
    let slug: String = sanitize_name(&name);

    // Create move contract file in base_folder/sources folder
    let sources_folder: String = format!("{}/{}", base_folder, sub_folder);
    let file_name: String = format!("{}/{}.move", sources_folder, slug.to_lowercase());

    fs::write(&file_name, token_template)?;
    Ok(())
}

// Creating contract base folder and sources folder
pub fn create_base_folder(base_folder: &String) -> Result<()> {
    create_dir(&base_folder, SUB_FOLDER)?;
    create_dir(&base_folder, TEST_FOLDER)?;
    Ok(())
}

// Creating move.toml file from RPC response
pub fn create_move_toml(package_name: String, toml_content: String) -> Result<()> {
    let file_path: String = format!("{}/Move.toml", package_name);
    fs::write(&file_path, toml_content)?;
    Ok(())
}

pub fn create_dir(base_folder: &String, sub_folder: &str) -> Result<()> {
    let dir: String = format!("{}/{}", &base_folder, sub_folder);
    fs::create_dir_all(&dir)?;
    Ok(())
}
