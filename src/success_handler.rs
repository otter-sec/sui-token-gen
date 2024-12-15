use crate::utils::prompts::TokenInfo;
use colored::*;

#[derive(Debug)]
pub enum SuccessType {
    TokenCreated(TokenInfo),
    TokenVerified {
        path: Option<String>,
        url: Option<String>,
    },
}

pub fn handle_success(success_type: SuccessType) {
    let success_prefix = "SUCCESS: ".green().bold();
    let message = match success_type {
        SuccessType::TokenCreated(token_info) => {
            format!("Contract has been generated!\nToken Details:\n  Name: {}\n  Symbol: {}\n  Decimals: {}\n  Environment: {}\n  Description: {}\n  Frozen: {}",
                token_info.name,
                token_info.symbol,
                token_info.decimals,
                token_info.environment,
                if token_info.description.is_empty() { "None".to_string() } else { token_info.description },
                if token_info.is_frozen { "Yes" } else { "No" }
            )
        }
        SuccessType::TokenVerified { path, url } => match (path, url) {
            (Some(p), None) => format!("Verified successfully from path: {}", p),
            (None, Some(u)) => format!("Verified successfully from url: {}", u),
            _ => "Verified successfully".to_string(),
        },
    };
    println!("{}{}", success_prefix, message);
}
