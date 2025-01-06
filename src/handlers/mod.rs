//! Response Handler Implementations
//!
//! This module contains handlers for managing command responses:
//! - Error handling (`error`)
//! - Success handling (`success`)
//!
//! Each submodule provides specific response handling functionality.

mod error;
mod success;

pub use error::handle_error;
pub use success::{handle_success, SuccessType};
