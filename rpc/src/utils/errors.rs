use serde::{Deserialize, Serialize};
use thiserror::Error;

#[derive(Error, Debug, Deserialize, Serialize)]
pub enum TokenGenErrors {
    #[error("Invalid path: No .move file found")]
    InvalidPathNoMoveFiles,

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

    #[error("The provided URL is not a valid URL.")]
    InvalidUrl,

    #[error("The provided URL is not a valid Git URL.")]
    InvalidGitUrl,

    #[error("Failed to extract repository name.")]
    InvalidRepo,

    #[error("Content mismatch detected")]
    ContractModified,

    #[error("Cloned repo not found")]
    ClonedRepoNotFound,

    #[error("Git operation failed: {0}")]
    GitError(String),

    #[error("File I/O error: {0}")]
    FileIoError(String),

    #[error("{0}")]
    VerifyResultError(String),

    #[error("An error occurred: {0}")]
    GeneralError(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),
}
