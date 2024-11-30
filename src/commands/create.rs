use crate::utils::generation::{generate_token, generate_move_toml};
use crate::utils::prompts::get_user_prompt;
use crate::utils::helpers::create_base_folder;


pub async fn create_token() {
    let token_data: Result<(u8, String, String, String, bool), String> = get_user_prompt();
    println!("Creating token...");
    const BASE_FOLDER: &str = "tokengen";

    if let Ok((decimals, symbol, name, description, is_frozen)) = token_data {
        create_base_folder(BASE_FOLDER);
        generate_move_toml(BASE_FOLDER);
        generate_token(decimals, symbol, name, description, is_frozen, BASE_FOLDER);
        println!("Contract has been generated!");
    } else {
        eprintln!("Failed to create token: {:?}", token_data.err());
    }
}
