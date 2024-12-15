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
    GitError(String),

    #[error("File I/O error: {0}")]
    FileIoError(#[serde(skip)] io::Error),

    #[error("Template error: {0}")]
    TeraError(String),

    #[error("Prompt error: {0}")]
    PromptError(String),

    #[error("RPC error: {0}")]
    RpcError(String),

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

// Implement From for tera::Error
impl From<tera::Error> for TokenGenErrors {
    fn from(e: tera::Error) -> Self {
        TokenGenErrors::TeraError(e.to_string())
    }
}

// Implement From for InquireError
impl From<InquireError> for TokenGenErrors {
    fn from(e: InquireError) -> Self {
        TokenGenErrors::PromptError(e.to_string())
    }
}

// Implement From for RpcError
impl From<RpcError> for TokenGenErrors {
    fn from(e: RpcError) -> Self {
        TokenGenErrors::RpcError(e.to_string())
    }
}

// Implement From for git2::Error
impl From<git2::Error> for TokenGenErrors {
    fn from(e: git2::Error) -> Self {
        TokenGenErrors::GitError(e.message().to_string())
    }
}

// Implement From<RpcResponseErrors> for TokenGenErrors
impl From<RpcResponseErrors> for TokenGenErrors {
    fn from(e: RpcResponseErrors) -> Self {
        match e {
            RpcResponseErrors::FailedToCreateTokenContract(msg) => TokenGenErrors::FailedToCreateTokenContract(msg),
            RpcResponseErrors::InvalidInput(msg) => TokenGenErrors::InvalidInput(msg),
            RpcResponseErrors::InvalidPath(msg) => TokenGenErrors::InvalidPath(msg),
            RpcResponseErrors::InvalidUrl(msg) => TokenGenErrors::InvalidUrl(msg),
            RpcResponseErrors::GitError(msg) => TokenGenErrors::GitError(msg),
            RpcResponseErrors::FileIoError(msg) => TokenGenErrors::FileIoError(io::Error::new(io::ErrorKind::Other, msg)),
            RpcResponseErrors::TeraError(msg) => TokenGenErrors::TeraError(msg),
            RpcResponseErrors::PromptError(msg) => TokenGenErrors::PromptError(msg),
            RpcResponseErrors::RpcError(msg) => TokenGenErrors::RpcError(msg),
            RpcResponseErrors::VerificationError(msg) => TokenGenErrors::VerificationError(msg),
            RpcResponseErrors::TemplateNotFound(msg) => TokenGenErrors::TemplateNotFound(msg),
        }
    }
}

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum RpcResponseErrors {
    #[error("Failed to create token contract: {0}")]
    FailedToCreateTokenContract(String),

    #[error("{0}")]
    InvalidInput(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("File I/O error: {0}")]
    FileIoError(String),

    #[error("Template error: {0}")]
    TeraError(String),

    #[error("Prompt error: {0}")]
    PromptError(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Verification failed: {0}")]
    VerificationError(String),

    #[error("Template not found: {0}")]
    TemplateNotFound(String),
}
