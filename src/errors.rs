//! Custom error types for the Sui Token Generator
//!
//! # Error Categories
//! - Input validation errors (InvalidInput, InvalidGitUrl)
//! - File system errors (FileIoError)
//! - RPC communication errors (RpcError)
//! - Git operation errors (GitError)
//! - Token verification errors (VerificationError)
//! - User interaction errors (PromptError)
//!
//! Each error type includes context about what went wrong and how to potentially
//! fix the issue.

use inquire::error::InquireError;
use std::io;
use tarpc::client::RpcError;
use thiserror::Error;

/**
 * Enum representing all possible errors that can occur in the token generation process.
 *
 * This error enum is designed to encapsulate and provide meaningful descriptions
 * for different error scenarios. The `thiserror` crate is used to derive the `Error` trait,
 * which simplifies error handling and formatting.
 *
 * Each variant includes a detailed error message to improve debugging and user feedback.
 */
#[derive(Debug, Error)]
pub enum TokenGenErrors {
    /// Error returned when trying to create contract at the current directory.
    #[error("Unable to access the current working directory. Please ensure you have the required permissions.")]
    CurrentDirectoryError,

    /// Error returned when the RPC server is not running or refuses the connection
    #[error("Unable to connect to the RPC service.")]
    FailedToConnectRpc,

    /// Error returned when given RPC url is invalid
    #[error("Invalid RPC url")]
    InvalidRpcUrl,

    /// Error returned when desktop directory not found.
    #[error("Unable to locate the desktop directory")]
    DesktopDirectoryNotFound,

    /// Error returned failed to convert path to string.
    #[error("Unable to convert path to string")]
    PathConversionError,

    /// Error returned when no `.move` file is found in the specified path.
    #[error("No .move files found in the specified path")]
    InvalidPathNoMoveFiles,

    /// Error returned when the provided path is not a valid directory.
    #[error("Specified path is not a directory")]
    InvalidPathNotDirectory,

    /// Error returned when the provided URL is not a valid GitHub or GitLab URL.
    #[error("The provided URL is not a valid Git URL.")]
    InvalidGitUrl,

    /// Error returned when creating a token contract fails, with details provided in the message.
    #[error("Failed to create token contract: {0}")]
    FailedToCreateTokenContract(String),

    /// Error returned for invalid user input, with a specific message.
    #[error("{0}")]
    InvalidInput(String),

    /// Error returned for an invalid path, with a specific message.
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Error returned for an invalid URL, with a specific message.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Error returned for Git-related operations, sourced from the `git2` crate.
    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    /// Error returned for file input/output operations.
    #[error("File I/O error: {0}")]
    FileIoError(io::Error),

    /// Error returned for issues with the Tera templating engine.
    #[error("Tera error: {0}")]
    TeraError(#[from] tera::Error),

    /// Error returned for prompt-related issues from the `inquire` crate.
    #[error(transparent)]
    PromptError(#[from] InquireError),

    /// Error returned for RPC-related issues, sourced from the `tarpc` crate.
    #[error(transparent)]
    RpcError(#[from] RpcError),

    /// Error returned for general verification failures, with details provided in the message.
    #[error("Verification failed: {0}")]
    VerificationError(String),
}

/// Implements conversion from `TokenGenErrors` to `io::Error`.
/// This allows `TokenGenErrors` to be treated as `io::Error` for easier interoperability with standard IO functions.
impl From<TokenGenErrors> for io::Error {
    fn from(err: TokenGenErrors) -> io::Error {
        io::Error::other(err.to_string())
    }
}

/// Implement `From` for `io::Error` to allow seamless conversion to `TokenGenErrors`.
impl From<io::Error> for TokenGenErrors {
    fn from(e: io::Error) -> Self {
        TokenGenErrors::FileIoError(e)
    }
}
