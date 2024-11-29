use std::env;
use tokio;

mod utils;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: sui-token-gen <command>");
        return;
    }

    println!("args {:?}", args);

    match args[2].as_str() {
        "create" => utils::create_token().await,
        "verify" => {
            if args.len() < 4 {
                eprintln!("Usage: sui-token-gen verify <repo URL / local path>");
            } else {
                utils::verify_token(&args[3]).await;
            }
        }
        _ => eprintln!("Unknown command: {}", args[1]),
    }
}
