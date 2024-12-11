use thiserror::Error;

use serde::{Deserialize, Serialize};

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum TokenGenErrors {
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
