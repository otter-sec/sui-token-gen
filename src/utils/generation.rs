use std::env;
use std::fs;
use std::collections::HashMap;
use tera::{Context, Tera};
use chrono::{Utc, Datelike};
use crate::utils::helpers::sanitize_name;
use serde::Serialize;

#[derive(Serialize)]
struct Package {
    name: String,
    edition: String,
    version: String
}

#[derive(Serialize)]
struct Dependency {
    #[serde(rename = "Sui")]
    sui: SuiDependency,
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

pub fn generate_token(
    decimals: u8,
    symbol: String,
    name: String,
    description: String,
    is_frozen: bool,
    base_folder: &str,
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

    let token_template: String = tera.render("token_template.move", &context).unwrap();
    let sources_folder: String = format!("{}/sources", base_folder);
    let file_name: String = format!("{}/{}.move", sources_folder, slug);
    fs::write(&file_name, token_template).expect("Failed to write Move contract file");
}

pub fn generate_move_toml(package_name: &str) {
    let current_year: u32 = Utc::now().year_ce().1;

    let move_toml = MoveToml {
        package: Package {
            name: package_name.to_string(),
            edition: format!("{}.beta", current_year),
            version: "0.0.1".to_string()
        },
        dependencies: Dependency {
            sui: SuiDependency {
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

    let toml_content: String = toml::to_string(&move_toml).expect("Failed to serialize Move.toml");

    let file_path: String = format!("{}/Move.toml", package_name);
    fs::write(&file_path, toml_content).expect("Failed to write Move.toml");
}
