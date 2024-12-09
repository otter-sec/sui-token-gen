use crate::{
    errors::TokenGenErrors,
    utils::helpers::sanitize_name,
    variables::{SUB_FOLDER, SUI_PROJECT, SUI_PROJECT_SUB_DIR},
    Result,
};
use chrono::{Datelike, Utc};
use serde::Serialize;
use std::{collections::HashMap, env, fs};
use tera::{Context, Tera};

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

//Generating token content and creating .move contract file
pub fn create_generate_token(
    decimals: u8,
    symbol: String,
    name: &str,
    description: String,
    is_frozen: bool,
    base_folder: &str,
) -> Result<()> {
    //Filtering alphanumeric characters only
    let slug = sanitize_name(name.to_owned());

    //Generating token content
    let token_template: String = generate_token(decimals, symbol, name, description, is_frozen)?;

    //Create move contract file in base_folder/sources folder
    let sources_folder: String = format!("{}/{}", base_folder, SUB_FOLDER);
    let file_name: String = format!("{}/{}.move", sources_folder, slug.to_lowercase());

    if let Err(e) = fs::write(&file_name, token_template) {
        return Err(TokenGenErrors::FileIoError(e));
    }
    Ok(())
}

//Creating token content with user inputs
pub fn generate_token(
    decimals: u8,
    symbol: String,
    name: &str,
    description: String,
    is_frozen: bool,
) -> Result<String> {
    //Filtering alphanumeric characters only
    let slug = sanitize_name(name.to_owned());

    let module_name = slug.clone();
    let token_type = slug.to_uppercase();

    //Getting current directory
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

    let token_template = tera.render("token_template.move", &context);
    match token_template {
        Ok(template) => Ok(template),
        Err(e) => Err(TokenGenErrors::TeraError(e)),
    }
}

//Generating move.toml file with basic requirements
pub fn generate_move_toml(package_name: &str) -> Result<()> {
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

    let file_path: String = format!("{}/Move.toml", package_name);
    if let Err(e) = fs::write(&file_path, toml_content) {
        return Err(TokenGenErrors::FileIoError(e));
    }
    Ok(())
}
