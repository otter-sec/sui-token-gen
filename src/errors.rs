use inquire::error::InquireError;
use std::io;
use tarpc::client::RpcError;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum TokenGenErrors {
    #[error("Failed to create token contract: {0}")]
    FailedToCreateTokenContract(String),

    #[error("{0}")]
    InvalidInput(String),

    // Path-related errors
    #[error("Invalid path: directory not found")]
    InvalidPathNotFound,

    #[error("Invalid path: not a directory")]
    InvalidPathNotDirectory,

    #[error("Invalid path: no Move files in sources")]
    InvalidPathNoMoveFiles,

    // URL-related errors
    #[error("Invalid URL: not a GitHub repository")]
    InvalidUrlNotGithub,

    #[error("Invalid URL: repository not found")]
    InvalidUrlRepoNotFound,

    #[error("Invalid URL: malformed URL")]
    InvalidUrlMalformed,

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("File I/O error: {0}")]
    FileIoError(io::Error),

    #[error("Tera error: {0}")]
    TeraError(#[from] tera::Error),

    #[error(transparent)]
    PromptError(#[from] InquireError),

    #[error(transparent)]
    RpcError(#[from] RpcError),

    #[error("Verification failed: {0}")]
    VerificationError(String),
}

// Implement From for io::Error separately since we can't use #[from]
impl From<io::Error> for TokenGenErrors {
    fn from(e: io::Error) -> Self {
        TokenGenErrors::FileIoError(e)
    }
}

use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum RpcResponseErrors {
    #[error("Given contract is modified")]
    ProgramModified,

    #[error("Invalid decimals provided")]
    InvalidDecimals,

    #[error("Invalid symbol provided")]
    InvalidSymbol,

    #[error("Invalid name provided")]
    InvalidName,

    #[error("Invalid description provided")]
    InvalidDescription,

    #[error("An error occurred: {0}")]
    GeneralError(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("File I/O error: {0}")]
    FileIoError(String),

    #[error("{0}")]
    VerifyResultError(String),
}
