use std::io;

use crate::{
    errors::TokenGenErrors,
    utils::{
        generation::{create_generate_token, generate_move_toml},
        helpers::{create_base_folder, sanitize_name},
        prompts::get_user_prompt,
    },
    Result,
};

impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

pub async fn create_token() -> Result<()> {
    // Prompt helper
    let token_data = get_user_prompt();
    println!("Creating contract...");

    match token_data {
        Ok(token_data) => {
            let base_folder = sanitize_name(token_data.name.to_owned());

            // Creating base folder
            create_base_folder(&base_folder)?;

            // Generating toml file
            generate_move_toml(&base_folder)?;

            // Generating token with user prompt
            create_generate_token(
                token_data.decimals,
                token_data.symbol,
                &token_data.name,
                token_data.description,
                token_data.is_frozen,
                &base_folder,
            )?;
            println!("Contract has been generated!");
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}
