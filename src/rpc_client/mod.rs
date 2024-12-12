use std::net::SocketAddr;
use tarpc::{client, context};
use tarpc::tokio_serde::formats::Json;

use crate::{errors::TokenGenErrors, Result, rpc::TokenGen};

pub type TokenGenClient = client::Client<client::Channel<tarpc::serde_transport::tcp::Transport<Json>, Json>>;

#[tarpc::client]
impl TokenGen for TokenGenClient {}

pub async fn connect_client(addr: SocketAddr) -> Result<TokenGenClient> {
    let transport = tarpc::serde_transport::tcp::connect(addr, Json::default)
        .await
        .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?;

    Ok(TokenGenClient::new(client::Config::default(), transport).spawn())
}
