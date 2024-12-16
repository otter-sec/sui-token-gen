use clap::Parser;
use futures::{future, StreamExt};
use std::{
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
    time::Duration,
};
use tarpc::{
    context,
    server::{BaseChannel, Channel},
    serde_transport,
    tokio_serde::formats::Json,
};
use suitokengentest::errors::TokenGenErrors;

use service::{TokenGen, init_tracing};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Flags {
    /// The port to listen on.
    #[arg(short, long, default_value_t = 5000)]
    port: u16,
}

#[derive(Clone)]
struct TokenServer;

fn get_project_root() -> Result<PathBuf, std::io::Error> {
    let current_dir = std::env::current_dir()?;
    let project_root = if current_dir.ends_with("rpc") {
        current_dir.parent().unwrap().to_path_buf()
    } else {
        current_dir
    };
    Ok(project_root)
}

impl TokenGen for TokenServer {
    async fn create(
        self,
        _ctx: context::Context,
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), TokenGenErrors> {
        tracing::info!("Starting token creation with name: {}, symbol: {}", name, symbol);
        tracing::debug!("Token details - decimals: {}, description: {}, frozen: {}, environment: {}",
            decimals, description, frozen, environment);

        let project_root = get_project_root()?;
        tracing::debug!("Project root path: {:?}", project_root);

        tracing::info!("Reading token template...");
        let token_template_path = project_root.join("src/templates/move/token.move.template");
        tracing::debug!("Token template path: {:?}", token_template_path);
        let token_template = std::fs::read_to_string(&token_template_path)
            .map_err(|e| {
                tracing::error!("Failed to read token template at {:?}: {}", token_template_path, e);
                TokenGenErrors::FileIoError(format!("Failed to read token template: {}", e))
            })?;

        tracing::info!("Reading TOML template...");
        let toml_template_path = project_root.join("src/templates/toml/Move.toml.template");
        tracing::debug!("TOML template path: {:?}", toml_template_path);
        let toml_template = std::fs::read_to_string(&toml_template_path)
            .map_err(|e| {
                tracing::error!("Failed to read toml template at {:?}: {}", toml_template_path, e);
                TokenGenErrors::FileIoError(format!("Failed to read toml template: {}", e))
            })?;

        tracing::info!("Generating token content...");
        let token_content = token_template
            .replace("{{name}}", &name)
            .replace("{{symbol}}", &symbol)
            .replace("{{description}}", &description)
            .replace("{{decimals}}", &decimals.to_string())
            .replace("{{is_frozen}}", &frozen.to_string());
        tracing::debug!("Token content generated successfully");

        tracing::info!("Generating TOML content...");
        let toml_content = toml_template
            .replace("{{name}}", &name)
            .replace("{{symbol}}", &symbol)
            .replace("{{environment}}", &environment);
        tracing::debug!("TOML content generated successfully");

        tracing::info!("Creating temporary directory...");
        let temp_dir = tempfile::tempdir()
            .map_err(|e| {
                tracing::error!("Failed to create temporary directory: {}", e);
                TokenGenErrors::FileIoError(format!("Failed to create temporary directory: {}", e))
            })?;
        tracing::debug!("Temporary directory created at: {:?}", temp_dir.path());

        tracing::info!("Token creation completed successfully");
        Ok((
            temp_dir.path().to_string_lossy().to_string(),
            token_content,
            toml_content,
        ))
    }

    async fn verify_url(
        self,
        _ctx: context::Context,
        url: String
    ) -> Result<(), TokenGenErrors> {
        service::utils::verify_helper::verify_token_using_url(&url).await
    }

    async fn verify_content(
        self,
        _ctx: context::Context,
        content: String
    ) -> Result<(), TokenGenErrors> {
        // For empty content, treat it as a ping and return success immediately
        if content.is_empty() {
            tracing::debug!("Received ping request, responding with success");
            return Ok(());
        }

        // For non-empty content, proceed with contract verification
        let temp_dir = tempfile::tempdir()
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to create temporary directory: {}", e)))?;
        let temp_file = temp_dir.path().join("temp.move");
        std::fs::write(&temp_file, &content)
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to write temporary file: {}", e)))?;

        service::utils::verify_helper::verify_contract(temp_dir.path(), temp_dir.path()).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::parse();
    init_tracing("token-gen-server")?;

    let addr = SocketAddr::new(IpAddr::V4(Ipv4Addr::LOCALHOST), flags.port);
    let server = TokenServer {};

    let listener = tarpc::serde_transport::tcp::listen(&addr, Json::default)
        .await?
        .filter_map(|r| {
            let result = r.map_err(|e| {
                tracing::error!("Transport error: {}", e);
                e
            });
            future::ready(result.ok())
        });

    tracing::info!("Starting server on {}", addr);

    listener
        .for_each(|transport| {
            let server = server.clone();
            tokio::spawn(async move {
                tracing::info!("New client connection established");
                BaseChannel::with_defaults(transport)
                    .execute(server.serve())
                    .for_each(|_| {
                        tracing::debug!("Handling RPC request");
                        future::ready(())
                    })
                    .await;
                tracing::info!("Client connection closed");
            });
            future::ready(())
        })
        .await;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
        // Add server tests here
    }
}
