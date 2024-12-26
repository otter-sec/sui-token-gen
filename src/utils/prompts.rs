use inquire::{required, Confirm, Select, Text};
use regex::Regex;

use crate::{
    errors::TokenGenErrors,
    variables::{CANCEL_ERROR_MESSAGE, FROZEN_OPTIONS},
    Result,
};

use super::helpers::sanitize_name;

// Define struct to hold token information from user input.
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TokenInfo {
    pub decimals: u8,        // Number of decimal places for the token.
    pub symbol: String,      // Symbol for the token (e.g., "ETH").
    pub name: String,        // Name of the token (e.g., "Ethereum").
    pub description: String, // Optional description of the token.
    pub is_frozen: bool,     // Indicates if metadata is frozen.
    pub environment: String, // Blockchain environment (e.g., mainnet, devnet, testnet).
}

// Default implementation for `TokenInfo` to provide initial values.
#[allow(clippy::derivable_impls)]
impl Default for TokenInfo {
    fn default() -> Self {
        Self {
            decimals: 0,
            symbol: String::new(),
            name: String::new(),
            description: String::new(),
            is_frozen: false,
            environment: "devnet".to_string(),
        }
    }
}

/**
 * Prompts the user for token-related input and validates their responses.
 *
 * # Returns
 * - `Ok(TokenInfo)`: Contains the user's input for token configuration.
 * - `Err(TokenGenErrors)`: If an error occurs during user input or validation.
 */
pub fn get_user_prompt() -> Result<TokenInfo> {
    // Regular expressions for validating user input
    let valid_regex: Regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap(); // Allows only alphanumeric characters and spaces.
    let symbol_regex: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap(); // Allows only alphanumeric characters.

    // Prompt for decimals: must be a positive number greater than 0.
    let decimals: u8 = loop {
        match inquire::CustomType::<u8>::new("Decimals: ")
            .with_help_message("e.g. 6")
            .with_formatter(&|i: u8| format!("{i}"))
            .with_error_message("Please type a valid number")
            .prompt()
        {
            Ok(value) if value > 0 => break value,
            Ok(_) => eprintln!("Decimals must be greater than 0. Please try again."),
            Err(e) => {
                if e.to_string() == CANCEL_ERROR_MESSAGE {
                    return Err(TokenGenErrors::PromptError(e));
                } else {
                    eprintln!("Error: {e}. Please try again.");
                }
            }
        }
    };

    // Prompt for the token symbol: must be alphanumeric and ≤ 5 characters.
    let symbol: String = Text::new("Symbol: ")
        .with_validator(required!("Symbol is required"))
        .with_validator(&|input| {
            if symbol_regex.is_match(input) {
                if input.len() <= 5 {
                    Ok(())
                } else {
                    Err("Symbol has to be less than 5 letters".into())
                }
            } else {
                Err("Symbol can only contain alphabets and numbers".into())
            }
        })
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;

    // Prompt for the token name: must be alphanumeric with optional spaces.
    let mut name: String = Text::new("Name: ")
        .with_validator(required!("Name is required"))
        .with_help_message("e.g. MyToken")
        .with_validator(&|input| {
            if valid_regex.is_match(input) {
                Ok(())
            } else {
                Err("Name can only contain alphabets, numbers, and whitespace".into())
            }
        })
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;

    // Check if a folder with the same name exists in the current directory.
    let current_dir = std::env::current_dir().map_err(|_| TokenGenErrors::CurrentDirectoryError)?;
    let mut base_folder_path = current_dir.join(sanitize_name(&name).to_lowercase());

    while base_folder_path.exists() {
        if Confirm::new("A folder with this name already exists. Do you want to overwrite it?")
            .with_default(false)
            .prompt()
            .unwrap_or(false)
        {
            break; // User chose to overwrite, proceed with the existing folder name.
        } else {
            // Ask the user to provide a new folder name.
            name = Text::new("Please provide a new token name:")
                .with_validator(required!("Name is required"))
                .with_validator(&|input| {
                    if valid_regex.is_match(input) {
                        Ok(())
                    } else {
                        Err("Name can only contain alphabets, numbers, and whitespace".into())
                    }
                })
                .prompt()
                .map_err(TokenGenErrors::PromptError)?;
            base_folder_path = current_dir.join(sanitize_name(&name).to_lowercase());
        }
    }

    // Prompt for the token description: optional and must be alphanumeric with spaces.
    let description: String = Text::new("Description: ")
        .with_help_message("Optional")
        .with_validator(&|input| {
            if input.is_empty() || valid_regex.is_match(input) {
                Ok(())
            } else {
                Err("Description can only contain alphabets, numbers, and whitespace".into())
            }
        })
        .prompt()
        .unwrap_or_default();

    // Prompt for frozen metadata: user selects "Yes" or "No".
    let frozen_metadata = Select::new("Frozen metadata?", &FROZEN_OPTIONS)
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;
    let is_frozen: bool = frozen_metadata.value == "Yes";

    // Prompt for blockchain environment: options are "mainnet", "devnet", and "testnet".
    let env_options = vec!["mainnet", "devnet", "testnet"];
    let default_index = env_options.iter().position(|&r| r == "devnet").unwrap(); // Default to "devnet".
    let env_option = Select::new("Select environment:", &env_options)
        .with_starting_cursor(default_index)
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;

    let environment: String = env_option.value;

    // Return the collected and validated token information.
    Ok(TokenInfo {
        decimals,
        symbol,
        name,
        description,
        is_frozen,
        environment,
    })
}
