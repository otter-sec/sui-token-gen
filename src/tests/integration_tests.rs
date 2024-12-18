use std::{fs, path::Path};
use tarpc::context;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::sanitize_name,
    },
    variables::SUB_FOLDER,
    Result,
};

use super::command_tests::setup_test_client;

#[tokio::test]
async fn test_full_token_creation_flow() -> Result<()> {
    // Test data
    let test_folder = "integration_test_token";
    let token_name = "IntegrationToken";
    let token_symbol = "INT";
    let token_description = "Integration test token";
    let decimals = 6;
    let is_frozen = false;
    let environment = "devnet".to_string();

    // Initialize client
    let client: TokenGenClient = setup_test_client("[::1]:5000").await?;

    // Test successful creation
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
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?;

    // Create base structure
    create_base_folder(test_folder)?;

    // Test successful full creation
    create_base_folder(test_folder)?;
    create_move_toml(test_folder, &move_toml)?;
    create_contract_file(token_name, test_folder, &token_content, SUB_FOLDER)?;

    // Verify created files
    let sources_path = format!("{}/{}", test_folder, SUB_FOLDER);
    let contract_path = format!("{}/{}.move", sources_path, sanitize_name(token_name).to_lowercase());
    let toml_path = format!("{}/Move.toml", test_folder);

    assert!(
        Path::new(&sources_path).exists(),
        "Sources folder not created"
    );
    assert!(
        Path::new(&contract_path).exists(),
        "Contract file not created"
    );
    assert!(Path::new(&toml_path).exists(), "Move.toml not created");

    // Clean up
    if Path::new(test_folder).exists() {
        fs::remove_dir_all(test_folder)?;
    }

    Ok(())
}

#[tokio::test]
async fn test_error_handling_integration() -> Result<()> {
    let client: TokenGenClient = setup_test_client("[::1]:5000").await?;

    // Test error handling in full workflow
    let result = client
        .create(
            context::current(),
            255,            // Invalid decimals
            "".to_string(), // Empty name
            "TEST".to_string(),
            "Description".to_string(),
            false,
            "invalid_env".to_string(),
        )
        .await?;
    assert!(result.is_err());
    Ok(())
}
