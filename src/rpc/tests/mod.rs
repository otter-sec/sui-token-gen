pub mod server;
pub mod client;
pub mod integration;
pub mod error_scenarios;
pub mod fixtures;
pub mod helpers;

use crate::utils::errors::TokenGenErrors;
use tarpc::{
    client::{self, Client, Config},
    context,
    server::Channel,
    tokio_serde::formats::Json,
};
use anyhow::Result;
use std::{net::SocketAddr, time::Instant};
use tokio::time::Duration;

use crate::{TokenGen, TokenServer};

pub async fn setup_test_environment() -> Result<Client<TokenGen>> {
    let listener = tarpc::serde_transport::tcp::listen(&"127.0.0.1:0", Json::default).await?;
    let addr = listener.local_addr();

    let server = TokenServer::new(addr);
    let transport = tarpc::serde_transport::tcp::connect(addr, Json::default).await?;
    Ok(Client::new(Config::default(), transport).spawn())
}

pub async fn cleanup_test_environment() {
    tokio::time::sleep(Duration::from_millis(100)).await;

    // Clean up any temporary files created during tests
    let paths = [
        "test_token.move",
        "Move.toml",
        "test_output.json",
    ];

    for path in paths.iter() {
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.is_file() {
                let _ = std::fs::remove_file(path);
            }
        }
    }
}
