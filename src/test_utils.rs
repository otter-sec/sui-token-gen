use std::{
    net::{IpAddr, Ipv6Addr, SocketAddr},
    time::Duration,
    sync::Mutex,
};
use tokio::net::TcpStream as TokioTcpStream;
use once_cell::sync::Lazy;

use crate::{
    errors::TokenGenErrors,
    rpc_client::TokenGenClient,
    rpc::{server::TokenServer, TokenGen},
    Result,
};

// Global shutdown sender
static SHUTDOWN_SENDER: Lazy<Mutex<Option<tokio::sync::oneshot::Sender<()>>>> = Lazy::new(|| Mutex::new(None));

/// Helper function to set up a test client with consistent error handling
pub async fn setup_test_client() -> Result<impl TokenGen> {
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 50051);
    TokenGenClient::connect(addr).await
}

/// Helper function to set up a test server
pub async fn setup_test_server() -> Result<()> {
    let addr = SocketAddr::new(IpAddr::V6(Ipv6Addr::LOCALHOST), 50051);
    let server = TokenServer::new(addr);

    // Create shutdown channel
    let (tx, rx) = tokio::sync::oneshot::channel();

    // Store sender for cleanup
    SHUTDOWN_SENDER.lock().unwrap().replace(tx);

    // Spawn server with proper shutdown handling
    tokio::spawn(async move {
        tokio::select! {
            _ = rx => {
                println!("Shutting down server...");
            }
            res = server.serve() => {
                if let Err(e) = res {
                    eprintln!("Server error: {}", e);
                }
            }
        }
    });

    // Wait for server to be ready with increased timeout
    let mut attempts = 5;
    while attempts > 0 {
        if let Ok(_) = TokioTcpStream::connect(addr).await {
            return Ok(());
        }
        tokio::time::sleep(Duration::from_millis(200)).await;
        attempts -= 1;
    }

    Err(TokenGenErrors::RpcError("Server failed to start".into()))
}

/// Sets up the complete test environment including server and client
pub async fn setup_test_environment() -> Result<impl TokenGen> {
    setup_test_server().await?;

    // Retry connection a few times before giving up
    let mut retries = 3;
    let mut last_error = None;

    while retries > 0 {
        match setup_test_client().await {
            Ok(client) => return Ok(client),
            Err(e) => {
                last_error = Some(e);
                retries -= 1;
                tokio::time::sleep(Duration::from_millis(100)).await;
            }
        }
    }

    Err(last_error.unwrap_or_else(|| TokenGenErrors::RpcError("Failed to connect to test server".into())))
}

/// Helper function to clean up the test environment
pub fn cleanup_test_environment() {
    // Shutdown the server if it's running
    if let Some(tx) = SHUTDOWN_SENDER.lock().unwrap().take() {
        let _ = tx.send(());
    }

    // Clean up any temporary test files in the current directory
    if let Ok(entries) = std::fs::read_dir(".") {
        for entry in entries.flatten() {
            let path = entry.path();
            if let Some(name) = path.file_name().and_then(|n| n.to_str()) {
                // Clean up temporary test directories and files
                if name.starts_with("test_") {
                    let _ = if path.is_dir() {
                        std::fs::remove_dir_all(path)
                    } else {
                        std::fs::remove_file(path)
                    };
                }
            }
        }
    }
}

/// Mock implementations for testing
pub mod mocks {
    use std::path::PathBuf;
    use git2::Repository;
    use crate::Result;

    /// Mock Git repository for testing
    pub struct MockGitRepo {
        path: PathBuf,
    }

    impl MockGitRepo {
        /// Create a new mock git repository
        pub fn new(path: PathBuf) -> Self {
            Self { path }
        }

        /// Initialize a mock git repository
        pub fn init(&self) -> Result<Repository> {
            Repository::init(&self.path)
                .map_err(|e| crate::errors::TokenGenErrors::GitError(e.to_string()))
        }

        /// Add test files to the mock repository
        pub fn add_test_files(&mut self, files: Vec<(&str, &str)>) -> Result<()> {
            let repo = self.init()?;

            for (name, content) in files {
                let file_path = self.path.join(name);
                if let Some(parent) = file_path.parent() {
                    std::fs::create_dir_all(parent)?;
                }
                std::fs::write(&file_path, content)?;

                let mut index = repo.index()?;
                index.add_path(std::path::Path::new(name))?;
                index.write()?;
            }

            let tree_id = repo.index()?.write_tree()?;
            let tree = repo.find_tree(tree_id)?;

            let sig = git2::Signature::now("test", "test@example.com")?;
            repo.commit(Some("HEAD"), &sig, &sig, "Initial commit", &tree, &[])?;

            Ok(())
        }

        /// Clean up the mock repository
        pub fn cleanup(&self) -> std::io::Result<()> {
            if self.path.exists() {
                std::fs::remove_dir_all(&self.path)?;
            }
            Ok(())
        }
    }
}
