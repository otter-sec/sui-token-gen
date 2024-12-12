use std::io;
use tarpc::{client, context, Response};

use crate::{TokenGen, TokenGenErrors, Result};
use crate::utils::prompts::get_user_prompt;

impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

pub async fn create_token(client: &client::NewClient<dyn TokenGen, Response<Result<()>>>) -> Result<()> {
    let token_data = get_user_prompt()?;
    println!("Creating contract...");

    // Calling RPC create function
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
        .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?;

    println!("Contract content: {}", token_content);
    println!("Move.toml content: {}", move_toml);
    println!("Contract has been generated!");
    Ok(())
}
