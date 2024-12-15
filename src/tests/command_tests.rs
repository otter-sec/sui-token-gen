use std::fs;
use std::path::Path;

use crate::{
    errors::TokenGenErrors,
    rpc_client::{create_timeout_context, initiate_client, TokenGenClient},
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml},
        helpers::sanitize_name,
    },
    variables::SUB_FOLDER,
};

/// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client(address: &str) -> crate::Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}

async fn test_initiate_client() -> crate::Result<TokenGenClient> {
    setup_test_client("127.0.0.1:5000").await
}

#[tokio::test]
async fn create_command() -> crate::Result<()> {
    // Test user inputs
    let decimals: u8 = 6;
    let symbol: String = "SAMPLE".to_string();
    let name: &str = "SampleToken";
    let description: String = "This is a sample token for testing.".to_string();
    let is_frozen: bool = false;
    let environment: String = "devnet".to_string();

    // Testing contract folder
    let base_folder = sanitize_name(&name.to_string());

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // If the test base folder already exists, delete it
    if Path::new(&base_folder).exists() {
        fs::remove_dir_all(&base_folder).expect("Failed to delete test base folder");
    }

    // Call the `create` method
    let (token_content, move_toml, _test_token_content) = client
        .create(
            create_timeout_context(),
            decimals,
            name.to_owned(),
            symbol.to_owned(),
            description.to_owned(),
            is_frozen,
            environment,
        )
        .await
        .map_err(|e| TokenGenErrors::RpcError(e))?
        .map_err(|e| TokenGenErrors::FailedToCreateTokenContract(e.to_string()))?;

    // Create base folder
    create_base_folder(&base_folder)?;

    // Generate Move.toml file
    create_move_toml(&base_folder, &move_toml).expect("Failed to create Move.toml");

    // Generate token contract file
    create_contract_file(
        &name,
        &base_folder,
        &token_content,
        SUB_FOLDER,
    )
    .expect("Failed to create contract file");

    // Validate folder and file creation
    let sources_folder = format!("{}/{}", base_folder, SUB_FOLDER);
    let toml_file: String = format!("{}/Move.toml", base_folder);
    let move_file: String = format!(
        "{}/{}.move",
        sources_folder,
        sanitize_name(&name.to_string())
    );

    assert!(
        Path::new(&sources_folder).exists(),
        "Sources folder not created"
    );
    assert!(Path::new(&toml_file).exists(), "Move.toml file not created");
    assert!(
        Path::new(&move_file).exists(),
        "Move contract file not created"
    );

    // Validate Move.toml file content
    let toml_content = fs::read_to_string(&toml_file).expect("Failed to read Move.toml file");
    assert!(
        toml_content.contains("0.0.1"),
        "Move.toml file does not contain the correct version"
    );
    assert!(
        toml_content.contains(&base_folder),
        "Move.toml file does not contain the correct package name"
    );

    // Validate Move contract file content
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

    // Clean up: Delete the test base folder
    fs::remove_dir_all(base_folder).expect("Failed to delete test base folder");

    Ok(())
}

#[tokio::test]
async fn verify_command_valid_file() -> crate::Result<()> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let templates_path = format!("{}/src/test_tokens/valid_token.move", current_dir.display());

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Read content from the existing valid token file
    let valid_content =
        fs::read_to_string(templates_path).expect("Failed to read valid token file");

    let response = client
        .verify_content(create_timeout_context(), valid_content)
        .await;
    assert!(response.is_ok(), "Verification failed");
    Ok(())
}

#[tokio::test]
async fn verify_command_invalid_file() -> crate::Result<()> {
    let current_dir = std::env::current_dir().expect("Failed to get current directory");
    let templates_path = format!(
        "{}/src/test_tokens/invalid_token.move",
        current_dir.display()
    );

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Read content from the existing valid token file
    let valid_content =
        fs::read_to_string(templates_path).expect("Failed to read valid token file");

    let response = client
        .verify_content(create_timeout_context(), valid_content)
        .await
        .map_err(|e| TokenGenErrors::RpcError(e))?;
    assert!(response.is_err(), "Verification failed");

    Ok(())
}

#[tokio::test]
async fn verify_command_valid_git() -> crate::Result<()> {
    // Testing repo
    let valid_url = "https://github.com/meumar-osec/test-sui-token";

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Call verify_token
    let response = verify_token_using_url(valid_url, client).await;
    assert!(response.is_ok(), "Failed to verify URL");
    Ok(())
}

#[tokio::test]
async fn verify_command_invalid_git() -> crate::Result<()> {
    let valid_url = "https://github.com/meumar-osec/sui-token1";

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Call verify_token
    let response = verify_token_using_url(valid_url, client).await;
    assert!(response.is_err(), "Failed to verify URL");
    Ok(())
}
