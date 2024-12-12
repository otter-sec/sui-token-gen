use crate::utils::errors::TokenGenErrors;
use crate::rpc::tests::fixtures::{
    token_config::{TokenConfig, TEST_TOKEN_CONFIG, INVALID_TOKEN_CONFIG},
    network_responses::{MOCK_SUCCESS_RESPONSE, MOCK_ERROR_RESPONSE},
};
use tarpc::{
    client,
    context::{self},
    server::{self, Channel},
    tokio_serde::formats::Json,
};
use anyhow::Result;
use std::path::PathBuf;

use crate::{TokenGen, TokenServer};

/// Creates a test token with the provided configuration
pub async fn create_test_token(config: Option<TokenConfig>) -> Result<String> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    let config = config.unwrap_or(TEST_TOKEN_CONFIG.clone());

    let (token_content, _, _) = client
        .create(
            ctx,
            config.decimals,
            config.name,
            config.symbol,
            config.description,
            config.is_frozen,
            config.environment,
        )
        .await?;

    Ok(token_content)
}

/// Verifies a test token's content
pub async fn verify_test_token(token_content: String) -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    client.verify_content(ctx, token_content).await?;
    Ok(())
}

/// Helper to create and verify a token in one step
pub async fn create_and_verify_token(config: Option<TokenConfig>) -> Result<()> {
    let token_content = create_test_token(config).await?;
    verify_test_token(token_content).await
}

/// Helper to test invalid token creation
pub async fn test_invalid_token_creation() -> Result<()> {
    let client = setup_test_environment().await?;
    let ctx = context::current();

    let result = client
        .create(
            ctx,
            INVALID_TOKEN_CONFIG.decimals,
            INVALID_TOKEN_CONFIG.name,
            INVALID_TOKEN_CONFIG.symbol,
            INVALID_TOKEN_CONFIG.description,
            INVALID_TOKEN_CONFIG.is_frozen,
            INVALID_TOKEN_CONFIG.environment,
        )
        .await;

    assert!(result.is_err());
    Ok(())
}

/// Helper to clean up test artifacts
pub async fn cleanup_test_artifacts() -> Result<()> {
    // Clean up any temporary files created during tests
    let paths = [
        "test_token.move",
        "Move.toml",
        "test_output.json",
    ];

    for path in paths.iter() {
        if let Ok(metadata) = std::fs::metadata(path) {
            if metadata.is_file() {
                std::fs::remove_file(path)?;
            }
        }
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_create_test_token() -> Result<()> {
        let token_content = create_test_token(None).await?;
        assert!(!token_content.is_empty());
        Ok(())
    }

    #[tokio::test]
    async fn test_verify_test_token() -> Result<()> {
        let token_content = create_test_token(None).await?;
        verify_test_token(token_content).await
    }

    #[tokio::test]
    async fn test_create_and_verify_token() -> Result<()> {
        create_and_verify_token(None).await
    }

    #[tokio::test]
    async fn test_invalid_token_creation() -> Result<()> {
        super::test_invalid_token_creation().await
    }
}
