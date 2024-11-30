use std::path::Path;
use url::Url;

// Verify command Utils
pub async fn verify_token(path_or_url: &str) {
    println!("Verifying token at: {}", path_or_url);

    if !is_url(path_or_url) && !is_local_file(path_or_url) {
        eprintln!("The input is neither a valid URL nor a local file path.");
        return;
    }
}

fn is_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}

fn is_local_file(input: &str) -> bool {
    let path = Path::new(input);
    path.exists()
}