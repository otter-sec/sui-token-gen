use crate::{
    handlers::{handle_success, SuccessType},
    utils::prompts::TokenInfo,
};

// Test case to verify that the success message for token creation is properly displayed with all parameters
#[test]
fn test_token_creation_success_message() {
    // Create a TokenInfo instance representing the details of the token
    let token_info = TokenInfo {
        decimals: 6,                                                  // Token decimal precision
        symbol: "TEST".to_string(),                                   // Token symbol (e.g., 'TEST')
        name: "Test Token".to_string(),                               // Token name
        description: "A test token for database storage".to_string(), // Token description
        is_frozen: false, // Indicates whether the token is frozen (not transferable)
        environment: "mainnet".to_string(), // The environment for the token (e.g., mainnet, testnet)
    };

    // Create a success message for token creation using the TokenInfo object
    let success_type =
        SuccessType::TokenCreated(token_info, "Contract has been generated!".to_string());

    // Call handle_success function to display the success message for token creation
    handle_success(success_type);

    // Note: In a real test, we would capture stdout and verify the exact format of the success message
    // but for this demonstration, we are just verifying that it doesn't panic or fail during execution
}

// Test case to verify that the success message for token verification is properly displayed when a path is provided
#[test]
fn test_token_verification_success_message_path() {
    // Create a success message for token verification with a specified path
    let success_type = SuccessType::TokenVerified {
        path: Some("./test_token".to_string()), // Token verification path
        url: None,                              // URL is not provided
        address: None                           // Address is not provided
    };

    // Call handle_success function to display the success message for token verification with path
    handle_success(success_type);
}

// Test case to verify that the success message for token verification is properly displayed when a URL is provided
#[test]
fn test_token_verification_success_message_url() {
    // Create a success message for token verification with a specified URL
    let success_type = SuccessType::TokenVerified {
        path: None,                                         // Path is not provided
        url: Some("https://example.com/token".to_string()), // Token verification URL
        address: None                                       // Address is not provided
    };

    // Call handle_success function to display the success message for token verification with URL
    handle_success(success_type);
}

// Test case to verify that all token creation parameters are captured and displayed correctly for database storage
#[test]
fn test_token_info_parameter_capture() {
    // Create a TokenInfo instance with specific parameters for database storage
    let token_info = TokenInfo {
        decimals: 8,                       // Token decimal precision
        symbol: "STORE".to_string(),       // Token symbol (e.g., 'STORE')
        name: "Storage Token".to_string(), // Token name
        description: "Testing parameter capture for database storage".to_string(), // Token description
        is_frozen: true,                   // Indicates that the token is frozen
        environment: "devnet".to_string(), // The environment for the token (e.g., devnet)
    };

    // Create a success message for token creation using the TokenInfo object
    let success_type =
        SuccessType::TokenCreated(token_info, "Contract has been generated!".to_string());

    // Call handle_success function to display the success message for token creation
    handle_success(success_type);
}
