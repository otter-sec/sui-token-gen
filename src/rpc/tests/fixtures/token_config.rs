use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenConfig {
    pub decimals: u8,
    pub name: String,
    pub symbol: String,
    pub description: String,
    pub is_frozen: bool,
    pub environment: String,
}

pub const TEST_TOKEN_CONFIG: TokenConfig = TokenConfig {
    decimals: 8,
    name: String::from("Test Token"),
    symbol: String::from("TST"),
    description: String::from("Test Description"),
    is_frozen: false,
    environment: String::from("devnet"),
};

pub const INVALID_TOKEN_CONFIG: TokenConfig = TokenConfig {
    decimals: 255,
    name: String::from(""),
    symbol: String::from("TOOLONGSYMBOL"),
    description: String::from(""),
    is_frozen: false,
    environment: String::from("invalid_network"),
};
