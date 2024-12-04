use clap::{Parser, Subcommand};

mod commands;
mod utils;
mod variables;

#[derive(Parser, Debug)]
#[command(
    author = "Osec",
    version = "1.0.0",
    about = "Create and verify Sui Coin contracts",
    long_about = "Sui Token Generator is a CLI tool that helps you create and verify tokens contracts."
)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    #[command(about = "Creates a new token contract.")]
    Create,
    #[command(about = "Verifies an existing contract from repo or local.")]
    Verify {
        /// Path to the file
        #[arg(short, long)]
        path: Option<String>,

        /// URL to fuzz
        #[arg(short, long)]
        url: Option<String>,
    },
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let cli = Cli::parse();

    match &cli.command {
        Commands::Create => {
            commands::create::create_token().await?;
        }
        Commands::Verify { path, url } => {
            if path.is_none() && url.is_none() {
                eprintln!("Error: Either --path or --url must be provided.");
                std::process::exit(1);
            }

            if let Some(path) = path {
                commands::verify::verify_token_from_path(path).await?;
            }

            if let Some(url) = url {
                commands::verify::verify_token_using_url(url).await?;
            }
        }
    }
    Ok(())
}
