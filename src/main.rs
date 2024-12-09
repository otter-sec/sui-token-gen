use clap::{Parser, Subcommand};
use errors::TokenGenErrors;

mod commands;
mod errors;
mod utils;
mod variables;

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

    match &cli.command {
        Commands::Create => {
            commands::create::create_token().await?;
        }
        Commands::Verify { path, url } => {
            if path.is_none() && url.is_none() {
                eprintln!("Error: Either --path or --url must be provided.");
                std::process::exit(1);
            }

            if let Some(path) = path {
                commands::verify::verify_token_from_path(path).await?;
            }

            if let Some(url) = url {
                commands::verify::verify_token_using_url(url).await?;
            }
        }
    }
    Ok(())
}

#[cfg(test)]
mod test {
    use std::{env, fs, path::Path};

    use crate::{
        commands::verify::{verify_token_from_path, verify_token_using_url},
        utils::{
            generation::{create_generate_token, generate_move_toml},
            helpers::{create_base_folder, sanitize_name},
        },
        variables::{SUB_FOLDER, SUI_GITREPO_DIR},
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
        let base_folder = "test_base_folder";

        // If already test base folder existed delete that folder
        if Path::new(base_folder).exists() {
            fs::remove_dir_all(base_folder).expect("Failed to delete test base folder");
        }

        // Creating token
        let _ = create_base_folder(base_folder);
        let _ = generate_move_toml(base_folder);
        let _ = create_generate_token(
            decimals,
            symbol.clone(),
            name,
            description.clone(),
            is_frozen,
            base_folder,
        );

        // Check contract base folders created
        let sources_folder = format!("{}/{}", base_folder, SUB_FOLDER);
        let toml_file = format!("{}/Move.toml", base_folder);
        let move_file = format!("{}/{}.move", sources_folder, sanitize_name(name.to_owned()));

        assert!(
            Path::new(&sources_folder).exists(),
            "Sources folder not created"
        );
        assert!(Path::new(&toml_file).exists(), "Move.toml file not created");
        assert!(
            Path::new(&move_file).exists(),
            "Move contract file not created"
        );

        // Read and Check move.toml file
        let toml_content = fs::read_to_string(&toml_file).expect("Failed to read toml file");
        assert!(
            toml_content.contains("0.0.1"),
            "Move.toml file does not contain the correct version"
        );
        assert!(
            toml_content.contains(base_folder),
            "Move.toml file does not contain the correct package name"
        );

        // Read and Check move contract
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

        // Delete test base folder
        fs::remove_dir_all(base_folder).expect("Failed to deletetest base folder");
    }

    #[tokio::test]
    async fn test_verify_command_valid_file() {
        let temp_dir = "test_verify_valid_temp_folder";
        let file_name = "valid_token.move";
        let file_path = format!("{}/{}", temp_dir, file_name);
        std::env::set_var("RUNNING_TEST", "true");

        // If already test test temp folder folder existed and delete that folder
        if Path::new(temp_dir).exists() {
            fs::remove_dir_all(temp_dir).expect("Failed to delete test folder");
        }
        fs::create_dir(temp_dir).expect("Failed to create temp folder");

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let templates_path = format!("{}/src/test_tokens/valid_token.move", current_dir.display());

        // Read content from the existing valid token file
        let valid_content =
            fs::read_to_string(templates_path).expect("Failed to read valid token file");

        // Write the content to the temporary test file
        fs::write(&file_path, valid_content).expect("Failed to write test Move file");

        //Call verify_token
        let _ = verify_token_from_path(&file_path).await;

        // Check no errors occurred and the file still exists
        assert!(
            Path::new(&file_path).exists(),
            "Move file should still exist"
        );

        // delete all test files
        fs::remove_dir_all(temp_dir).expect("Failed to clean up test folder");
    }

    #[tokio::test]
    async fn test_verify_command_invalid_file() {
        let temp_dir = "test_verify_invalid_temp_folder";
        let file_name = "invalid_token.move";
        let file_path = format!("{}/{}", temp_dir, file_name);
        std::env::set_var("RUNNING_TEST", "true");

        // If already test test temp folder folder existed and delete that folder
        if Path::new(temp_dir).exists() {
            fs::remove_dir_all(temp_dir).expect("Failed to clean test folder");
        }
        fs::create_dir(temp_dir).expect("Failed to create temp folder");

        let current_dir = env::current_dir().expect("Failed to get current directory");
        let templates_path = format!(
            "{}/src/test_tokens/invalid_token.move",
            current_dir.display()
        );

        // Read content from the existing invalid token file
        let invalid_content =
            fs::read_to_string(templates_path).expect("Failed to read invalid token file");

        // Write the content to the temporary test file
        fs::write(&file_path, invalid_content).expect("Failed to write test Move file");

        //Call verify_token
        let _ = verify_token_from_path(&file_path).await;

        //Check no errors occurred and the file still exists
        assert!(
            Path::new(&file_path).exists(),
            "Invalid file path should still exist"
        );

        // delete all test files
        fs::remove_dir_all(temp_dir).expect("Failed to clean up test folder");
    }

    #[tokio::test]
    async fn test_verify_command_valid_git() {
        //Testing repo
        let valid_url = "https://github.com/meumar-osec/test-sui-token";
        std::env::set_var("RUNNING_TEST", "true");

        //Call verify_token
        let _ = verify_token_using_url(valid_url).await;

        let cloned_repo_path = Path::new(SUI_GITREPO_DIR.trim_start_matches("./"));

        assert!(cloned_repo_path.exists(), "Failed to clone repo");

        let sources_folder = cloned_repo_path.join(SUB_FOLDER);
        assert!(
            sources_folder.exists(),
            "Cloned repository should contain the 'sources' folder"
        );

        // delete all test files
        if cloned_repo_path.exists() {
            fs::remove_dir_all(cloned_repo_path).expect("Failed to delete up cloned repository");
        }
    }

    #[tokio::test]
    async fn test_verify_command_invalid_git() {
        let valid_url = "https://github.com/meumar-osec/sui-token1";
        std::env::set_var("RUNNING_TEST", "true");

        //Call verify_token
        let _ = verify_token_using_url(valid_url).await;

        let cloned_repo_path = Path::new(SUI_GITREPO_DIR.trim_start_matches("./"));
        assert!(
            !cloned_repo_path.exists(),
            "The repository should be cloned to the specified directory"
        );

        let sources_folder = cloned_repo_path.join(SUB_FOLDER);
        assert!(
            !sources_folder.exists(),
            "The cloned repository should contain the 'sources' folder"
        );

        // delete all test files
        if cloned_repo_path.exists() {
            fs::remove_dir_all(cloned_repo_path).expect("Failed to clean up cloned repository");
        }
    }
}
