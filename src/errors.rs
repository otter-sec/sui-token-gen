use std::io;
use thiserror::Error;
use serde::{Serialize, Deserialize};

#[derive(Error, Debug, Serialize, Deserialize)]
pub enum TokenGenErrors {
    #[error("Failed to create token contract: {0}")]
    FailedToCreateTokenContract(String),

    #[error("Contract creation error: {0}")]
    ContractCreationError(String),

    #[error("{0}")]
    InvalidInput(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Invalid content: {0}")]
    InvalidContent(String),

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("File I/O error: {0}")]
    FileIoError(String),

    #[error("Tera error: {0}")]
    TeraError(String),

    #[error("Prompt error: {0}")]
    PromptError(String),

    #[error("RPC error: {0}")]
    RpcError(String),

    #[error("Verification failed: {0}")]
    VerificationError(String),

    #[error("General error: {0}")]
    GeneralError(String),
}

impl From<git2::Error> for TokenGenErrors {
    fn from(err: git2::Error) -> Self {
        TokenGenErrors::GitError(err.to_string())
    }
}

impl From<io::Error> for TokenGenErrors {
    fn from(err: io::Error) -> Self {
        TokenGenErrors::FileIoError(err.to_string())
    }
}

impl From<tera::Error> for TokenGenErrors {
    fn from(err: tera::Error) -> Self {
        TokenGenErrors::TeraError(err.to_string())
    }
}

impl From<inquire::error::InquireError> for TokenGenErrors {
    fn from(err: inquire::error::InquireError) -> Self {
        TokenGenErrors::PromptError(err.to_string())
    }
}

impl From<tarpc::client::RpcError> for TokenGenErrors {
    fn from(err: tarpc::client::RpcError) -> Self {
        TokenGenErrors::RpcError(err.to_string())
    }
}
