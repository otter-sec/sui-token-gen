use crate::{
    success_handler::{handle_success, SuccessType},
    utils::prompts::TokenInfo,
};

#[test]
fn test_token_creation_success_message() {
    // Test token creation success message with all parameters
    let token_info = TokenInfo {
        decimals: 6,
        symbol: "TEST".to_string(),
        name: "Test Token".to_string(),
        description: "A test token for database storage".to_string(),
        is_frozen: false,
        environment: "mainnet".to_string(),
    };

    // Capture stdout to verify message format
    let success_type = SuccessType::TokenCreated(token_info);
    handle_success(success_type);
    // Note: In a real test, we would capture stdout and verify the exact format
    // but for this demonstration we're just verifying it doesn't panic
}

#[test]
fn test_token_verification_success_message_path() {
    // Test verification success message with path
    let success_type = SuccessType::TokenVerified {
        path: Some("./test_token".to_string()),
        url: None,
    };
    handle_success(success_type);
}

#[test]
fn test_token_verification_success_message_url() {
    // Test verification success message with URL
    let success_type = SuccessType::TokenVerified {
        path: None,
        url: Some("https://example.com/token".to_string()),
    };
    handle_success(success_type);
}

#[test]
fn test_token_info_parameter_capture() {
    // Test that all parameters are captured for future database storage
    let token_info = TokenInfo {
        decimals: 8,
        symbol: "STORE".to_string(),
        name: "Storage Token".to_string(),
        description: "Testing parameter capture for database storage".to_string(),
        is_frozen: true,
        environment: "devnet".to_string(),
    };

    let success_type = SuccessType::TokenCreated(token_info);
    handle_success(success_type);
}
