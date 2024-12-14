use std::io;
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::{log_success_message, sanitize_name},
        prompts::get_user_prompt,
    },
    variables::{SUB_FOLDER, TEST_FOLDER},
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

    // Calling RPC create function
    let (token_content, move_toml, test_token_content) = client
        .create(
            context::current(),
            token_data.decimals,
            token_data.name.to_owned(),
            token_data.symbol,
            token_data.description,
            token_data.is_frozen,
            token_data.environment,
        )
        .await
        .map_err(|e| {
            let error = TokenGenErrors::RpcError(e);
            error
        })?
        .map_err(|e| {
            let error = TokenGenErrors::FailedToCreateTokenContract(e.to_string());
            error
        })?;

    let base_folder: String = sanitize_name(&token_data.name);

    // Creating base contract folder
    create_base_folder(&base_folder)?;

    // Creating .toml and contract files
    create_move_toml(base_folder.to_owned(), move_toml)?;

    // Creating contract file
    create_contract_file(
        token_data.name.to_owned(),
        base_folder.to_owned(),
        token_content,
        SUB_FOLDER,
    )?;

    // Creating tests file
    create_contract_file(
        token_data.name,
        base_folder,
        test_token_content,
        TEST_FOLDER,
    )?;

    log_success_message("Contract has been generated!");
    Ok(())
}
