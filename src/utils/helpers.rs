use once_cell::sync::Lazy;
use regex::Regex;
use url::Url;

use crate::{errors::TokenGenErrors, Result};

// Define regex patterns as constants using Lazy
static REPOSITORY_URL_PATTERN: Lazy<Regex> = Lazy::new(|| {
    Regex::new(r"^https?://(www\.)?(github|gitlab)\.com/[\w\-]+/[\w\-]+/?$")
        .expect("Invalid pattern")
});

static RPC_URL_PATTERN: Lazy<Regex> =
    Lazy::new(|| Regex::new(r"^([0-9]{1,3}\.){3}[0-9]{1,3}:\d+$").expect("Invalid pattern"));

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
    if !REPOSITORY_URL_PATTERN.is_match(url) {
        return Err(TokenGenErrors::InvalidGitUrl);
    }
    Ok(())
}

/// Validates and extracts the RPC URL format.
pub fn validate_rpc_url(url: &str) -> Result<String> {
    if RPC_URL_PATTERN.is_match(url) {
        return Ok(url.to_string());
    }

    let parsed_url = Url::parse(url).map_err(|_| TokenGenErrors::InvalidRpcUrl)?;
    let host = parsed_url.host_str().ok_or(TokenGenErrors::InvalidRpcUrl)?;
    let port = parsed_url.port().ok_or(TokenGenErrors::InvalidRpcUrl)?;

    Ok(format!("{}:{}", host, port))
}
