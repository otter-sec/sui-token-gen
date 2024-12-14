use std::io;
use thiserror::Error;
use colored::*;

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
    GitError(git2::Error),

    #[error("File I/O error: {0}")]
    FileIoError(io::Error),

    #[error("Tera error: {0}")]
    TeraError(tera::Error),

    #[error(transparent)]
    PromptError(inquire::error::InquireError),

    #[error(transparent)]
    RpcError(tarpc::client::RpcError),

    #[error("Verification failed: {0}")]
    VerificationError(String),
}

impl TokenGenErrors {
    pub fn log(&self) {
        println!("{} {}", "ERROR: ".red(), self.to_string());
    }
}

impl From<io::Error> for TokenGenErrors {
    fn from(e: io::Error) -> Self {
        let error = Self::FileIoError(e);
        error.log();
        error
    }
}

impl From<git2::Error> for TokenGenErrors {
    fn from(e: git2::Error) -> Self {
        let error = Self::GitError(e);
        error.log();
        error
    }
}

impl From<tera::Error> for TokenGenErrors {
    fn from(e: tera::Error) -> Self {
        let error = Self::TeraError(e);
        error.log();
        error
    }
}

impl From<inquire::error::InquireError> for TokenGenErrors {
    fn from(e: inquire::error::InquireError) -> Self {
        let error = Self::PromptError(e);
        error.log();
        error
    }
}

impl From<tarpc::client::RpcError> for TokenGenErrors {
    fn from(e: tarpc::client::RpcError) -> Self {
        let error = Self::RpcError(e);
        error.log();
        error
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
