use std::{env, fs, path::Path};
use tarpc::context;

use crate::{
    commands::verify::{verify_token_address, verify_token_using_url},
    constants::{ADDRESS, DEFAULT_ENVIRONMENT, SUB_FOLDER},
    errors::TokenGenErrors,
    tests::common::setup_test_client,
    utils::{
        client::rpc_client::TokenGenClient, generation::ContractGenerator, helpers::sanitize_name,
    },
    Result,
};

// Test function to initiate the RPC client for testing purposes
// It simply calls the setup_test_client function with the predefined ADDRESS constant.
async fn test_initiate_client() -> Result<TokenGenClient> {
    setup_test_client(ADDRESS).await
}

#[tokio::test]
async fn create_command() -> Result<()> {
    // Test user inputs for creating a token contract
    let decimals: u8 = 6;
    let symbol: String = "SAMPLE".to_string();
    let name: &str = "SampleToken";
    let description: String = "This is a sample token for testing.".to_string();
    let is_frozen: bool = false;
    let environment: String = DEFAULT_ENVIRONMENT.to_string();

    // Sanitizing the token name for folder creation
    let base_folder = sanitize_name(name);

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // If the test base folder already exists, delete it
    if Path::new(&base_folder).exists() {
        fs::remove_dir_all(&base_folder).expect("Failed to delete test base folder");
    }

    // Create token contract by calling the `create` method on the RPC client
    let (token_content, move_toml, _test_token_content) = client
        .create(
            context::current(),
            decimals,
            name.to_owned(),
            symbol.to_owned(),
            description.to_owned(),
            is_frozen,
            environment,
        )
        .await
        .map_err(TokenGenErrors::RpcError)?
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?;

    // Create the base folder for the token contract
    let contract_generator = ContractGenerator::new(base_folder.to_string());

    // Create the base folder for the token contract
    contract_generator.create_base_folder()?;

    // Generate the Move.toml file for the contract
    contract_generator.create_move_toml(&move_toml)?;

    // Generate the actual contract file for the token
    contract_generator.create_contract_file(name, &token_content, SUB_FOLDER)?;

    // Validate folder and file creation
    let sources_folder = format!("{}/{}", base_folder, SUB_FOLDER);
    let toml_file: String = format!("{}/Move.toml", base_folder);
    let move_file: String = format!(
        "{}/{}.move",
        sources_folder,
        sanitize_name(name).to_lowercase()
    );

    // Assertions to ensure the correct files were created
    assert!(
        Path::new(&sources_folder).exists(),
        "Sources folder not created"
    );
    assert!(Path::new(&toml_file).exists(), "Move.toml file not created");
    assert!(
        Path::new(&move_file).exists(),
        "Move contract file not created"
    );

    // Validate the content of Move.toml file
    let toml_content = fs::read_to_string(&toml_file).expect("Failed to read Move.toml file");
    assert!(
        toml_content.contains("0.0.1"),
        "Move.toml file does not contain the correct version"
    );
    assert!(
        toml_content.contains(&base_folder),
        "Move.toml file does not contain the correct package name"
    );

    // Validate the content of the Move contract file
    let move_content = fs::read_to_string(&move_file).expect("Failed to read contract file");
    assert!(
        move_content.contains(&symbol),
        "Contract does not contain the correct symbol"
    );
    assert!(
        move_content.contains(name),
        "Contract does not contain the correct name"
    );
    assert!(
        move_content.contains(&description),
        "Contract does not contain the correct description"
    );

    // Clean up by deleting the test base folder after validation
    fs::remove_dir_all(base_folder).expect("Failed to delete test base folder");

    Ok(())
}

// Test case for verifying a valid token file
#[tokio::test]
async fn verify_command_valid_file() -> Result<()> {
    // Get the current directory path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let templates_path = format!(
        "{}/src/tests/tokens/valid_token.move",
        current_dir.display()
    );

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Read content from the existing valid token file
    let valid_content =
        fs::read_to_string(templates_path).expect("Failed to read valid token file");

    // Verify the content using the RPC client
    let response = client
        .verify_content(context::current(), valid_content)
        .await;
    assert!(response.is_ok(), "Verification failed");
    Ok(())
}

// Test case for verifying an invalid token file
#[tokio::test]
async fn verify_command_invalid_file() -> Result<()> {
    // Get the current directory path
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let templates_path = format!(
        "{}/src/tests/tokens/invalid_token.move",
        current_dir.display()
    );

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Read content from the existing invalid token file
    let invalid_content =
        fs::read_to_string(templates_path).expect("Failed to read invalid token file");

    // Verify the content using the RPC client
    let response = client
        .verify_content(context::current(), invalid_content)
        .await
        .map_err(TokenGenErrors::RpcError)?;
    assert!(response.is_err(), "Verification failed");
    Ok(())
}

// Test case for verifying a valid GitHub URL
#[tokio::test]
async fn verify_command_valid_github() -> Result<()> {
    // Testing repo URL
    let valid_url = "https://github.com/meumar-osec/test-sui-token";

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Call verify_token with the valid GitHub URL
    let response = verify_token_using_url(valid_url, client).await;
    assert!(response.is_ok(), "Failed to verify URL");
    Ok(())
}

// Test case for verifying an invalid GitHub URL
#[tokio::test]
async fn verify_command_invalid_github() -> Result<()> {
    let invalid_url = "https://github.com/meumar-osec/sui-token1";

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Call verify_token with the invalid GitHub URL
    let response = verify_token_using_url(invalid_url, client).await;
    assert!(response.is_err(), "Failed to verify URL");
    Ok(())
}

// Test case for verifying a valid GitLab URL
#[tokio::test]
async fn verify_command_valid_gitlab() -> Result<()> {
    // Testing repo URL
    let valid_url = "https://gitlab.com/osec/test-sui-token";

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Call verify_token with the valid GitLab URL
    let response = verify_token_using_url(valid_url, client).await;
    assert!(response.is_ok(), "Failed to verify URL");
    Ok(())
}

// Test case to verify correct handling of invalid token addresses
#[tokio::test]
async fn verify_token_address_invalid_cases() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Test with an empty address
    let empty_address = "";
    let result = verify_token_address(empty_address, "testnet", client.to_owned()).await;
    assert!(result.is_err()); // Expecting an error due to empty address

    // Test with an invalid address format
    let invalid_address = "invalid_token_address";
    let result = verify_token_address(invalid_address, "testnet", client.to_owned()).await;
    assert!(result.is_err()); // Expecting an error due to incorrect address format

    Ok(())
}

// Test case to verify valid token addresses
#[tokio::test]
async fn verify_token_address_successful_case() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Assume this is a valid token address (mocked in test setup)
    let valid_address = "0xc2f47262639d93701c28453b88df9e6c5feb28925741fcab7b75ffc710805217";
    let result = verify_token_address(valid_address, "testnet", client.to_owned()).await;

    // Expecting the verification to pass for a valid token address
    assert!(result.is_ok());

    Ok(())
}

// Test case to verify handling of different environments
#[tokio::test]
async fn verify_token_address_with_different_environments() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    let valid_address = "0xc2f47262639d93701c28453b88df9e6c5feb28925741fcab7b75ffc710805217";

    // Test with a valid environment
    let result = verify_token_address(valid_address, "testnet", client.to_owned()).await;
    assert!(result.is_ok());

    // Test with an invalid environment (should default to "testnet" internally)
    let result = verify_token_address(valid_address, "invalid_env", client.to_owned()).await;
    assert!(result.is_err()); // It should fail because of "invalid_env"

    Ok(())
}
