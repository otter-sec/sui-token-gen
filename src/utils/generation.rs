use crate::{
    errors::TokenGenErrors,
    utils::helpers::sanitize_name,
    variables::SUB_FOLDER,
    Result,
};
use std::fs;
pub fn create_contract_file(
    name: String,
    base_folder: String,
    token_template: String
) -> Result<()> {
    //Filtering alphanumeric characters only
    let slug: String = sanitize_name(name.to_owned());

    //Create move contract file in base_folder/sources folder
    let sources_folder: String = format!("{}/{}", base_folder, SUB_FOLDER);
    let file_name: String = format!("{}/{}.move", sources_folder, slug.to_lowercase());

    if let Err(e) = fs::write(&file_name, token_template) {
        return Err(TokenGenErrors::FileIoError(e));
    }
    Ok(())
}

// Creating contract base folder and sources folder
pub fn create_base_folder(base_folder: String) -> Result<()> {
    let sources_folder: String = format!("{}/{}", base_folder, SUB_FOLDER);
    if let Err(e) = fs::create_dir_all(&sources_folder) {
        return Err(TokenGenErrors::FileIoError(e));
    }
    Ok(())
}

//Creating move.toml file from RPC response
pub fn create_move_toml(package_name: String, toml_content: String) -> Result<()> {
    let file_path: String = format!("{}/Move.toml", package_name);
    if let Err(e) = fs::write(&file_path, toml_content) {
        return Err(TokenGenErrors::FileIoError(e));
    }
    Ok(())
}