use clap::{Parser, Subcommand};
use errors::TokenGenErrors;

mod commands;
mod errors;
mod rpc_client;
mod utils;
mod variables;

use rpc_client::{initiate_client, TokenGenClient};

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
    let client: TokenGenClient = initiate_client().await?;

    match &cli.command {
        Commands::Create => {
            commands::create::create_token(client).await?;
        }
        Commands::Verify { path, url } => {
            if path.is_none() && url.is_none() {
                TokenGenErrors::InvalidInput(
                    "Error: Either --path or --url must be provided.".to_string(),
                );
                std::process::exit(1);
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
        rpc_client::{initiate_client, TokenGenClient},
        utils::{
            generation::{create_base_folder, create_contract_file, create_move_toml},
            helpers::sanitize_name,
        },
        variables::SUB_FOLDER,
    };

    #[tokio::test]
    async fn test_create_command() {
        // Test user inputs
        let decimals: u8 = 6;
        let symbol: String = "SAMPLE".to_string();
        let name: &str = "SampleToken";
        let description: String = "This is a sample token for testing.".to_string();
        let is_frozen: bool = false;

        // Testing contract folder
        let base_folder = sanitize_name(name.to_owned());

        // Initialize the RPC client
        let client: TokenGenClient = match initiate_client().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to initiate client: {}", e);
                std::process::exit(1);
            }
        };

        // If the test base folder already exists, delete it
        if Path::new(&base_folder).exists() {
            fs::remove_dir_all(&base_folder).expect("Failed to delete test base folder");
        }

        // Call the `create` method and handle nested `Result`
        match client
            .create(
                context::current(),
                decimals,
                name.to_owned(),
                symbol.to_owned(),
                description.to_owned(),
                is_frozen,
            )
            .await
        {
            Ok(Ok((token_content, move_toml))) => {
                println!("Token Content:\n{}", token_content);
                println!("Move.toml Content:\n{}", move_toml);

                // Create base folder
                create_base_folder(base_folder.to_owned()).expect("Failed to create base folder");

                // Generate Move.toml file
                create_move_toml(base_folder.to_owned(), move_toml)
                    .expect("Failed to create Move.toml");

                // Generate token contract file
                create_contract_file(name.to_owned(), base_folder.to_owned(), token_content)
                    .expect("Failed to create contract file");

                // Validate folder and file creation
                let sources_folder = format!("{}/{}", base_folder, SUB_FOLDER);
                let toml_file = format!("{}/Move.toml", base_folder);
                let move_file =
                    format!("{}/{}.move", sources_folder, sanitize_name(name.to_owned()));

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
                let toml_content =
                    fs::read_to_string(&toml_file).expect("Failed to read Move.toml file");
                assert!(
                    toml_content.contains("0.0.1"),
                    "Move.toml file does not contain the correct version"
                );
                assert!(
                    toml_content.contains(&base_folder),
                    "Move.toml file does not contain the correct package name"
                );

                // Validate Move contract file content
                let move_content =
                    fs::read_to_string(&move_file).expect("Failed to read contract file");
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
            }
            Ok(Err(service_error)) => {
                eprintln!("Service error: {:?}", service_error);
            }
            Err(rpc_error) => {
                eprintln!("RPC error: {:?}", rpc_error);
            }
        }
    }

    #[tokio::test]
    async fn test_verify_command_valid_file() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let templates_path = format!("{}/src/test_tokens/valid_token.move", current_dir.display());

        // Initialize the RPC client
        let client: TokenGenClient = match initiate_client().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to initiate client: {}", e);
                std::process::exit(1);
            }
        };

        // Read content from the existing valid token file
        let valid_content =
            fs::read_to_string(templates_path).expect("Failed to read valid token file");

        let response = client
            .verify_content(context::current(), valid_content)
            .await;
        assert!(response.is_ok(), "Verification failed");

        match response {
            Ok(result) => {
                assert_eq!(
                    result.trim_matches('"'),
                    "Contract is not modified",
                    "Contract should not be modified"
                );
            }
            Err(_) => {}
        }
    }

    #[tokio::test]
    async fn test_verify_command_invalid_file() {
        let current_dir = env::current_dir().expect("Failed to get current directory");
        let templates_path = format!(
            "{}/src/test_tokens/invalid_token.move",
            current_dir.display()
        );

        // Initialize the RPC client
        let client: TokenGenClient = match initiate_client().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to initiate client: {}", e);
                std::process::exit(1);
            }
        };

        // Read content from the existing valid token file
        let valid_content =
            fs::read_to_string(templates_path).expect("Failed to read valid token file");

        let response = client
            .verify_content(context::current(), valid_content)
            .await;
        assert!(response.is_ok(), "Verification failed");

        match response {
            Ok(result) => {
                assert_eq!(
                    result.trim_matches('"'),
                    "Contract is modified",
                    "Contract should be modified"
                );
            }
            Err(_) => {}
        }
    }

    #[tokio::test]
    async fn test_verify_command_valid_git() {
        // Testing repo
        let valid_url = "https://github.com/meumar-osec/test-sui-token";

        // Initialize the RPC client
        let client: TokenGenClient = match initiate_client().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to initiate client: {}", e);
                std::process::exit(1);
            }
        };

        // Call verify_token
        let response = verify_token_using_url(valid_url, client).await;
        assert!(response.is_ok(), "Failed to verify URL");
    }

    #[tokio::test]
    async fn test_verify_command_invalid_git() {
        let valid_url = "https://github.com/meumar-osec/sui-token1";

        // Initialize the RPC client
        let client: TokenGenClient = match initiate_client().await {
            Ok(client) => client,
            Err(e) => {
                eprintln!("Failed to initiate client: {}", e);
                std::process::exit(1);
            }
        };

        // Call verify_token
        let response = verify_token_using_url(valid_url, client).await;
        assert!(response.is_err(), "Failed to verify URL");
    }
}
