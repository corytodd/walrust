use std::fmt;
use std::io;
use std::path::PathBuf;

pub mod commit;
pub mod filesystem;
pub mod repository;
pub mod repository_locator;

pub type Result<T> = std::result::Result<T, WalrustError>;

/// Walrust error types.
#[derive(Debug)]
pub enum WalrustError {
    /// An error related to Git operations.
    GitError(git2::Error),
    /// An error related to IO operations.
    IoError(io::Error),
    /// An error related to invalid paths.
    PathError(PathBuf),
}

impl fmt::Display for WalrustError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            WalrustError::GitError(err) => write!(f, "Git error: {}", err),
            WalrustError::IoError(err) => write!(f, "IO error: {}", err),
            WalrustError::PathError(path) => write!(f, "Invalid path: {}", path.display()),
        }
    }
}

impl std::error::Error for WalrustError {}

impl From<git2::Error> for WalrustError {
    fn from(err: git2::Error) -> Self {
        WalrustError::GitError(err)
    }
}

impl From<io::Error> for WalrustError {
    fn from(err: io::Error) -> Self {
        WalrustError::IoError(err)
    }
}
