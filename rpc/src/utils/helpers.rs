use rand::{distributions::Alphanumeric, Rng};
use regex::Regex;
use std::{fs, path::Path};

use crate::utils::{errors::TokenGenErrors, variables::TokenDetails};

// URL is github/gitlab url or not
pub fn is_valid_repository_url(url: &str) -> Result<bool, TokenGenErrors> {
    let repository_url_pattern = r"^https?://(www\.)?(github|gitlab)\.com/[\w\-]+/[\w\-]+/?$";
    let re = Regex::new(repository_url_pattern).expect("Invalid pattern");
    re.is_match(url)
        .then_some(true)
        .ok_or(TokenGenErrors::InvalidGitUrl)
}

// Returing filtered alphanumeric characters string
pub fn sanitize_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

// Function to sanitize the repository name and append a random string
pub fn sanitize_repo_name_with_random(repo_name: &str) -> String {
    // Replace "..", "/", and "\\" to remove path traversal and invalid characters
    let sanitized_name = sanitize_repo_name(repo_name);

    // Generate a random 8-character alphanumeric string
    let random_suffix: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(8)
        .map(char::from)
        .collect();

    // Append the random string to the sanitized name
    format!("{}_{}", sanitized_name, random_suffix)
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
                    let trimmed = char.trim_end_matches(',')
                        .trim_start_matches(" b\"")
                        .to_string();
                    args.push(trimmed);
                    char.clear();
                }

                if char.is_empty() {
                    char = arg.trim_end_matches("\",").to_string();
                } else {
                    char.push(' ');
                    char.push_str(arg.trim_end_matches("\","));
                }
            }

            // Handle args carefully
            if args.len() >= 4 {
                decimals = args[0].trim().parse().unwrap_or(0);
                symbol = args[1].trim_start_matches("b\"").trim_end_matches("\"").to_string();
                name = args[2].trim_start_matches("b\"").trim_end_matches("\"").to_string();
                description = args[3].trim_start_matches("b\"").trim_end_matches("\"").to_string();
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

// Sanitizes the provided repository name by removing path traversal sequences
// like "..". Ensures the resulting name is safe for use as a directory name.
pub fn sanitize_repo_name(repo_name: &str) -> String {
    // Replace ".." with an empty string to remove path traversal components
    repo_name
        .replace("..", "")
        .replace("/", "")
        .replace("\\", "")
}

pub fn check_cloned_contract(path: &Path) -> Result<(), TokenGenErrors> {
    if path.exists() && path.is_dir() {
        fs::remove_dir_all(path).map_err(|e| TokenGenErrors::FileIoError(e.to_string()))?;
    }
    Ok(())
}

// Need to clean the cloned contract on every error
pub struct CleanupGuard<'a> {
    pub path: &'a Path,
}

impl Drop for CleanupGuard<'_> {
    fn drop(&mut self) {
        if let Err(e) = check_cloned_contract(self.path) {
            eprintln!("Failed to clean cloned contract: {:?}", e);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_safe_path_valid() {
        let valid_target = "sui-token";
        // Check if the valid target path is inside the base path
        assert_eq!(sanitize_repo_name(&valid_target), valid_target);
    }

    #[test]
    fn test_safe_path_invalid() {
        let invalid_target = "../etc/psswd";

        // Check if the invalid target path is outside the base path
        assert_eq!(sanitize_repo_name(&invalid_target), "etcpsswd");
    }
}
