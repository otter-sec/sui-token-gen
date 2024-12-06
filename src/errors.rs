use thiserror::Error;
use std::io;

#[derive(Error, Debug)]
pub enum TokenGenErrors {
    #[error("Failed to create token contract: {0}")]
    FailedToCreateTokenContract(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("Invalid URL: {0}")]
    InvalidUrl(String),

    #[error("Git operation failed: {0}")]
    GitError(#[from] git2::Error),

    #[error("File I/O error: {0}")]
    FileIoError(#[from] io::Error),
  
}