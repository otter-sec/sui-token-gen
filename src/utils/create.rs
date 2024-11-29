use serde::Serialize;
use std::env;
use inquire::{required, CustomType, Select, Text};
use std::fs;
use tera::{Context, Tera};
use std::collections::HashMap;
use chrono::{Utc, Datelike};


#[derive(Serialize)]
struct Package {
    name: String,
    edition: String,
}

#[derive(Serialize)]
struct Dependency {
    Sui: SuiDependency,
}

#[derive(Serialize)]
struct SuiDependency {
    git: String,
    subdir: String,
    rev: String,
}

#[derive(Serialize)]
struct MoveToml {
    package: Package,
    dependencies: Dependency,
    addresses: HashMap<String, String>,
}

// Create command Utils
pub async fn create_token() {
    let token_data: Result<(u8, String, String, String, bool), String> = get_user_prompt();
    println!("Creating token...");
    const BASE_FOLDER: &str = "tokengen";

    if let Ok((decimals, symbol, name, description, is_frozen)) = token_data {
        create_base_folder(BASE_FOLDER);
        generate_move_toml(BASE_FOLDER);
        generate_token(decimals, symbol, name, description, is_frozen, BASE_FOLDER);
    } else {
        eprintln!("Failed to create token: {:?}", token_data.err());
    }
}

fn get_user_prompt() -> Result<(u8, String, String, String, bool), String> {
    const FROZEN_OPTIONS: [&str; 2] = ["Yes", "No"];

    // Prompt for decimals
    let decimals: u8 = CustomType::new("Decimals: ")
        .with_help_message("e.g. 6")
        .with_formatter(&|i: u8| format!("{i}"))
        .with_error_message("Please type a valid number")
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for symbol
    let symbol: String = Text::new("Symbol: ")
        .with_validator(required!("Symbol is required"))
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for name
    let name: String = Text::new("Name: ")
        .with_validator(required!("Name is required"))
        .with_help_message("e.g. MyToken")
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for description - optional
    let description: String = Text::new("Description: ")
        .with_help_message("Optional")
        .prompt()
        .unwrap_or_default();

    // Prompt for token type
    let frozen_metadata = Select::new("Frozen metadata?", &FROZEN_OPTIONS)
        .prompt()
        .map_err(|e| e.to_string())?;
    let is_frozen = frozen_metadata.value == "Yes";

    Ok((decimals, symbol, name, description, is_frozen))
}

fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}

fn generate_token(
    decimals: u8,
    symbol: String,
    name: String,
    description: String,
    is_frozen: bool,
    base_folder: &str
) {
    let slug = sanitize_name(name.clone());
    let module_name = slug.clone();
    let token_type = slug.to_uppercase();

    let current_dir = env::current_dir().expect("Failed to get current directory");
    let templates_path = format!("{}/src/templates/**/*", current_dir.display());
    let tera = Tera::new(&templates_path).expect("Failed to initialize Tera");

    let mut context = Context::new();
    context.insert("module_name", &module_name);
    context.insert("token_type", &token_type);
    context.insert("name", &name);
    context.insert("symbol", &symbol);
    context.insert("decimals", &decimals);
    context.insert("description", &description);
    context.insert("is_frozen", &is_frozen);

    let token_template = tera.render("token_template.move", &context).unwrap();
    let sources_folder = format!("{}/sources", base_folder);
    let file_name = format!("{}/{}.move", sources_folder, slug);
    fs::write(&file_name, token_template).expect("Failed to write Move contract file");
    println!("Token contract generated at {}", file_name);
}

fn create_base_folder(base_folder: &str){
    let sources_folder = format!("{}/sources", base_folder);
    fs::create_dir_all(&sources_folder).expect("Failed to create folder structure");
}

fn generate_move_toml(package_name: &str) {
    let current_year = Utc::now().year_ce().1;

    let move_toml = MoveToml {
        package: Package {
            name: package_name.to_string(),
            edition: format!("{}.beta", current_year),
        },
        dependencies: Dependency {
            Sui: SuiDependency {
                git: "https://github.com/MystenLabs/sui.git".to_string(),
                subdir: "crates/sui-framework/packages/sui-framework".to_string(),
                rev: "framework/testnet".to_string(),
            },
        },
        addresses: {
            let mut addresses = HashMap::new();
            addresses.insert(package_name.to_string(), "0x0".to_string());
            addresses
        },
    };

    let toml_content = toml::to_string(&move_toml).expect("Failed to serialize Move.toml");

    let file_path = format!("{}/Move.toml", package_name);
    fs::write(&file_path, toml_content).expect("Failed to write Move.toml");

    println!("Move.toml file generated at {}", file_path);
}