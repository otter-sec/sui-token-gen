use std::{
    fs,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    path::PathBuf,
};

use anyhow::Result;
use clap::Parser;
use futures::{future, StreamExt};
use service::{
    init_tracing,
    utils::verify_helper,
    TokenGen,
};
use suitokengentest::errors::TokenGenErrors;
use tarpc::{
    server::{BaseChannel, Channel},
    tokio_serde::formats::Json,
};
use tempfile::tempdir;

#[derive(Parser)]
#[clap(name = "server")]
struct Flags {
    /// The port the server should listen on.
    #[clap(long, default_value = "5000")]
    port: u16,
}

#[derive(Clone)]
struct TokenServer;

fn get_project_root() -> Result<PathBuf, TokenGenErrors> {
    let current_dir = std::env::current_dir()
        .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to get current directory: {}", e)))?;
    let project_root = current_dir
        .parent()
        .and_then(|p| p.parent())
        .ok_or_else(|| TokenGenErrors::FileIoError("Failed to find project root".into()))?
        .to_path_buf();

    Ok(project_root)
}

impl TokenGen for TokenServer {
    async fn create(
        self,
        _context: ::tarpc::context::Context,
        name: String,
        symbol: String,
        decimals: u8,
        description: String,
        frozen: bool,
        environment: String,
    ) -> Result<(String, String, String), TokenGenErrors> {
        let project_root = get_project_root()?;

        let token_template = fs::read_to_string(
            project_root.join("src/templates/move/token.move.template")
        ).map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read token template: {}", e)))?;

        let toml_template = fs::read_to_string(
            project_root.join("src/templates/toml/Move.toml.template")
        ).map_err(|e| TokenGenErrors::FileIoError(format!("Failed to read toml template: {}", e)))?;

        let token_content = token_template
            .replace("{{name}}", &name)
            .replace("{{symbol}}", &symbol)
            .replace("{{description}}", &description)
            .replace("{{decimals}}", &decimals.to_string())
            .replace("{{is_frozen}}", &frozen.to_string());

        let toml_content = toml_template
            .replace("{{name}}", &name)
            .replace("{{symbol}}", &symbol)
            .replace("{{environment}}", &environment);

        let temp_dir = tempdir()
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to create temporary directory: {}", e)))?;

        Ok((
            temp_dir.path().to_string_lossy().to_string(),
            token_content,
            toml_content,
        ))
    }

    async fn verify_url(
        self,
        _context: ::tarpc::context::Context,
        url: String
    ) -> Result<(), TokenGenErrors> {
        verify_helper::verify_token_using_url(&url).await
    }

    async fn verify_content(
        self,
        _context: ::tarpc::context::Context,
        content: String
    ) -> Result<(), TokenGenErrors> {
        let temp_dir = tempdir()
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to create temporary directory: {}", e)))?;
        let temp_file = temp_dir.path().join("temp.move");
        fs::write(&temp_file, &content)
            .map_err(|e| TokenGenErrors::FileIoError(format!("Failed to write temporary file: {}", e)))?;

        verify_helper::verify_contract(temp_dir.path(), temp_dir.path()).await
    }
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let flags = Flags::parse();
    let port = flags.port;

    let addr = format!("127.0.0.1:{}", port).parse()?;
    let server = TokenServer;

    let mut listener = tarpc::serde_transport::tcp::listen(&addr, Json::default).await?;
    listener.config_mut().max_frame_length(usize::MAX);

    println!("Server listening on {}", addr);

    let server_fut = listener
        .filter_map(|r| futures::future::ready(r.ok()))
        .map(tarpc::server::BaseChannel::with_defaults)
        .map(|channel| {
            let server = server.clone();
            channel.execute(server.serve())
        })
        .buffer_unordered(10)
        .for_each(|_| async {});

    tokio::select! {
        _ = server_fut => {},
        _ = tokio::signal::ctrl_c() => {
            println!("Received Ctrl+C, shutting down...");
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_server() {
        // Test implementation will be added later
    }
}
