use std::{fs, path::Path};

/// A structure to manage atomic file operations, ensuring partial changes
/// are cleaned up if the operation is not explicitly committed.
///
/// This is particularly useful for file operations that require temporary
/// directories or intermediate states. If the operation is not marked as
/// completed using `commit`, the temporary files and directories will
/// be automatically cleaned up when the struct goes out of scope.
pub struct AtomicFileOperation {
    /// The base path for the operation, typically a directory or file path.
    base_path: String,
    /// Indicates whether the operation has been successfully completed.
    completed: bool,
}

impl AtomicFileOperation {
    /**
     * Creates a new `AtomicFileOperation` instance.
     *
     * # Arguments
     * - `base_path`: The path to the directory or file to manage atomically.
     *
     * # Returns
     * - A new instance of `AtomicFileOperation`.
     */
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            completed: false,
        }
    }

    /**
     * Marks the operation as completed, preventing automatic cleanup during
     * the `Drop` phase.
     *
     * Call this method after successfully completing the operation to retain
     * the files or directories created during the process.
     */
    pub fn commit(&mut self) {
        self.completed = true;
    }
}

impl Drop for AtomicFileOperation {
    /**
     * Automatically cleans up the directory specified by `base_path`
     * if the operation is not marked as completed.
     *
     * This ensures that temporary files and directories are removed
     * to prevent leaving unwanted data behind in case of an error
     * or early termination of the process.
     *
     * # Cleanup Behavior
     * - If `commit` has not been called, and the path exists, the directory
     *   (and all its contents) will be removed using `fs::remove_dir_all`.
     * - Any errors during cleanup are ignored.
     */
    fn drop(&mut self) {
        if !self.completed && Path::new(&self.base_path).exists() {
            let _ = fs::remove_dir_all(&self.base_path);
        }
    }
}
