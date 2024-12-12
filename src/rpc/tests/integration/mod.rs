use super::{setup_test_environment, cleanup_test_environment};
use crate::utils::errors::TokenGenErrors;
use tarpc::context;
use anyhow::Result;
use futures::future;
use std::time::{Duration, Instant};

#[tokio::test]
async fn test_token_creation_flow() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    // Create token
    let (token_content, move_toml, test_token) = client
        .create(
            ctx.clone(),
            8,
            "Integration Test Token".to_string(),
            "ITT".to_string(),
            "Integration Test Description".to_string(),
            false,
            "devnet".to_string(),
        )
        .await?;

    // Verify token content
    assert!(client.verify_content(ctx.clone(), token_content.clone()).await.is_ok());

    // Verify TOML structure
    assert!(move_toml.contains("[package]"));
    assert!(move_toml.contains("name = \"Integration Test Token\""));
    assert!(move_toml.contains("[dependencies]"));
    assert!(move_toml.contains("Sui = "));

    // Verify test token
    assert!(test_token.contains("Integration Test Token"));
    assert!(test_token.contains("#[test]"));

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_concurrent_requests() -> Result<()> {
    let client = setup_test_environment().await?;

    let futures: Vec<_> = (0..5)
        .map(|i| {
            let client = client.clone();
            tokio::spawn(async move {
                client
                    .create(
                        context::current(),
                        8,
                        format!("Token {}", i),
                        format!("T{}", i),
                        "Test Description".to_string(),
                        false,
                        "devnet".to_string(),
                    )
                    .await
            })
        })
        .collect();

    let results = future::join_all(futures).await;
    for result in results {
        assert!(result?.is_ok());
    }

    cleanup_test_environment().await;
    Ok(())
}

#[tokio::test]
async fn test_token_verification_flow() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    // Test URL verification flow
    assert!(client
        .verify_url(ctx.clone(), "https://github.com/test/repo.git".to_string())
        .await
        .is_ok());

    // Test content verification flow with valid Move code
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

    cleanup_test_environment().await;
    Ok(())
}
