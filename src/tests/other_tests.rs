use tarpc::context;

use super::common::test_utils::setup_test_client;
use crate::{constants::ADDRESS, Result};

// Test case to verify token creation works across different environments (devnet, testnet, mainnet)
// It attempts to create a token in each environment and asserts that the token creation is successful for all environments.
#[tokio::test]
async fn environment_specific_token_creation() -> Result<()> {
    let client = setup_test_client(ADDRESS).await?;

    // Loop through different environments and test token creation
    for env in ["devnet", "testnet", "mainnet"] {
        let result = client
            .create(
                context::current(),
                6, // 6 decimal places
                "TestToken".to_string(),
                "TEST".to_string(),
                "Test Description".to_string(),
                false,
                env.to_string(), // Environment for deployment
            )
            .await;

        // Assert that token creation succeeds for each environment
        assert!(
            result.is_ok(),
            "Token creation failed for environment: {}",
            env
        );
    }
    Ok(())
}
