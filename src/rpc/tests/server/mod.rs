use crate::utils::errors::TokenGenErrors;
use tarpc::{
    client,
    context::{self},
    server::{self, Channel},
    tokio_serde::formats::Json,
};
use anyhow::Result;

use crate::{TokenGen, TokenServer};

pub async fn setup_test_environment() -> Result<client::Client<TokenGen>> {
    let listener = tarpc::serde_transport::tcp::listen(&"127.0.0.1:0", Json::default).await?;
    let addr = listener.local_addr();

    let server = TokenServer::new(addr);
    let transport = tarpc::serde_transport::tcp::connect(addr, Json::default).await?;
    Ok(client::Client::new(client::Config::default(), transport).spawn())
}

pub async fn cleanup_test_environment() {
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
}

#[tokio::test]
async fn test_server_startup() -> Result<()> {
    let client = setup_test_environment().await?;

    let ctx = context::current();
    let result = client
        .verify_url(ctx, "https://github.com/test/repo.git".to_string())
        .await?;

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_server_create_token() -> Result<()> {
    let client = setup_test_environment().await?;

    let ctx = context::current();
    let result = client
        .create(
            ctx,
            8,
            "Test Token".to_string(),
            "TST".to_string(),
            "Test Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await?;

    let (token_content, move_toml, test_token) = result;
    assert!(token_content.contains("Test Token"));
    assert!(move_toml.contains("Test Token"));
    assert!(test_token.contains("Test Token"));

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_server_verify_url() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    assert!(client.verify_url(ctx.clone(), "https://github.com/test/repo.git".to_string()).await.is_ok());
    assert!(client.verify_url(ctx.clone(), "https://github.com/valid/repo.git".to_string()).await.is_ok());

    let result = client
        .verify_url(ctx, "invalid-url".to_string())
        .await;
    assert!(matches!(result, Err(TokenGenErrors::InvalidUrl(_))));

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_server_verify_content() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    let valid_content = r#"
        module test::token {
            use std::string;
            use sui::coin::{Self, TreasuryCap};
            use sui::transfer;
            use sui::tx_context::{Self, TxContext};

            struct TEST has drop {}

            fun init(witness: TEST, ctx: &mut TxContext) {
                let (treasury_cap, metadata) = coin::create_currency(
                    witness,
                    8,
                    b"Test Token",
                    b"TST",
                    b"Test Description",
                    option::none(),
                    ctx
                );
                transfer::public_freeze_object(metadata);
                transfer::public_transfer(treasury_cap, tx_context::sender(ctx));
            }
        }
    "#.to_string();

    assert!(client.verify_content(ctx.clone(), valid_content).await.is_ok());

    let result = client
        .verify_content(ctx, "invalid content".to_string())
        .await;
    assert!(result.is_err());

    cleanup_test_environment().await;
    Ok(())
}
