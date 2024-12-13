use clap::{Parser, Subcommand};
use errors::TokenGenErrors;

mod commands;
mod errors;
mod rpc_client;
pub mod test_utils;
mod utils;
mod variables;

use rpc_client::{initiate_client, TokenGenClient};
use variables::ADDRESS;

#[derive(Parser, Debug)]
#[command(
    author = "Osec",
    version = "1.0.0",
    about = "Create and verify Sui Coin contracts",
    long_about = "Sui Token Generator is a CLI tool that helps you create and verify tokens contracts."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Creates a new token contract.")]
    Create,
    #[command(about = "Verifies an existing contract from repo or local.")]
    Verify {
        /// Path to the file
        #[arg(short, long)]
        path: Option<String>,

        /// URL to fuzz
        #[arg(short, long)]
        url: Option<String>,
    },
}

// Define Return type for main function as Result<T, TokenGenErrors>
pub type Result<T> = std::result::Result<T, TokenGenErrors>;

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();

    // Initialize the RPC client
    let client: TokenGenClient = initiate_client(ADDRESS)
        .await
        .map_err(|e| TokenGenErrors::InvalidInput(format!("Failed to initiate client: {}", e)))?;

    match &cli.command {
        Commands::Create => {
            commands::create::create_token(client).await?;
        }
        Commands::Verify { path, url } => {
            if path.is_none() && url.is_none() {
                return Err(TokenGenErrors::InvalidInput(
                    "Error: Either --path or --url must be provided.".to_string(),
                ));
            }

            if let Some(path) = path {
                commands::verify::verify_token_from_path(path, client.clone()).await?;
            }

            if let Some(url) = url {
                commands::verify::verify_token_using_url(url, client).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{env, fs, path::Path};
    use tarpc::context;

    use crate::{
        commands::verify::verify_token_using_url,
        errors::TokenGenErrors,
        rpc_client::TokenGenClient,
        test_utils::setup_test_client,
        utils::{
            generation::{create_base_folder, create_contract_file, create_move_toml},
            helpers::sanitize_name,
        },
        variables::{ADDRESS, SUB_FOLDER},
        Result,
    };

    async fn test_initiate_client() -> Result<TokenGenClient> {
        setup_test_client(ADDRESS).await
    }
    #[tokio::test]
    async fn test_environment_specific_token_creation() -> Result<()> {
        let client = setup_test_client(ADDRESS).await?;
        for env in ["devnet", "testnet", "mainnet"] {
            let result = client
                .create(
                    context::current(),
                    6,
                    "TestToken".to_string(),
                    "TEST".to_string(),
                    "Test Description".to_string(),
                    false,
                    env.to_string(),
                )
                .await;
            assert!(
                result.is_ok(),
                "Token creation failed for environment: {}",
                env
            );
        }
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_token_rpc_error_mapping() -> Result<()> {
        let client = setup_test_client(ADDRESS).await?;

        // Test invalid URL scenario
        let invalid_url = "https://invalid-url-that-does-not-exist";
        let result = verify_token_using_url(invalid_url, client.to_owned()).await;
        assert!(matches!(result, Err(TokenGenErrors::RpcError(_))));

        // Test malformed URL scenario
        let malformed_url = "not-a-url";
        let result = verify_token_using_url(malformed_url, client).await;
        assert!(matches!(result, Err(TokenGenErrors::RpcError(_))));
        Ok(())
    }

    #[tokio::test]
    async fn test_error_propagation_flow() -> Result<()> {
        let client = setup_test_client(ADDRESS).await?;

        // Test invalid decimals
        let result = client
            .create(
                context::current(),
                255, // Invalid decimals
                "TestToken".to_string(),
                "TEST".to_string(),
                "Description".to_string(),
                false,
                "devnet".to_string(),
            )
            .await?;
        assert!(result.is_err());

        // Test empty name
        let result = client
            .create(
                context::current(),
                6,
                "".to_string(), // Empty name
                "TEST".to_string(),
                "Description".to_string(),
                false,
                "devnet".to_string(),
            )
            .await?;
        assert!(result.is_err());

        // Test invalid environment should be succeed. i.e taking devnet as default if it's invalid
        let result = client
            .create(
                context::current(),
                6,
                "TestToken".to_string(),
                "TEST".to_string(),
                "Description".to_string(),
                false,
                "invalid_env".to_string(),
            )
            .await?;
        assert!(result.is_ok());

        Ok(())
    }

    #[tokio::test]
    async fn test_create_command() -> Result<()> {
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

        println!("Token Content:\n{}", token_content);
        println!("Move.toml Content:\n{}", move_toml);

        // Create base folder
        create_base_folder(&base_folder)?;

        // Generate Move.toml file
        create_move_toml(base_folder.to_owned(), move_toml).expect("Failed to create Move.toml");

        // Generate token contract file
        create_contract_file(
            name.to_owned(),
            base_folder.to_owned(),
            token_content,
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
    async fn test_verify_command_valid_file() -> Result<()> {
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
    async fn test_verify_command_invalid_file() -> Result<()> {
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
            .await;
        assert!(response.is_err(), "Verification failed");

        Ok(())
    }

    #[tokio::test]
    async fn test_verify_command_valid_git() -> Result<()> {
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
    async fn test_verify_command_invalid_git() -> Result<()> {
        let valid_url = "https://github.com/meumar-osec/sui-token1";

        // Initialize the RPC client
        let client: TokenGenClient = test_initiate_client().await?;

        // Call verify_token
        let response = verify_token_using_url(valid_url, client).await;
        assert!(response.is_err(), "Failed to verify URL");
        Ok(())
    }
}
