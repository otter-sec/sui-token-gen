use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
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
    FileIoError(#[from] io::Error),

    #[error("Tera error: {0}")]
    TeraError(#[from] tera::Error),

    #[error(transparent)]
    PromptError(#[from] inquire::error::InquireError),

    #[error(transparent)]
    RpcError(#[from] tarpc::client::RpcError),

    #[error("Verification failed: {0}")]
    VerificationError(String),
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