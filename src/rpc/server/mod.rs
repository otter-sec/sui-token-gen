use std::{
    future,
    net::SocketAddr,
};
use futures::StreamExt;
use tarpc::{
    server::{self, BaseChannel},
    context,
    tokio_serde::formats::Json,
};

use crate::{TokenGenErrors, Result, rpc::TokenGen};

#[derive(Clone)]
pub struct TokenServer {
    addr: SocketAddr,
}

impl TokenServer {
    pub fn new(addr: SocketAddr) -> Self {
        Self { addr }
    }

    pub async fn serve(self) -> Result<()> {
        let listener = tarpc::serde_transport::tcp::listen(&self.addr, Json::default)
            .await
            .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?;

        listener
            .filter_map(|r| future::ready(r.ok()))
            .map(BaseChannel::with_defaults)
            .for_each(|channel| {
                let server = self.clone();
                tokio::spawn(async move {
                    channel.execute(server).await;
                });
                future::ready(())
            })
            .await;

        Ok(())
    }

    fn serve_fn(
        self,
    ) -> impl Fn(tarpc::context::Context, TokenGenRequest) -> impl future::Future<Output = TokenGenResponse> + Send
           + 'static {
        move |_ctx, req| {
            let server = self.clone();
            async move {
                match req {
                    TokenGenRequest::VerifyUrl { url } => {
                        TokenGenResponse::VerifyUrl(server.verify_url(url).await)
                    }
                    TokenGenRequest::VerifyContent { content } => {
                        TokenGenResponse::VerifyContent(server.verify_content(content).await)
                    }
                    TokenGenRequest::Create {
                        decimals,
                        name,
                        symbol,
                        description,
                        is_frozen,
                    } => {
                        TokenGenResponse::Create(
                            server
                                .create(decimals, name, symbol, description, is_frozen)
                                .await,
                        )
                    }
                }
            }
        }
    }
}

#[tarpc::server]
impl TokenGen for TokenServer {
    async fn verify_url(self, ctx: context::Context, url: String) -> Result<()> {
        if url.starts_with("http") || url.starts_with("https") {
            Ok(())
        } else {
            Err(TokenGenErrors::InvalidUrl(format!("Invalid URL format: {}", url)))
        }
    }

    async fn verify_content(self, ctx: context::Context, content: String) -> Result<()> {
        if content.trim().is_empty() {
            Err(TokenGenErrors::InvalidContent("Empty content".to_string()))
        } else {
            Ok(())
        }
    }

    async fn create(
        self,
        ctx: context::Context,
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> Result<(String, String)> {
        let token_content = format!(
            "// Token contract\n// Name: {}\n// Symbol: {}\n// Decimals: {}\n// Description: {}\n// Frozen: {}",
            name, symbol, decimals, description, is_frozen
        );

        let move_toml = format!(
            "[package]\nname = \"{}\"\nversion = \"0.1.0\"\n[dependencies]",
            name
        );

        Ok((token_content, move_toml))
    }
}
