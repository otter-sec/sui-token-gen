use std::{fs, path::Path};

pub struct AtomicFileOperation {
    base_path: String,
    completed: bool,
}

impl AtomicFileOperation {
    pub fn new(base_path: &str) -> Self {
        Self {
            base_path: base_path.to_string(),
            completed: false,
        }
    }

    pub fn commit(&mut self) {
        self.completed = true;
    }
}

impl Drop for AtomicFileOperation {
    fn drop(&mut self) {
        if !self.completed && Path::new(&self.base_path).exists() {
            let _ = fs::remove_dir_all(&self.base_path);
        }
    }
}
