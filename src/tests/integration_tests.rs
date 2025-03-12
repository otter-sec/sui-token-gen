use std::{fs, path::Path};
use tarpc::context;

use crate::{
    constants::{ADDRESS, DEFAULT_ENVIRONMENT, SUB_FOLDER},
    errors::TokenGenErrors,
    utils::{
        client::rpc_client::TokenGenClient, generation::ContractGenerator, helpers::sanitize_name,
    },
    Result,
};

use super::common::test_utils::setup_test_client;

// Test case for the full token creation flow, including RPC client interaction and file generation
#[tokio::test]
async fn test_full_token_creation_flow() -> Result<()> {
    // Test data: Token attributes for creation
    let test_folder = "integration_test_token"; // Folder name for token contract files
    let token_name = "IntegrationToken"; // Token name
    let token_symbol = "INT"; // Token symbol
    let token_description = "Integration test token"; // Token description
    let decimals = 6; // Number of decimal places for the token
    let is_frozen = false; // Whether the token is frozen or not
    let environment = DEFAULT_ENVIRONMENT.to_string(); // Environment for token deployment

    // Initialize the RPC client using the local address for testing
    let client: TokenGenClient = setup_test_client(ADDRESS).await?;

    // Test the successful creation of the token contract using the provided data
    let (token_content, move_toml, _test_content) = client
        .create(
            context::current(),
            decimals,
            token_name.to_string(),
            token_symbol.to_string(),
            token_description.to_string(),
            is_frozen,
            environment,
        )
        .await
        .map_err(TokenGenErrors::RpcError)? // Map RPC error to a custom error type
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?; // Map contract creation failure

    let contract_generator = ContractGenerator::new(test_folder.to_string());
    // Create the base folder to store generated files
    contract_generator.create_base_folder()?;

    // Generate the Move.toml file and token contract file for the token
    contract_generator.create_move_toml(&move_toml)?;
    contract_generator.create_contract_file(token_name, &token_content, SUB_FOLDER)?;

    // Verify the existence of the necessary files after creation
    let sources_path = format!("{}/{}", test_folder, SUB_FOLDER); // Path to sources folder
    let contract_path = format!(
        "{}/{}.move", // Path to the generated Move contract file
        sources_path,
        sanitize_name(token_name).to_lowercase()
    );
    let toml_path = format!("{}/Move.toml", test_folder); // Path to the generated Move.toml file

    // Assertions to ensure that all necessary files have been created
    assert!(
        Path::new(&sources_path).exists(),
        "Sources folder not created"
    );
    assert!(
        Path::new(&contract_path).exists(),
        "Contract file not created"
    );
    assert!(Path::new(&toml_path).exists(), "Move.toml not created");

    // Clean up: Remove the test folder and its contents after the test
    if Path::new(test_folder).exists() {
        fs::remove_dir_all(test_folder)?; // Delete the folder if it exists
    }

    Ok(())
}

// Test case to ensure error handling works properly during token creation
#[tokio::test]
async fn test_error_handling_integration() -> Result<()> {
    // Initialize the RPC client
    let client: TokenGenClient = setup_test_client(ADDRESS).await?;
    // Test invalid token creation by providing incorrect parameters
    let result = client
        .create(
            context::current(),
            255,            // Invalid decimal places (too high)
            "".to_string(), // Empty token name (invalid)
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "invalid_env".to_string(), // Invalid environment
        )
        .await?;
    // Assert that the result is an error due to invalid parameters
    assert!(result.is_err(), "Expected error during token creation");

    Ok(())
}
