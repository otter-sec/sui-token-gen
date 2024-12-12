use inquire::{required, Select, Text};
use regex::Regex;

use crate::{
    errors::TokenGenErrors,
    variables::{CANCEL_ERROR_MESSAGE, FROZEN_OPTIONS},
    Result,
};

// Define struct for user prompt
pub struct TokenInfo {
    pub decimals: u8,
    pub symbol: String,
    pub name: String,
    pub description: String,
    pub is_frozen: bool,
    pub environment: String,
}

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

pub fn get_user_prompt() -> Result<TokenInfo> {
    // Regex for allowing only alphabets, numbers, and whitespace
    let valid_regex: Regex = Regex::new(r"^[a-zA-Z0-9\s]+$").unwrap();

    //Regex for allowing only alphabets, numbers
    let symbol_regex: Regex = Regex::new(r"^[a-zA-Z0-9]+$").unwrap();

    /*
        Prompt for decimals:
        number type
        greater than 0
        only contains numbers
    */
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

    /*
        Prompt for symbol:
        string type
        less than or equal to 6 letters
        only contains alphabets, numbers
    */
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
                Err("Symbol can only contain alphabets, numbers, and whitespace".into())
            }
        })
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;

    /*
        Prompt for name:
        string type
        only contains alphabets, numbers and whitespace
    */
    let name: String = Text::new("Name: ")
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

    /*
        Prompt for description - optional:
        string type
        only contains alphabets, numbers and whitespace
    */
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

    /*
        Prompt for token type:
        Options Yes or No
    */
    let frozen_metadata = Select::new("Frozen metadata?", &FROZEN_OPTIONS)
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;
    let is_frozen: bool = frozen_metadata.value == "Yes";

    /*
        Prompt for environment:
        Options mainnet, devnet, testnet
        Default: devnet
    */
    let env_options = vec!["mainnet", "devnet", "testnet"];
    let default_index = env_options.iter().position(|&r| r == "devnet").unwrap();
    let env_option = Select::new("Select environment:", &env_options)
        .with_starting_cursor(default_index)
        .prompt()
        .map_err(TokenGenErrors::PromptError)?;

    let environment: String = env_option.value;

    Ok(TokenInfo {
        decimals,
        symbol,
        name,
        description,
        is_frozen,
        environment,
    })
}
