pub mod commands;
pub mod error_handler;
pub mod errors;
pub mod rpc_client;
pub mod success_handler;
pub mod tests;
pub mod utils;
pub mod variables;

pub use error_handler::*;
pub use errors::*;
pub use success_handler::*;
pub use utils::*;
pub use variables::*;

pub type Result<T> = std::result::Result<T, errors::TokenGenErrors>;
