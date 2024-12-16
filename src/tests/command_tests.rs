use std::{env, fs, path::Path, os::unix::fs::PermissionsExt};
use tarpc::context;
use tokio::time::{sleep, Duration};

use crate::{
    commands::verify::verify_token_using_url,
    errors::TokenGenErrors,
    rpc_client::{initiate_client, TokenGenClient},
    utils::{
        generation::{create_base_folder, create_contract_file, create_move_toml, remove_dir},
        helpers::sanitize_name,
    },
    variables::{ADDRESS, SUB_FOLDER},
    Result,
};

// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client(address: &str) -> Result<TokenGenClient> {
    initiate_client(address)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))
}

async fn test_initiate_client() -> Result<TokenGenClient> {
    setup_test_client(ADDRESS).await
}

#[tokio::test]
async fn create_command() -> Result<()> {
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
            context::current(),
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
    create_contract_file(&name, &base_folder, &token_content, SUB_FOLDER)
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
async fn verify_command_valid_file() -> Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    let templates_path = format!("{}/src/test_tokens/valid_token.move", current_dir.display());

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Read content from the existing valid token file
    let valid_content =
        fs::read_to_string(templates_path).expect("Failed to read valid token file");

    let response = client
        .verify_content(context::current(), valid_content)
        .await;
    assert!(response.is_ok(), "Verification failed");
    Ok(())
}

#[tokio::test]
async fn verify_command_invalid_file() -> Result<()> {
    let current_dir = env::current_dir().expect("Failed to get current directory");
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
        .verify_content(context::current(), valid_content)
        .await
        .map_err(|e| TokenGenErrors::RpcError(e))?;
    assert!(response.is_err(), "Verification failed");

    Ok(())
}

#[tokio::test]
async fn verify_command_valid_git() -> Result<()> {
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
async fn verify_command_invalid_git() -> Result<()> {
    let valid_url = "https://github.com/meumar-osec/sui-token1";

    // Initialize the RPC client
    let client: TokenGenClient = test_initiate_client().await?;

    // Call verify_token
    let response = verify_token_using_url(valid_url, client).await;
    assert!(response.is_err(), "Failed to verify URL");
    Ok(())
}

#[tokio::test]
async fn test_create_token_concurrent() -> Result<()> {
    let test_folder = "test_concurrent_token";
    let lock_path = format!("{}.lock", test_folder);

    // Create lock file to simulate concurrent access
    fs::write(&lock_path, "")?;

    // Attempt to create folder while lock exists
    let result = create_base_folder(test_folder);

    // Clean up
    fs::remove_file(&lock_path)?;
    if Path::new(test_folder).exists() {
        fs::remove_dir_all(test_folder)?;
    }

    assert!(matches!(result, Err(TokenGenErrors::ConcurrentAccess)));
    Ok(())
}

#[tokio::test]
async fn test_create_token_permissions() -> Result<()> {
    let test_folder = "test_permission_token";

    // Create folder with readonly permissions
    fs::create_dir(test_folder)?;
    let metadata = fs::metadata(test_folder)?;
    let mut perms = metadata.permissions();
    perms.set_mode(0o444); // readonly
    fs::set_permissions(test_folder, perms)?;

    // Attempt to create files in readonly folder
    let result = create_move_toml(test_folder, "test content");

    // Clean up: restore permissions to delete
    let metadata = fs::metadata(test_folder)?;
    let mut perms = metadata.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(test_folder, perms)?;
    fs::remove_dir_all(test_folder)?;

    assert!(matches!(result, Err(TokenGenErrors::PermissionDenied)));
    Ok(())
}

#[tokio::test]
async fn test_create_token_rollback() -> Result<()> {
    let test_folder = "test_rollback_token";

    // Create initial structure
    create_base_folder(test_folder)?;

    // Attempt to create a file with invalid content to trigger rollback
    let result = create_contract_file(
        "InvalidToken",
        test_folder,
        "", // Empty content to trigger error
        SUB_FOLDER,
    );

    // Verify folder is cleaned up after error
    let folder_exists = Path::new(test_folder).exists();
    assert!(!folder_exists, "Folder should be cleaned up after error");

    assert!(result.is_err());
    Ok(())
}

// Note: Disk space test is commented out as it requires root permissions to modify disk quotas
// #[tokio::test]
// async fn test_create_token_disk_space() -> Result<()> {
//     // This test would require root permissions to set disk quotas
//     // For now, we rely on the error type being properly propagated
//     Ok(())
// }
