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
    /// Error returned when no `.move` file is found in the specified path.
    #[error("Invalid path: No .move file found")]
    InvalidPathNoMoveFiles,

    /// Error returned when the provided path is not a valid directory.
    #[error("Invalid path: Directory not found")]
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

// Implement `From` for `io::Error` to allow seamless conversion to `TokenGenErrors`.
impl From<io::Error> for TokenGenErrors {
    fn from(e: io::Error) -> Self {
        TokenGenErrors::FileIoError(e)
    }
}

use serde::{Deserialize, Serialize};

/**
 * Enum representing errors that can occur in RPC responses during token generation.
 *
 * This error enum is serialized and deserialized using the `serde` crate, allowing it to
 * be transmitted or stored easily. Each variant provides a specific error message
 * for clearer error handling on the client side.
 */
#[derive(Error, Debug, Deserialize, Serialize)]
pub enum RpcResponseErrors {
    /// Error returned when the provided contract has been modified.
    #[error("Given contract is modified")]
    ProgramModified,

    /// Error returned for invalid token decimals.
    #[error("Invalid decimals provided")]
    InvalidDecimals,

    /// Error returned for invalid token symbols.
    #[error("Invalid symbol provided")]
    InvalidSymbol,

    /// Error returned for invalid token names.
    #[error("Invalid name provided")]
    InvalidName,

    /// Error returned for invalid token descriptions.
    #[error("Invalid description provided")]
    InvalidDescription,

    /// General error with a detailed message.
    #[error("An error occurred: {0}")]
    GeneralError(String),

    /// Error returned for invalid paths, with a specific message.
    #[error("Invalid path: {0}")]
    InvalidPath(String),

    /// Error returned for invalid URLs, with a specific message.
    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    /// Error returned for Git-related operations, with details provided in the message.
    #[error("Git operation failed: {0}")]
    GitError(String),

    /// Error returned for file input/output operations, with details provided in the message.
    #[error("File I/O error: {0}")]
    FileIoError(String),

    /// Error returned for verification issues, with a specific message.
    #[error("{0}")]
    VerifyResultError(String),
}
