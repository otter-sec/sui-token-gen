use colored::*;

use crate::errors::TokenGenErrors;

// Centralized error handler that handles both logging and process termination.
// This eliminates the need for manual .log() and std::process::exit(1) calls
pub fn handle_error<T>(result: Result<T, TokenGenErrors>) -> T {
    match result {
        Ok(value) => value,
        Err(error) => {
            let error_prefix = "ERROR: ".red().bold();
            eprintln!("{} {}", error_prefix, error);
            std::process::exit(1);
        }
    }
}
