use regex::Regex;
use std::{fs, path::Path};

use crate::utils::{errors::TokenGenErrors, variables::TokenDetails};

// URL is github url or not
pub fn is_valid_repository_url(url: &str) -> Result<bool, TokenGenErrors> {
    let repository_url_pattern = r"^https?://(www\.)?(github)\.com/[\w\-]+/[\w\-]+/?$";
    let re = Regex::new(repository_url_pattern).expect("Invalid pattern");
    re.is_match(url).then_some(true).ok_or(TokenGenErrors::InvalidUrlNotGithub)
}

// Returing filtered alphanumeric characters string
pub fn sanitize_name(name: &String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

// Removing: comments, empty lines, whitespaces
pub fn filter_token_content(content: &str) -> String {
    content
        .lines()
        .filter_map(|line| {
            // Trim whitespace from the line
            let trimmed = line.trim();

            // Skip empty lines and lines starting and ending with comments (/// or //)
            if trimmed.is_empty()
                || trimmed.starts_with("///")
                || trimmed.starts_with("//")
                || trimmed.ends_with("///")
                || trimmed.ends_with("//")
            {
                None
            } else {
                Some(trimmed) // Keep non-empty, non-comment lines
            }
        })
        .collect::<Vec<&str>>()
        .join("")
}

// Extracting decimals, symbol, name, description, is_frozen from contract (String)
pub fn get_token_info(content: &str) -> TokenDetails {
    let mut decimals = 0;
    let mut symbol = String::new();
    let mut name = String::new();
    let mut description = String::new();
    let mut is_frozen = false;
    let mut tokens = content.split_whitespace().peekable();

    while let Some(token) = tokens.next() {
        if token.contains("witness") {
            let mut args = Vec::new();
            let mut char = String::new();
            for arg in tokens.by_ref() {
                if arg.ends_with(");") || arg.ends_with(")") || arg.ends_with("option::none(),") {
                    let trimmed = char.trim_end_matches(&[')', ';'][..]).to_string();
                    args.push(trimmed);
                    break;
                }

                if arg.starts_with("b\"") {
                    let trimmed = char
                        .trim_end_matches(',')
                        .trim_start_matches(" b\"")
                        .to_string();
                    args.push(trimmed);
                    char.clear();
                }

                if char.is_empty() {
                    char = "".to_string() + arg.trim_end_matches("\",");
                } else {
                    char += " ";
                    char += arg.trim_end_matches("\",");
                }
            }
            if args.len() >= 4 {
                decimals = args[0].trim().parse().unwrap_or(0);
                symbol = args[1]
                    .trim_start_matches("b\"")
                    .trim_end_matches("\"")
                    .to_string();
                name = args[2]
                    .trim_start_matches("b\"")
                    .trim_end_matches("\"")
                    .to_string();
                description = args[3]
                    .trim_start_matches("b\"")
                    .trim_end_matches("\"")
                    .to_string();
            }
        } else if token.contains("transfer::public_freeze_object") {
            is_frozen = true;
        }
    }

    TokenDetails {
        decimals,
        symbol,
        name,
        description,
        is_frozen,
    }
}

// Checks if the provided path contains path traversal sequences like ".."
// that might lead to directories outside the expected root.
pub fn is_safe_path(repo_name: &str) -> Result<bool, TokenGenErrors> {
    // Check for path traversal patterns (e.g., '..', '/../', '..\\')
    if repo_name.contains("..") || repo_name.contains("\\") || repo_name.contains("/") {
        return Err(TokenGenErrors::InvalidPathNotFound);
    }
    Ok(true)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::env;
    use std::path::{Path, PathBuf};

    #[test]
    fn test_safe_path_valid() {
        let valid_target = "sui-token";
        // Check if the valid target path is inside the base path
        assert_eq!(is_safe_path(&valid_target).unwrap(), true);
    }

    #[test]
    fn test_safe_path_invalid() {
        let invalid_target = "../etc/psswd";

        // Check if the invalid target path is outside the base path
        match is_safe_path(&invalid_target) {
            Err(TokenGenErrors::InvalidPathNotFound) => {
                assert_eq!(
                    "Path traversal detected in the clone path.",
                    "Path traversal detected in the clone path."
                )
            }
            _ => panic!("Expected path traversal error, but got a valid path."),
        }
    }
}
