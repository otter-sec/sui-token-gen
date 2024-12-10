use std::io;
use tarpc::context;

use crate::{
        errors::TokenGenErrors,
        rpc_client::TokenGenClient, 
        utils::{
            generation::{create_contract_file, create_base_folder, create_move_toml},
            helpers::sanitize_name,
            prompts::get_user_prompt,
        }, Result
};

impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

pub async fn create_token(client: TokenGenClient) -> Result<()> {
    // Prompt helper
    let token_data = get_user_prompt();
    println!("Creating contract...");

    match token_data {
        Ok(token_data) => {
            // Call the `create` method and handle the nested Result
            match client
                .create(
                    context::current(),
                    token_data.decimals,
                    token_data.name.to_owned(),
                    token_data.symbol,
                    token_data.description,
                    token_data.is_frozen,
                )
                .await
            {
                Ok(Ok((token_content, move_toml))) => {
                    println!("Token Content:\n{}", token_content);
                    println!("Move.toml Content:\n{}", move_toml);

                    let base_folder = sanitize_name(token_data.name.to_owned());

                    // Creating base folder
                    create_base_folder(base_folder.to_owned())?;

                    // Generating toml file
                    create_move_toml(base_folder.to_owned(), move_toml)?;

                    // Generating token with user prompt
                    create_contract_file(token_data.name, base_folder, token_content)?;
                }
                Ok(Err(err)) => {
                    eprintln!("TokenGen Error: {:?}", err);
                }
                Err(rpc_err) => {
                    eprintln!("RPC Error: {:?}", rpc_err);
                }
            }

            println!("Contract has been generated!");
        }
        Err(e) => {
            return Err(e);
        }
    }
    Ok(())
}

