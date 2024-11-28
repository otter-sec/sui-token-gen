use std::fs;
use std::env;
use std::path::Path;
use inquire::{required, CustomType, Select, Text};
use tokio;
use url::Url;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: sui-token-gen <command>");
        return;
    }

    println!("args {:?}", args);

    match args[2].as_str() {
        "create" => create_token().await,
        "verify" => {
            if args.len() < 4 {
                eprintln!("Usage: sui-token-gen verify <repo URL / local path>");
            } else {
                verify_token(&args[3]).await;
            }
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
macro_rules! generate_token_contract {
    (
        name = $name:expr,
        module_name = $module_name:expr,
        token_type = $token_type:expr,
        symbol = $symbol:expr,
        decimals = $decimals:expr,
        description = $description:expr
    ) => {
        format!(
            r#"
module tokengen::{module_name} {{
    use sui::coin;
    public struct {token_type} has drop {{}}

    /// Initialize the token with treasury and metadata
    fun init(witness: {token_type}, ctx: &mut TxContext) {{
        let (treasury, metadata) = coin::create_currency(
            witness, {decimals}, b"{symbol}", b"{name}", b"{description}", option::none(), ctx
        );
        transfer::public_freeze_object(metadata);
        transfer::public_transfer(treasury, ctx.sender());
    }}

    /// Mint tokens and transfer them to the recipient
    public entry fun mint(
        treasury: &mut coin::TreasuryCap<{token_type}>,
        amount: u64,
        recipient: address,
        ctx: &mut TxContext
    ) {{
        coin::mint_and_transfer(treasury, amount, recipient, ctx);
    }}

    /// Transfer TreasuryCap ownership to a new recipient
    public entry fun transferTreasuryCap(
        treasury: coin::TreasuryCap<{token_type}>,
        recipient: address,
        _ctx: &mut TxContext
    ) {{
        transfer::public_transfer(treasury, recipient);
    }}

    /// Burn tokens to reduce total supply
    public entry fun burn(
        treasury: &mut coin::TreasuryCap<{token_type}>,
        coin_obj: coin::Coin<{token_type}>,
        _ctx: &mut TxContext
    ) {{
        coin::burn(treasury, coin_obj);
    }}

    /// Transfer tokens between two accounts
    public entry fun transfer(
        coin_obj: coin::Coin<{token_type}>,
        recipient: address,
        _ctx: &mut TxContext
    ) {{
        transfer::public_transfer(coin_obj, recipient);
    }}
}}
"#,
            module_name = $module_name,
            token_type = $token_type,
            name = $name,
            symbol = $symbol,
            decimals = $decimals,
            description = $description
        )
    };
}
// Create command Utils
async fn create_token() {
    let token_data = get_user_prompt().unwrap();
    println!("Creating token...");

    generate_token(token_data.0, token_data.1, token_data.2, token_data.3, token_data.4);
}
fn get_user_prompt() -> Result<(u8, String, String, String, bool), String> {
    const FROZEN_OPTIONS: [&str; 2] = ["Yes", "No"];

    // Prompt for decimals
    let decimals = CustomType::new("Decimals: ")
        .with_help_message("e.g. 6")
        .with_formatter(&|i: u8| format!("{i}"))
        .with_error_message("Please type a valid number")
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for symbol
    let symbol = Text::new("Symbol: ")
        .with_validator(required!("Symbol is required"))
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for name
    let name = Text::new("Name: ")
        .with_validator(required!("Name is required"))
        .with_help_message("e.g. MyToken")
        .prompt()
        .map_err(|e| e.to_string())?;

    // Prompt for description - optional
    let description = Text::new("Description: ")
        .with_help_message("Optional")
        .prompt()
        .unwrap_or_default();

    // Prompt for token type
    let frozen_metadata = Select::new("Frozen metadata?", &FROZEN_OPTIONS)
        .prompt()
        .map_err(|e| e.to_string())?;
    let is_frozen = frozen_metadata.value == "Yes";

    Ok((
        decimals,
        symbol,
        name,
        description,
        is_frozen,
    ))
}
fn sanitize_name(name: String) -> String {
    name.chars()
        .filter(|c| c.is_alphanumeric())
        .collect::<String>()
}
fn generate_token(decimals: u8, symbol: String, name: String, description: String, _is_frozen: bool) {
    let slug = sanitize_name(name.clone());
    let module_name = slug.clone();
    let token_type = slug.to_uppercase();

    let token_template = generate_token_contract!(
        name = name,
        module_name = module_name,
        token_type = token_type,
        symbol = symbol,
        decimals = decimals,
        description = description
    );

    let file_name = format!("{}.move", slug);
    fs::write(&file_name, token_template).expect("Failed to write Move contract file");

    println!("Token contract generated as {}", file_name);
}



// Verify command Utils
async fn verify_token(path_or_url: &str) {
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
