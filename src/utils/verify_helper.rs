use std::{
    fs::{self, ReadDir},
    io,
    path::Path,
};
use tarpc::context;

use crate::{TokenGenErrors, Result, rpc::TokenGen};

pub fn read_file(file_path: &Path) -> io::Result<String> {
    if file_path.extension().and_then(|ext| ext.to_str()) != Some("move") {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "File is not a .move file",
        ));
    }

    let content: String = fs::read_to_string(file_path)?;
    Ok(content)
}

pub fn read_dir(dir: &Path) -> io::Result<ReadDir> {
    let content = fs::read_dir(dir)?;
    Ok(content)
}

pub async fn verify_contract(dir: &Path, client: &impl TokenGen) -> Result<()> {
    if !dir.is_dir() {
        return Err(TokenGenErrors::InvalidPath(
            "Path is not a directory".to_string(),
        ));
    }
    let mut current_content = String::new();

    let entries = read_dir(dir).unwrap();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().map(|e| e == "move").unwrap_or(false) {
                    let content = read_file(&path)?;
                    current_content.push_str(&content);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading entry in directory: {}", e);
                return Err(TokenGenErrors::FileIoError(e.to_string()));
            }
        }
    }

    if current_content.is_empty() {
        return Err(TokenGenErrors::InvalidPath(
            "No .move file found in the directory.".to_string(),
        ));
    }

    let ctx = context::current();
    client.verify_content(ctx, current_content).await?;
    Ok(())
}
