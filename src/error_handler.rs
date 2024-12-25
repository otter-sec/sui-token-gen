use colored::*;

use crate::errors::TokenGenErrors;

/**
 * Centralized error handler for managing errors in a consistent way across the application.
 *
 * This function provides a streamlined mechanism for handling errors by:
 * 1. Logging the error message in a formatted and styled manner for better visibility.
 * 2. Terminating the process with an exit code of `1` to indicate failure.
 *
 * The handler eliminates the need for manually calling `.log()` and `std::process::exit(1)`
 * in multiple places, ensuring a single point of error handling logic.
 *
 * # Arguments
 * - `result`: A `Result` type that contains either a successful value (`Ok(T)`)
 *   or an error (`Err(TokenGenErrors)`).
 *
 * # Returns
 * - The successful value of type `T` if the result is `Ok(T)`.
 *
 * # Behavior on Error
 * - If the result is `Err`, it logs the error message to `stderr` with a red, bold "ERROR: "
 *   prefix and terminates the process with an exit code of `1`.
 */
pub fn handle_error<T>(result: Result<T, TokenGenErrors>) -> T {
    match result {
        // If the result is `Ok`, return the contained value
        Ok(value) => value,

        // If the result is `Err`, log the error and terminate the process
        Err(error) => {
            let error_prefix = "ERROR: ".red().bold(); // Styled prefix for the error message
            eprintln!("{} {}", error_prefix, error); // Log the error message to `stderr`
            std::process::exit(1); // Terminate the process with an exit code of `1`
        }
    }
}
