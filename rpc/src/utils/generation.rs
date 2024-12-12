use chrono::{Datelike, Utc};
use serde::Serialize;
use std::{collections::HashMap, env};

use crate::utils::helpers::sanitize_name;
use tera::{Context, Tera};

use crate::utils::variables::{SUI_PROJECT, SUI_PROJECT_SUB_DIR};

#[derive(Serialize)]
struct Package {
    name: String,
    edition: String,
    version: String,
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

// Creating token content with user inputs
pub fn generate_token(
    decimals: u8,
    symbol: String,
    name: &str,
    description: String,
    is_frozen: bool,
    is_test: bool
) -> String {
    // Filtering alphanumeric characters only
    let slug = sanitize_name(name.to_owned());

    let module_name = slug.clone();
    let token_type = slug.to_uppercase();

    // Getting current directory
    let current_dir = env::current_dir().unwrap();
    let templates_path = format!("{}/src/templates/**/*", current_dir.display());

    let tera = Tera::new(&templates_path).unwrap();

    let mut context = Context::new();
    context.insert("module_name", &module_name);
    context.insert("token_type", &token_type);
    context.insert("name", &name);
    context.insert("symbol", &symbol);
    context.insert("decimals", &decimals);
    context.insert("description", &description);
    context.insert("is_frozen", &is_frozen);

    let template_file = if is_test {
        "test_token_template.move"
    } else {
        "token_template.move"
    };
    let token_template: String = tera.render(template_file, &context).unwrap();
    token_template
}

// Generating move.toml file with basic requirements
pub fn generate_move_toml(package_name: &str) -> String {
    let current_year: u32 = Utc::now().year_ce().1;

    let move_toml = MoveToml {
        package: Package {
            name: package_name.to_string(),
            edition: format!("{}.beta", current_year),
            version: "0.0.1".to_string(),
        },
        dependencies: Dependency {
            sui: SuiDependency {
                git: SUI_PROJECT.to_string(),
                subdir: SUI_PROJECT_SUB_DIR.to_string(),
                rev: "framework/testnet".to_string(),
            },
        },
        addresses: {
            let mut addresses = HashMap::new();
            addresses.insert(package_name.to_string(), "0x0".to_string());
            addresses
        },
    };

    let toml_content: String = toml::to_string(&move_toml).unwrap();

    toml_content
}
