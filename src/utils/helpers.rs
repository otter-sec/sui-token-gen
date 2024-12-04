use std::fs;
use std::path::Path;
use url::Url;
use regex::Regex;

use crate::variables::SUB_FOLDER;


fn is_url(input: &str) -> bool {
    Url::parse(input).is_ok()
}

//Path exists or not
fn is_local_file(input: &str) -> bool {
    let path = Path::new(input);
    path.exists()
}

//URL is github url or not
fn is_valid_github_url(url: &str) -> bool {
    let github_url_pattern = r"^https?://(www\.)?github\.com/[\w\-]+/[\w\-]+(/)?$";
    let re = Regex::new(github_url_pattern).expect("Invalid pattern");
    re.is_match(url)
}

//Validating path parameter and returning input type
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

//Returing filtered alphanumeric characters string
pub fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

// Creating contract base folder and sources folder
pub fn create_base_folder(base_folder: &str) {
    let sources_folder: String = format!("{}/{}", base_folder, SUB_FOLDER);
    fs::create_dir_all(&sources_folder).expect("Failed to create folder structure");
}

//Removing: comments, empty lines, whitespaces
pub fn filter_token_content(content: &str) -> String {
    let re = Regex::new(r"///.*|//.*").unwrap();
    let cleaned_content: std::borrow::Cow<'_, str> = re.replace_all(content, "");
    let non_empty_lines: Vec<&str> = cleaned_content.lines().filter(|line| !line.trim().is_empty()).map(|line| line.trim()).collect();
    non_empty_lines.join("")
}

//Extracting decimals, symbol, name, description, is_frozen from contract (String)
pub fn get_token_info(content: &str) -> (u8, String, String, String, bool) {
    //Default values
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
            while let Some(arg) = tokens.next() {
                if arg.ends_with(");") || arg.ends_with(")") || arg.ends_with("option::none(),") {
                    let trimmed = char.trim_end_matches(&[')', ';'][..]).to_string();
                    args.push(trimmed);
                    break;
                }

                if arg.starts_with("b\"") {
                    let trimmed = char.trim_end_matches(',').trim_start_matches(" b\"").to_string();
                    args.push(trimmed);
                    char.clear();
                }
                
                if char.is_empty(){
                    char = "".to_string() + arg.trim_end_matches("\",");
                }else{
                    char += " ";
                    char += arg.trim_end_matches("\",");
                }
            }
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

    (decimals, symbol, name, description, is_frozen)
}