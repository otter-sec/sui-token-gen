use regex::Regex;
use url::Url;

use crate::{errors::TokenGenErrors, Result};

/**
 * Filters out non-alphanumeric characters from the input string.
 *
 * # Arguments
 * - `name`: A string slice representing the input name to be sanitized.
 *
 * # Returns
 * - A `String` containing only alphanumeric characters from the input.
 */
pub fn sanitize_name(name: &str) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

/**
 * Validates whether a given URL is a valid GitHub or GitLab repository URL.
 *
 * # Arguments
 * - `url`: A string slice representing the URL to be validated.
 *
 * # Returns
 * - `Ok(())`: If the URL is valid.
 * - `Err(TokenGenErrors::InvalidGitUrl)`: If the URL is invalid.
 *
 * # Validation Criteria
 * - The URL must start with `http` or `https`.
 * - The domain must be `github.com` or `gitlab.com`.
 * - The path must follow the pattern `/user/repository`, where `user` and `repository` consist of alphanumeric characters, underscores, or hyphens.
 * - An optional trailing `/` is allowed.
 */
pub fn is_valid_repository_url(url: &str) -> Result<()> {
    // Regular expression pattern for validating GitHub and GitLab repository URLs.
    let repository_url_pattern = r"^https?://(www\.)?(github|gitlab)\.com/[\w\-]+/[\w\-]+/?$";

    // Compile the regular expression; panic only if the pattern itself is invalid (unlikely in this case).
    let re = Regex::new(repository_url_pattern).expect("Invalid pattern");

    // Check if the URL matches the repository URL pattern.
    if !re.is_match(url) {
        return Err(TokenGenErrors::InvalidGitUrl);
    }

    // Return success if the URL is valid.
    Ok(())
}

/// Validates and extracts the RPC URL format.
pub fn validate_rpc_url(url: &str) -> Result<String> {
    // Check if the input is already in the correct format (e.g., "127.0.0.1:5001")
    let re = Regex::new(r"^([0-9]{1,3}\.){3}[0-9]{1,3}:\d+$").unwrap();
    if re.is_match(url) {
        return Ok(url.to_string());
    }

    // Parse the URL and extract the host and port
    let parsed_url = Url::parse(url).map_err(|_| TokenGenErrors::InvalidRpcUrl)?;
    let host = parsed_url.host_str().ok_or(TokenGenErrors::InvalidRpcUrl)?;
    let port = parsed_url.port().ok_or(TokenGenErrors::InvalidRpcUrl)?;

    Ok(format!("{}:{}", host, port))
}