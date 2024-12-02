use std::fs;
use std::path::Path;
use url::Url;
use regex::Regex;

pub fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

pub fn create_base_folder(base_folder: &str) {
    let sources_folder: String = format!("{}/sources", base_folder);
    fs::create_dir_all(&sources_folder).expect("Failed to create folder structure");
}

fn is_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}

fn is_local_file(input: &str) -> bool {
    let path = Path::new(input);
    path.exists()
}

fn is_valid_github_url(url: &str) -> bool {
    let github_url_pattern = r"^https?://(www\.)?github\.com/[\w\-]+/[\w\-]+(/)?$";
    let re = Regex::new(github_url_pattern).expect("Invalid pattern");
    re.is_match(url)
}

pub fn validate_input(input: &str) -> (bool, String, Option<&'static str>) {
    if is_local_file(input) {
        return (true, String::new(), Some("FILE"));
    }

    if is_url(input) && is_valid_github_url(input) {
        return (true, String::new(), Some("URL"));
    }

    let error_message = if is_url(input) {
        "The URL provided is not a valid GitHub URL.".to_string()
    } else {
        "The input is neither a valid file path nor a valid URL.".to_string()
    };

    (false, error_message, None)
}
