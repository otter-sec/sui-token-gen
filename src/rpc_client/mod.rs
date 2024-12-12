use std::net::SocketAddr;
use tarpc::{client, context};
use tarpc::tokio_serde::formats::Json;

use crate::{errors::TokenGenErrors, Result};

pub async fn connect_client(addr: SocketAddr) -> Result<client::NewClient<dyn crate::rpc::TokenGen>> {
    let transport = tarpc::serde_transport::tcp::connect(addr, Json::default)
        .await
        .map_err(|e| TokenGenErrors::RpcError(e.to_string()))?;

    Ok(client::new(client::Config::default(), transport).spawn())
}
