use crate::utils::generation::{create_generate_token, generate_move_toml};
use crate::utils::prompts::get_user_prompt;
use crate::utils::helpers::create_base_folder;


pub async fn create_token() {
    //Prompt helper
    let token_data: Result<(u8, String, String, String, bool), String> = get_user_prompt();
    println!("Creating contract...");

    if let Ok((decimals, symbol, name, description, is_frozen)) = token_data {
        // Creating base folder
        create_base_folder(&name);

        //Generating toml file
        generate_move_toml(&name);

        //Generating token with user prompt
        create_generate_token(decimals, symbol, &name, description, is_frozen, &name);
        println!("Contract has been generated!");
    } else {
        eprintln!("Failed to create contract: {:?}", token_data.err());
    }
}
