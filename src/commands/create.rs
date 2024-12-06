use std::io;
use anyhow::Result;

use crate::{
    utils::{
        generation::{create_generate_token, generate_move_toml},
        helpers::{create_base_folder, sanitize_name},
        prompts::get_user_prompt
    },
    errors::TokenGenErrors
};

impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

pub async fn create_token() -> io::Result<()> {
    // Prompt helper
    let token_data: Result<(u8, String, String, String, bool), String> = get_user_prompt();
    println!("Creating contract...");

    if let Ok((decimals, symbol, name, description, is_frozen)) = token_data {
        let base_folder = sanitize_name(name.to_owned());

        // Creating base folder
        create_base_folder(&base_folder);

        // Generating toml file
        generate_move_toml(&base_folder);

        // Generating token with user prompt
        create_generate_token(decimals, symbol, &name, description, is_frozen, &base_folder);
        println!("Contract has been generated!");
    } else {
        let error_message = token_data.err().unwrap_or_default();
        return Err(TokenGenErrors::FailedToCreateTokenContract(error_message).into());
    }
    Ok(())
}
