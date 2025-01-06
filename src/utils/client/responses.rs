use serde::{Deserialize, Serialize};
use thiserror::Error;

/**
 * Enum representing errors that can occur in RPC responses during token generation.
 *
 * This error enum is serialized and deserialized using the `serde` crate, allowing it to
 * be transmitted or stored easily. Each variant provides a specific error message
 * for clearer error handling on the client side.
 */
#[derive(Error, Debug, Deserialize, Serialize)]
pub enum RpcResponseErrors {
    /// Error returned when the contract has been modified from its original state
    #[error("Contract has been modified")]
    ProgramModified,

    /// Error returned when token decimals are invalid or out of range
    #[error("Invalid token decimals")]
    InvalidDecimals,

    /// Error returned when token symbol format is invalid
    #[error("Invalid token symbol")]
    InvalidSymbol,

    /// Error returned when token name format is invalid
    #[error("Invalid token name")]
    InvalidName,

    /// Error returned when token description format is invalid
    #[error("Invalid token description")]
    InvalidDescription,

    /// General error with a detailed message
    #[error("{0}")]
    GeneralError(String),

    /// Error returned for invalid file or directory paths
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Error returned for malformed or inaccessible URLs
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Error returned when a Git operation fails
    #[error("Git operation failed: {0}")]
    GitError(String),

    /// Error returned when a file operation fails
    #[error("File operation failed: {0}")]
    FileIoError(String),

    /// Error returned when verification fails
    #[error("Verification failed: {0}")]
    VerifyResultError(String),
}
