use inquire::{required, Confirm, Select, Text};
use once_cell::sync::Lazy;
use regex::Regex;

use crate::{
    constants::{CANCEL_ERROR_MESSAGE, FROZEN_OPTIONS},
    errors::TokenGenErrors,
    CreateTokenParams, Result,
};

use super::{constants::DEFAULT_ENVIRONMENT, helpers::sanitize_name};

// Define regex as Lazy static variables
static VALID_NAME_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9\s]+$").expect("Invalid pattern"));

static SYMBOL_REGEX: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^[a-zA-Z0-9]+$").expect("Invalid pattern"));

static DESCRIPTION_REGEX: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^[a-zA-Z0-9\s.,'\!?;:(){}\[\]\-\_@#$%&*+=|~]+$").expect("Invalid pattern")
});

const DEFAULT_INDEX: usize = 1;

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
            environment: DEFAULT_ENVIRONMENT.to_string(),
        }
    }
}

/**
 * Prompts the user for token-related input and validates their responses.
 *
 * This function collects input parameters for token creation, such as name, symbol, decimals,
 * description, frozen metadata status, and environment selection. It ensures valid user input
 * through regex validation and interactive prompts.
 *
 * # Parameters
 * - `params: &CreateTokenParams`: Predefined token parameters that may contain values already provided by the user.
 *
 * # Returns
 * - `Ok(TokenInfo)`: Contains validated user input for token configuration.
 * - `Err(TokenGenErrors)`: Returns an error if input validation fails or an issue occurs during prompting.
 */
pub fn get_user_prompt(params: &CreateTokenParams) -> Result<TokenInfo> {
    // Get the current working directory
    let current_dir = std::env::current_dir().map_err(|_| TokenGenErrors::CurrentDirectoryError)?;

    // Prompt for token name (if not provided)
    let name = if let Some(ref name) = params.name {
        name.clone()
    } else {
        let mut name: String = Text::new("Name: ")
            .with_validator(required!("Name is required"))
            .with_validator(&|input| {
                if VALID_NAME_REGEX.is_match(input) {
                    Ok(())
                } else {
                    Err("Name can only contain alphabets, numbers, and whitespace".into())
                }
            })
            .prompt()
            .map_err(TokenGenErrors::PromptError)?;

        // Ensure unique token directory name
        let mut base_folder_path = current_dir.join(sanitize_name(&name).to_lowercase());

        while base_folder_path.exists() {
            if Confirm::new("A folder with this name already exists. Do you want to overwrite it?")
                .with_default(false)
                .prompt()
                .unwrap_or(false)
            {
                break;
            } else {
                name = Text::new("Please provide a new token name:")
                    .with_validator(required!("Name is required"))
                    .with_validator(&|input| {
                        if VALID_NAME_REGEX.is_match(input) {
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
        name
    };

    // Prompt for token symbol (if not provided)
    let symbol = if let Some(ref symbol) = params.symbol {
        symbol.clone()
    } else {
        Text::new("Symbol: ")
            .with_validator(required!("Symbol is required"))
            .with_validator(&|input| {
                if SYMBOL_REGEX.is_match(input) {
                    if input.len() <= 5 {
                        Ok(())
                    } else {
                        Err("Symbol must be at most 5 characters long".into())
                    }
                } else {
                    Err("Symbol can only contain alphabets and numbers".into())
                }
            })
            .prompt()
            .map_err(TokenGenErrors::PromptError)?
    };

    // Prompt for token decimals (if not provided)
    let decimals = if let Some(decimals) = params.decimals {
        decimals
    } else {
        loop {
            match inquire::CustomType::<u8>::new("Decimals: ")
                .with_help_message("Enter a value between 1 and 99 (e.g., 6 for USDC)")
                .with_formatter(&|i: u8| format!("{i}"))
                .with_error_message("Please enter a valid number")
                .prompt()
            {
                Ok(value) if value > 0 && value < 100 => break value,
                Ok(_) => {
                    eprintln!("Decimals must be between 1 and 99. Please try again.")
                }
                Err(e) => {
                    if e.to_string() == CANCEL_ERROR_MESSAGE {
                        return Err(TokenGenErrors::PromptError(e));
                    } else {
                        eprintln!("Error: {e}. Please try again.");
                    }
                }
            }
        }
    };

    // Prompt for token description (if not provided)
    let description = if let Some(ref description) = params.description {
        description.clone()
    } else {
        Text::new("Description: ")
            .with_help_message("Optional - Provide a brief token description")
            .with_validator(&|input| {
                if input.is_empty() || DESCRIPTION_REGEX.is_match(input) {
                    Ok(())
                } else {
                    Err("Description can only contain alphanumeric characters, spaces, and select special characters".into())
                }
            })
            .prompt()
            .unwrap_or_default()
    };

    // Prompt for frozen metadata status (if not provided)
    let is_frozen = if let Some(is_frozen) = params.is_frozen {
        is_frozen
    } else {
        let frozen_metadata = Select::new("Frozen metadata?", &FROZEN_OPTIONS)
            .prompt()
            .map_err(TokenGenErrors::PromptError)?;
        frozen_metadata.value == "Yes"
    };

    // Prompt for blockchain environment selection (if not provided)
    let environment = if let Some(ref environment) = params.environment {
        environment.clone()
    } else {
        let env_options = vec!["mainnet", "devnet", "testnet"];
        let env_option = Select::new("Select environment:", &env_options)
            .with_starting_cursor(DEFAULT_INDEX)
            .prompt()
            .map_err(TokenGenErrors::PromptError)?;
        env_option.value.to_string()
    };

    // Return the collected and validated token configuration
    Ok(TokenInfo {
        decimals,
        symbol,
        name,
        description,
        is_frozen,
        environment,
    })
}
