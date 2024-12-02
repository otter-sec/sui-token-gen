use std::fs;
use std::path::Path;
use url::Url;

pub fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

pub fn create_base_folder(base_folder: &str) {
    let sources_folder: String = format!("{}/sources", base_folder);
    fs::create_dir_all(&sources_folder).expect("Failed to create folder structure");
}

pub fn is_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}

pub fn is_local_file(input: &str) -> bool {
    let path = Path::new(input);
    path.exists()
}
