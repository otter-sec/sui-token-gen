use std::net::SocketAddr;
use tarpc::{client, context};
use tarpc::tokio_serde::formats::Json;

use crate::{errors::TokenGenErrors, Result, rpc::TokenGen};

#[derive(Clone)]
pub struct TokenGenClient {
    inner: client::Client<TokenGen>,
}

#[tarpc::client]
impl TokenGen for TokenGenClient {
    async fn verify_url(ctx: context::Context, url: String) -> Result<()> {
        let inner = &self.inner;
        inner.verify_url(ctx, url).await
    }

    async fn verify_content(ctx: context::Context, content: String) -> Result<()> {
        let inner = &self.inner;
        inner.verify_content(ctx, content).await
    }

    async fn create(
        ctx: context::Context,
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> Result<(String, String)> {
        let inner = &self.inner;
        inner.create(ctx, decimals, name, symbol, description, is_frozen).await
    }
}

impl TokenGenClient {
    pub async fn connect(addr: SocketAddr) -> Result<Self> {
        let transport = tarpc::serde_transport::tcp::connect(addr, Json::default)
            .await
            .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?;

        Ok(Self {
            inner: client::new(client::Config::default(), transport).spawn()
        })
    }
}
