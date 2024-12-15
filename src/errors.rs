use std::io;
use inquire::error::InquireError;
use tarpc::client::RpcError;
use thiserror::Error;
use serde::{Deserialize, Serialize};

#[derive(Debug, Error, Serialize, Deserialize)]
pub enum TokenGenErrors {
    #[error("Failed to create token contract: {0}")]
    FailedToCreateTokenContract(String),

    #[error("{0}")]
    InvalidInput(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

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

    #[error("Template not found: {0}")]
    TemplateNotFound(String),
}

// Implement From for io::Error separately since we can't use #[from]
impl From<io::Error> for TokenGenErrors {
    fn from(e: io::Error) -> Self {
        TokenGenErrors::FileIoError(e)
    }
}

// Implement From<RpcResponseErrors> for TokenGenErrors
impl From<RpcResponseErrors> for TokenGenErrors {
    fn from(e: RpcResponseErrors) -> Self {
        match e {
            RpcResponseErrors::ProgramModified => TokenGenErrors::VerificationError("Program modified".to_string()),
            RpcResponseErrors::InvalidDecimals => TokenGenErrors::InvalidInput("Invalid decimals".to_string()),
            RpcResponseErrors::InvalidSymbol => TokenGenErrors::InvalidInput("Invalid symbol".to_string()),
            RpcResponseErrors::InvalidName => TokenGenErrors::InvalidInput("Invalid name".to_string()),
            RpcResponseErrors::InvalidDescription => TokenGenErrors::InvalidInput("Invalid description".to_string()),
            RpcResponseErrors::GeneralError(msg) => TokenGenErrors::InvalidInput(msg),
            RpcResponseErrors::InvalidPath(msg) => TokenGenErrors::InvalidPath(msg),
            RpcResponseErrors::InvalidUrl(msg) => TokenGenErrors::InvalidUrl(msg),
            RpcResponseErrors::GitError(msg) => TokenGenErrors::GitError(git2::Error::from_str(&msg)),
            RpcResponseErrors::FileIoError(msg) => TokenGenErrors::FileIoError(io::Error::new(io::ErrorKind::Other, msg)),
            RpcResponseErrors::VerifyResultError(msg) => TokenGenErrors::VerificationError(msg),
            RpcResponseErrors::TemplateNotFound(msg) => TokenGenErrors::TemplateNotFound(msg),
        }
    }
}

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

    #[error("Template not found: {0}")]
    TemplateNotFound(String),
}
