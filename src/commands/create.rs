use std::io;
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    success_handler::{handle_success, SuccessType},
    utils::{
        atomic::AtomicFileOperation,
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::sanitize_name,
        prompts::{get_user_prompt, TokenInfo},
    },
    variables::{SUB_FOLDER, TEST_FOLDER},
    Result,
};
// use dirs::desktop_dir;

/// Implements conversion from `TokenGenErrors` to `io::Error`.
/// This allows `TokenGenErrors` to be treated as `io::Error` for easier interoperability with standard IO functions.
impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::new(io::ErrorKind::Other, err.to_string())
    }
}

/// Creates a new Sui token contract by interacting with the RPC server and managing local file operations.
///
/// This function follows these steps:
/// 1. Collect token data via user input.
/// 2. Send the token data to the RPC server to generate contract content.
/// 3. Write the contract files to disk using an atomic operation to ensure consistency.
/// 4. Handle success and rollback in case of errors.
///
/// # Arguments
/// * `client` - An instance of `TokenGenClient` to communicate with the RPC server.
///
/// # Returns
/// * `Ok(())` if the token contract is successfully created and saved.
/// * `Err(TokenGenErrors)` if any step in the process fails.
pub async fn create_token(client: TokenGenClient) -> Result<()> {
    // Collect token information from the user via a prompt.
    let token_data: TokenInfo = get_user_prompt()?;
    println!("Sending request to RPC service...");

    // Call the RPC server's `create` method to generate the contract's content.
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
        // Map RPC errors to custom `TokenGenErrors`.
        .map_err(TokenGenErrors::RpcError)?
        // Map token creation failure to `TokenGenErrors`.
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?;

    // Sanitize the token name and convert it to lowercase to generate a base folder name.
    let project_folder: String = sanitize_name(&token_data.name).to_lowercase();

    // Get the current working directory
    let current_dir = std::env::current_dir().map_err(|_| TokenGenErrors::CurrentDirectoryError)?;
    let base_folder_path = current_dir.join(&project_folder);

    // Convert to &str
    let base_folder = base_folder_path
        .to_str()
        .ok_or(TokenGenErrors::PathConversionError)?;

    // Initialize atomic file operation to manage all file writes with rollback support.
    let mut atomic_op = AtomicFileOperation::new(base_folder);

    // Create the base folder for the token contract files.
    create_base_folder(base_folder)?;

    // Generate and write the Move.toml file to the base folder.
    create_move_toml(base_folder, &move_toml)?;

    // Generate and write the main contract file to the appropriate subfolder.
    create_contract_file(&token_data.name, base_folder, &token_content, SUB_FOLDER)?;

    // Generate and write the test contract file to the test folder.
    create_contract_file(
        &token_data.name,
        base_folder,
        &test_token_content,
        TEST_FOLDER,
    )?;

    // Commit the atomic file operation. Rollback will occur automatically if any step fails.
    atomic_op.commit();

    // Notify the user and display the contract path
    println!(
        "Token contract created successfully at: {}",
        base_folder_path.display()
    );

    // Handle the success case by notifying the user and logging the operation.
    handle_success(SuccessType::TokenCreated(token_data));
    Ok(())
}
