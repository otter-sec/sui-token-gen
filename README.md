# sui-token-gen-test

```
cargo run build
```
Create Command
```
cargo run sui-token-gen create   
```

Verify Command
```
cargo run sui-token-gen verify https://github.com/meumar-osec/sui-token-gen-test
```
# sui-token-gen-test
# sui-token-gen-test








use std::env;
use tokio;

mod commands;
mod utils;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: sui-token-gen <command>");
        return;
    }

    println!("args {:?}", args);

    let mut iter = args.iter();
    let mut command = None;
    let mut input = None;

    while let Some(arg) = iter.next() {
        if arg == "sui-token-gen-test" {
            command = iter.next().map(|s| s.clone()); 
            input = iter.next().map(|s| s.clone());
            break;
        }
    }

    if let Some(cmd) = command {
        match cmd.as_str() {
            "create" => commands::create::create_token().await,
            "verify" => {
                if let Some(input_value) = input {
                    commands::verify::verify_token(&input_value).await;
                } else {
                    eprintln!("Usage: sui-token-gen-test verify <repo URL / local path>");
                }
            }
            _ => eprintln!("Unknown command: {}", cmd),
        }
    } else {
        eprintln!("Expected 'sui-token-gen-test' in arguments.");
    }
}

