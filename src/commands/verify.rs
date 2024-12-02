use crate::utils::helpers::{is_local_file, is_url};

// Verify command
pub async fn verify_token(path_or_url: &str) {
    println!("Verifying token at: {}", path_or_url);

    if !is_url(path_or_url) && !is_local_file(path_or_url) {
        eprintln!("The input is neither a valid URL nor a local file path.");
        return;
    }
}