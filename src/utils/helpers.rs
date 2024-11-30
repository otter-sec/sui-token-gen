use std::fs;

pub fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

pub fn create_base_folder(base_folder: &str) {
    let sources_folder = format!("{}/sources", base_folder);
    fs::create_dir_all(&sources_folder).expect("Failed to create folder structure");
}
