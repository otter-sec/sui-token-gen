pub mod server;
pub mod client;

use tarpc::context;

#[tarpc::service]
pub trait TokenGen {
    /// Verify a token contract from a URL
    async fn verify_url(url: String) -> crate::Result<()>;
    /// Verify token contract content
    async fn verify_content(content: String) -> crate::Result<()>;
    /// Create a new token contract
    async fn create(
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> crate::Result<(String, String)>;
}
