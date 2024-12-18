use std::{env, fs, path::Path};
use tarpc::context;

use crate::{
    commands::verify::verify_token_using_url,
    errors::TokenGenErrors,
    rpc_client::{initiate_client, TokenGenClient},
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::sanitize_name,
    },
    variables::{ADDRESS, SUB_FOLDER},
    Result,
};

// Helper function to set up a test client with consistent error handling
// This function initializes an RPC client by calling the initiate_client function and returns the client.
// If an error occurs during client initialization, it maps the error to a TokenGenError.
pub async fn setup_test_client(address: &str) -> Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}

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
    let environment: String = "devnet".to_string();

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
    create_base_folder(&base_folder)?;

    // Generate the Move.toml file for the contract
    create_move_toml(&base_folder, &move_toml).expect("Failed to create Move.toml");

    // Generate the actual contract file for the token
    create_contract_file(name, &base_folder, &token_content, SUB_FOLDER)
        .expect("Failed to create contract file");

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
    let templates_path = format!("{}/src/test_tokens/valid_token.move", current_dir.display());

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
        "{}/src/test_tokens/invalid_token.move",
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
