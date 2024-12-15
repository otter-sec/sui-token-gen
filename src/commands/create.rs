use std::io;
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    success_handler::{handle_success, SuccessType},
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::sanitize_name,
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

    // Calling RPC create function with owned Strings
    let (token_content, move_toml, test_token_content) = client
        .create(
            context::current(),
            token_data.decimals,
            token_data.name.clone(),
            token_data.symbol.clone(),
            token_data.description.clone(),
            token_data.is_frozen,
            token_data.environment.clone(),
        )
        .await
        .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?;

    let base_folder: String = sanitize_name(&token_data.name);

    // Creating base contract folder
    create_base_folder(&base_folder)?;

    // Creating .toml and contract files
    create_move_toml(&base_folder, &move_toml)?;

    // Creating contract file
    create_contract_file(
        &token_data.name,
        &base_folder,
        &token_content,
        SUB_FOLDER,
    )?;

    // Creating tests file
    create_contract_file(
        &token_data.name,
        &base_folder,
        &test_token_content,
        TEST_FOLDER,
    )?;

    handle_success(SuccessType::TokenCreated(token_data));
    Ok(())
}
