use serde::{Deserialize, Serialize};
use thiserror::Error;

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

    #[error("Invalid path: directory not found")]
    InvalidPathNotFound,

    #[error("Invalid path: not a directory")]
    InvalidPathNotDirectory,

    #[error("Invalid path: no Move files in sources")]
    InvalidPathNoMoveFiles,

    #[error("Invalid URL: not a GitHub repository")]
    InvalidUrlNotGithub,

    #[error("Invalid URL: repository not found")]
    InvalidUrlRepoNotFound,

    #[error("Invalid URL: malformed URL")]
    InvalidUrlMalformed,

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("File I/O error: {0}")]
    FileIoError(#[from] std::io::Error),

    #[error("{0}")]
    VerifyResultError(String),
}
