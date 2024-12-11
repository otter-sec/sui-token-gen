use anyhow::Result;
use clap::Parser;
use std::net::SocketAddr;
use tarpc::{client, context, tokio_serde::formats::Json};
use thiserror::Error;

mod utils;

use utils::errors::TokenGenErrors;

#[tarpc::service]
pub trait TokenGen {
    async fn create(
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> Result<(String, String), TokenGenErrors>;

    async fn verify_url(url: String) -> Result<(), TokenGenErrors>;
    async fn verify_content(content: String) -> Result<(), TokenGenErrors>;
}

#[derive(Debug, Error)]
enum ClientError {
    #[error("Missing required parameter: {0}")]
    MissingParameter(String),

    #[error("Unknown command: {0}")]
    UnknownCommand(String),

    #[error("Failed to communicate with server: {0}")]
    ServerError(#[from] tarpc::client::RpcError),

    #[error("Invalid inputs: {0}")]
    InvalidInputs(String),
}

impl From<TokenGenErrors> for ClientError {
    fn from(err: TokenGenErrors) -> Self {
        ClientError::InvalidInputs(err.to_string())
    }
}

#[derive(Parser)]
struct Flags {
    #[clap(long)]
    server_addr: SocketAddr,
    #[clap(long)]
    command: Option<String>,
    #[clap(long)]
    name: Option<String>,
    #[clap(long)]
    symbol: Option<String>,
    #[clap(long)]
    description: Option<String>,
    #[clap(long)]
    decimals: Option<u8>,
    #[clap(long)]
    is_frozen: Option<bool>,
    #[clap(long)]
    url: Option<String>,
    #[clap(long)]
    content: Option<String>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let flags = Flags::parse();

    let mut transport = tarpc::serde_transport::tcp::connect(flags.server_addr, Json::default);
    transport.config_mut().max_frame_length(usize::MAX);

    let client: TokenGenClient =
        TokenGenClient::new(client::Config::default(), transport.await?).spawn();

    if let Err(err) = handle_command(flags, client).await {
        eprintln!("Error: {}", err);
    }
    Ok(())
}

fn require_param<T>(param: Option<T>, param_name: &str, command: &str) -> Result<T, ClientError> {
    param.ok_or_else(|| {
        ClientError::MissingParameter(format!(
            "{} is required for '{}' command",
            param_name, command
        ))
    })
}

async fn handle_command(flags: Flags, client: TokenGenClient) -> Result<(), ClientError> {
    match flags.command.as_deref() {
        Some("verify_url") => {
            let url = require_param(flags.url, "url", "verify")?;
            let result = client.verify_url(context::current(), url).await?;
            println!("Verification Result: {:?}", result);
        }
        Some("verify_content") => {
            let content = require_param(flags.content, "content", "verify")?;
            let result = client.verify_content(context::current(), content).await?;
            println!("Verification Result: {:?}", result);
        }
        Some("create") => {
            let name = require_param(flags.name, "name", "create")?;
            let symbol = require_param(flags.symbol, "symbol", "create")?;
            let description = require_param(flags.description, "description", "create")?;
            let decimals = require_param(flags.decimals, "decimals", "create")?;
            let is_frozen = require_param(flags.is_frozen, "is_frozen", "create")?;

            let result = client
                .create(
                    context::current(),
                    decimals,
                    name,
                    symbol,
                    description,
                    is_frozen,
                )
                .await
                .map_err(|e| ClientError::ServerError(e))?;

            let (token_content, move_toml_content) = result?;
            println!("Token Content: {}", token_content);
            println!("Move.toml Content: {}", move_toml_content);
        }
        Some(cmd) => return Err(ClientError::InvalidInputs(cmd.into())),
        None => {
            return Err(ClientError::MissingParameter(
                "command is required (either 'create' or 'verify')".into(),
            ));
        }
    }

    Ok(())
}
