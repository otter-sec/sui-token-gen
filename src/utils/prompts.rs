use inquire::{required, CustomType, Select, Text};

pub fn get_user_prompt() -> Result<(u8, String, String, String, bool), String> {
    const FROZEN_OPTIONS: [&str; 2] = ["Yes", "No"];
    const CANCEL_ERROR_MESSAGE: &str = "Operation was canceled by the user";

    // Prompt for decimals
    let decimals: u8 = loop {
        match CustomType::new("Decimals: ")
            .with_help_message("e.g. 6")
            .with_formatter(&|i: u8| format!("{i}"))
            .with_error_message("Please type a valid number")
            .prompt()
        {
            Ok(value) if value > 0 => break value,
            Ok(_) => eprintln!("Decimals must be greater than 0. Please try again."),
            Err(e) => {
                if e.to_string() == CANCEL_ERROR_MESSAGE {
                    panic!("{CANCEL_ERROR_MESSAGE}");
                } else {
                    eprintln!("Error: {e}. Please try again.");
                }
            }
        }
    };

    // Prompt for symbol
    let symbol: String = Text::new("Symbol: ")
        .with_validator(required!("Symbol is required"))
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for name
    let name: String = Text::new("Name: ")
        .with_validator(required!("Name is required"))
        .with_help_message("e.g. MyToken")
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for description - optional
    let description: String = Text::new("Description: ")
        .with_help_message("Optional")
        .prompt()
        .unwrap_or_default();

    // Prompt for token type
    let frozen_metadata = Select::new("Frozen metadata?", &FROZEN_OPTIONS)
        .prompt()
        .map_err(|e| e.to_string())?;
    let is_frozen = frozen_metadata.value == "Yes";

    Ok((decimals, symbol, name, description, is_frozen))
}
