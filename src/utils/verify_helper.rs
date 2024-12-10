use std::{
    fs::{self, ReadDir},
    io,
    path::Path,
};
use tarpc::context;

use crate::{
    errors::TokenGenErrors, rpc_client::TokenGenClient,
    Result
};

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

/*
   Check dir is directory or not
   Take all .move files in that folder
   Call verify_content function from RPC
*/
pub async fn verify_contract(dir: &Path, client: TokenGenClient) -> Result<()> {
    if !dir.is_dir() {
        return Err(TokenGenErrors::InvalidPath(
            "Path is not a directory".to_string(),
        ));
    }
    let entries = read_dir(dir).unwrap();

    for entry in entries {
        match entry {
            Ok(entry) => {
                let path = entry.path();
                if path.is_file() && path.extension().map(|e| e == "move").unwrap_or(false) {
                    //Reading file
                    match read_file(&path) {
                        Ok(current_content) => {
                            let response = client.verify_content(context::current(), current_content).await;

                            match response {
                                Ok(result) => {
                                    println!("Verification success: {}", result);
                                }
                                Err(err) => {
                                    eprintln!("Verification failed: {:?}", err);
                                }
                            }
                        }
                        Err(e) => {
                            return Err(TokenGenErrors::FileIoError(e));
                        }
                    }
                }
            }
            Err(e) => {
                eprintln!("Error reading entry in directory: {}", e);
                return Err(TokenGenErrors::FileIoError(e));
            }
        }
    }
    Ok(())
}
