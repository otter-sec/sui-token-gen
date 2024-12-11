use std::io;
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::sanitize_name,
        prompts::get_user_prompt,
    },
    Result,
};

impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

pub async fn create_token(client: TokenGenClient) -> Result<()> {
    let token_data = get_user_prompt()?;
    println!("Creating contract...");

    let (token_content, move_toml) = client
        .create(
            context::current(),
            token_data.decimals,
            token_data.name.to_owned(),
            token_data.symbol,
            token_data.description,
            token_data.is_frozen,
        )
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(TokenGenErrors::FailedToCreateTokenContract)?;

    let base_folder = sanitize_name(token_data.name.to_owned());
    create_base_folder(base_folder.to_owned())?;
    create_move_toml(base_folder.to_owned(), move_toml)?;
    create_contract_file(token_data.name, base_folder, token_content)?;

    println!("Contract has been generated!");
    Ok(())
}
