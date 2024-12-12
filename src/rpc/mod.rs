pub mod server;
pub mod client;

use tarpc::{
    context,
    server::{Channel, BaseChannel},
};

#[tarpc::service]
pub trait TokenGen {
    /// Verify a token contract from a URL
    async fn verify_url(ctx: context::Context, url: String) -> crate::Result<()>;
    /// Verify token contract content
    async fn verify_content(ctx: context::Context, content: String) -> crate::Result<()>;
    /// Create a new token contract
    async fn create(
        ctx: context::Context,
        decimals: u8,
        name: String,
        symbol: String,
        description: String,
        is_frozen: bool,
    ) -> crate::Result<(String, String)>;
}
