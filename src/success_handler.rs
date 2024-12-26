use crate::utils::prompts::TokenInfo;
use colored::*;

/// Enum to define different types of success events that can occur during token generation or verification.
#[derive(Debug)]
pub enum SuccessType {
    /// Represents the success of a token creation process with token details.
    TokenCreated(TokenInfo, String),

    /// Represents the success of a token verification process, which could be from a path or URL.
    TokenVerified {
        /// Optional path where the token was verified.
        path: Option<String>,

        /// Optional URL where the token was verified.
        url: Option<String>,
    },
}

/// Centralized success handler that formats and prints a success message based on the success type.
pub fn handle_success(success_type: SuccessType) {
    // Set up a success prefix to be printed in green and bold.
    let success_prefix = "SUCCESS: ".green().bold();

    // Match on the success type to generate the appropriate success message.
    let message = match success_type {
        // Success from token creation
        SuccessType::TokenCreated(token_info, message) => {
            // Format the success message with the token details.
            format!(
                "{}\nToken Details:\n  Name: {}\n  Symbol: {}\n  Decimals: {}\n  Environment: {}\n  Description: {}\n  Frozen: {}",
                message,
                token_info.name,
                token_info.symbol,
                token_info.decimals,
                token_info.environment,
                if token_info.description.is_empty() { "None".to_string() } else { token_info.description },
                if token_info.is_frozen { "Yes" } else { "No" }
            )
        }
        // Success from token verification
        SuccessType::TokenVerified { path, url } => match (path, url) {
            // If path is provided but not URL
            (Some(p), None) => format!("Verified successfully from path: {}", p),
            // If URL is provided but not path
            (None, Some(u)) => format!("Verified successfully from url: {}", u),
            // If both path and URL are provided (unlikely case)
            _ => "Verified successfully".to_string(),
        },
    };

    // Print the success message with the "SUCCESS: " prefix
    println!("{}{}", success_prefix, message);
}
