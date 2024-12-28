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
                "{}\nToken Details:\n  Name: {}\n  Symbol: {}\n  Decimals: {}\n  Description: {}\n  Frozen: {}\n  Environment: {}",
                message,
                token_info.name,
                token_info.symbol,
                token_info.decimals,
                if token_info.description.is_empty() { "None".to_string() } else { token_info.description },
                if token_info.is_frozen { "Yes" } else { "No" },
                token_info.environment
            )
        }
        // Success from token verification
        SuccessType::TokenVerified { path, url } => {
            let source = path.unwrap_or_else(|| url.unwrap_or_default());
            format!(
                "{}{}\n{}{}",
                "Verified successfully from: ",
                source,
                "Note: ".yellow(),
                "This code is tool-generated and unmodified. Verification confirms the code matches the tool's output, not the published module."
            )
        }
    };

    // Print the success message with the "SUCCESS: " prefix
    println!("{}{}", success_prefix, message);
}
