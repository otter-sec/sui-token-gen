use tarpc::context;

use crate::{
    constants::{SUB_FOLDER, TEST_FOLDER},
    errors::TokenGenErrors,
    handlers::{handle_success, SuccessType},
    utils::{
        atomic::AtomicFileOperation,
        client::rpc_client::TokenGenClient,
        generation::ContractGenerator,
        helpers::sanitize_name,
        prompts::{get_user_prompt, TokenInfo},
    },
    CreateTokenParams, Result,
};

/// Creates a new Sui token contract by interacting with the RPC server and managing local file operations.
///
/// This function follows these steps:
/// 1. Collects token configuration data from the user via interactive prompts.
/// 2. Sends the collected data to the RPC server, which generates the contract's source code.
/// 3. Creates a project folder and writes the generated contract files to disk using an atomic operation for reliability.
/// 4. Ensures proper error handling, including rollback in case of failures.
///
/// # Arguments
/// * `client` - An instance of `TokenGenClient` that communicates with the RPC server.
/// * `params` - A reference to `CreateTokenParams` containing optional predefined token parameters.
///
/// # Returns
/// * `Ok(())` - If the token contract is successfully generated and saved.
/// * `Err(TokenGenErrors)` - If any step in the process fails, returning a specific error variant.
pub async fn create_token(client: TokenGenClient, params: &CreateTokenParams) -> Result<()> {
    // Step 1: Collect token details from user input (or use predefined parameters).
    let token_data: TokenInfo = get_user_prompt(params)?;
    println!("Sending request to RPC service...");

    // Step 2: Request contract generation from the RPC server.
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
        .map_err(TokenGenErrors::RpcError)? // Convert RPC-related errors to `TokenGenErrors`.
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?; // Handle failed contract generation.

    // Step 3: Prepare project folder paths.
    let project_folder: String = sanitize_name(&token_data.name).to_lowercase();
    let current_dir = std::env::current_dir().map_err(|_| TokenGenErrors::CurrentDirectoryError)?;
    let base_folder_path = current_dir.join(&project_folder);

    // Ensure the folder path is valid and convertible to a string.
    let base_folder = base_folder_path
        .to_str()
        .ok_or(TokenGenErrors::PathConversionError)?;

    // Step 4: Initialize an atomic file operation to prevent partial writes in case of failure.
    let mut atomic_op = AtomicFileOperation::new(base_folder);

    // Create contract generator instance.
    let contract_generator = ContractGenerator::new(base_folder.to_string());

    // Step 5: Create and populate contract files.
    contract_generator.create_base_folder()?; // Ensure the base folder exists.
    contract_generator.create_move_toml(&move_toml)?; // Write Move.toml configuration.
    contract_generator.create_contract_file(&token_data.name, &token_content, SUB_FOLDER)?; // Write main contract.
    contract_generator.create_contract_file(&token_data.name, &test_token_content, TEST_FOLDER)?; // Write test contract.

    // Step 6: Finalize by committing the atomic operation.
    atomic_op.commit();

    // Step 7: Notify the user about the successful contract creation.
    handle_success(SuccessType::TokenCreated(
        token_data,
        format!(
            "Contract has been generated at: {}",
            base_folder_path.display()
        ),
    ));

    Ok(())
}
