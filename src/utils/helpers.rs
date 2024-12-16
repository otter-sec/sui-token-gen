use regex::Regex;

use crate::{errors::TokenGenErrors, Result};

// Returing filtered alphanumeric characters string
pub fn sanitize_name(name: &String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

// URL is github/gitlab url or not
pub fn is_valid_repository_url(url: &str) -> Result<()> {
    let repository_url_pattern = r"^https?://(www\.)?(github|gitlab)\.com/[\w\-]+/[\w\-]+/?$";
    let re = Regex::new(repository_url_pattern).expect("Invalid pattern");
    if !re.is_match(url) {
        return Err(TokenGenErrors::InvalidGitUrl);
    }
    Ok(())
}
