use regex::Regex;
use std::fs;

use crate::{errors::TokenGenErrors, variables::SUB_FOLDER, Result};

use super::prompts::TokenInfo;

//URL is github url or not
pub fn is_valid_github_url(url: &str) -> bool {
    let github_url_pattern = r"^https?://(www\.)?github\.com/[\w\-]+/[\w\-]+(/)?$";
    let re = Regex::new(github_url_pattern).expect("Invalid pattern");
    re.is_match(url)
}

//Returing filtered alphanumeric characters string
pub fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

// Creating contract base folder and sources folder
pub fn create_base_folder(base_folder: &str) -> Result<()> {
    let sources_folder: String = format!("{}/{}", base_folder, SUB_FOLDER);
    if let Err(e) = fs::create_dir_all(&sources_folder) {
        return Err(TokenGenErrors::FileIoError(e));
    }
    Ok(())
}

//Removing: comments, empty lines, whitespaces
pub fn filter_token_content(content: &str) -> Result<String> {
    let re = Regex::new(r"///.*|//.*").unwrap();
    let cleaned_content: std::borrow::Cow<'_, str> = re.replace_all(content, "");
    let non_empty_lines: Vec<&str> = cleaned_content
        .lines()
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.trim())
        .collect();
    Ok(non_empty_lines.join(""))
}

//Extracting decimals, symbol, name, description, is_frozen from contract (String)
pub fn get_token_info(content: &str) -> Result<TokenInfo> {
    //Default values
    let mut token_info = TokenInfo::default();
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
                    let trimmed = char
                        .trim_end_matches(',')
                        .trim_start_matches(" b\"")
                        .to_string();
                    args.push(trimmed);
                    char.clear();
                }

                if char.is_empty() {
                    char = "".to_string() + arg.trim_end_matches("\",");
                } else {
                    char += " ";
                    char += arg.trim_end_matches("\",");
                }
            }
            if args.len() >= 4 {
                token_info.decimals = args[0].trim().parse().unwrap_or(0);
                token_info.symbol = args[1]
                    .trim_start_matches("b\"")
                    .trim_end_matches("\"")
                    .to_string();
                token_info.name = args[2]
                    .trim_start_matches("b\"")
                    .trim_end_matches("\"")
                    .to_string();
                token_info.description = args[3]
                    .trim_start_matches("b\"")
                    .trim_end_matches("\"")
                    .to_string();
            }
        } else if token.contains("transfer::public_freeze_object") {
            token_info.is_frozen = true;
        }
    }

    Ok(token_info)
}

pub fn is_running_test() -> bool {
    std::env::var("RUNNING_TEST").map_or(false, |val| val == "true")
}
