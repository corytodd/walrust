//! # Walrust
//!
//! `walrust` is a library for working with Git repositories in Rust. It provides abstractions
//! for interacting with Git repositories, locating repositories on the filesystem, and handling
//! commits. The library is designed to be extensible and testable, with support for mock
//! implementations.
//!
//! # Modules
//!
//! - [`commit`]: Defines the `Commit` struct and related functionality.
//! - [`filesystem`]: Provides abstractions for filesystem operations.
//! - [`repository`]: Defines the `GitRepository` trait and its implementations.
//! - [`repository_locator`]: Provides functionality for locating repositories on the filesystem.
//!
//! # Example
//!
//! ```rust
//! use walrust::repository_locator::GitRepositoryLocator;
//! use std::path::Path;
//!
//! let locator = GitRepositoryLocator::new(Path::new("/path/to/search"), 3);
//! let repositories = locator.locate().unwrap();
//! for repo in repositories {
//!     println!("Found repository: {}", repo.uri.display());
//! }
//! ```

use std::fmt;
use std::io;
use std::path::PathBuf;

pub mod commit;
pub mod filesystem;
pub mod repository;
pub mod repository_locator;

/// A type alias for results returned by the Walrust library.
///
/// This alias simplifies error handling by using the `WalrustError` type
/// as the error variant in `Result`.
pub type Result<T> = std::result::Result<T, WalrustError>;

/// Walrust error types.
///
/// This enum defines the various error types that can occur in the Walrust library.
/// It provides variants for Git-related errors, IO errors, and invalid paths.
///
/// # Variants
/// - `GitError`: Represents errors related to Git operations.
/// - `IoError`: Represents errors related to IO operations.
/// - `PathError`: Represents errors related to invalid paths.
///
/// # Example
/// ```rust
/// use walrust::WalrustError;
/// use std::path::PathBuf;
///
/// let error = WalrustError::PathError(PathBuf::from("/invalid/path"));
/// println!("{}", error);
/// ```
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
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_walrust_error_display_git_error() {
        let git_error = git2::Error::from_str("Git operation failed");
        let error = WalrustError::GitError(git_error);
        assert_eq!(format!("{}", error), "Git error: Git operation failed");
    }

    #[test]
    fn test_walrust_error_display_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "IO operation failed");
        let error = WalrustError::IoError(io_error);
        assert_eq!(format!("{}", error), "IO error: IO operation failed");
    }

    #[test]
    fn test_walrust_error_display_path_error() {
        let path = PathBuf::from("/invalid/path");
        let error = WalrustError::PathError(path.clone());
        assert_eq!(
            format!("{}", error),
            format!("Invalid path: {}", path.display())
        );
    }

    #[test]
    fn test_walrust_error_from_git_error() {
        let git_error = git2::Error::from_str("Git operation failed");
        let error: WalrustError = git_error.into();
        match error {
            WalrustError::GitError(_) => assert!(true),
            _ => assert!(false, "Expected WalrustError::GitError"),
        }
    }

    #[test]
    fn test_walrust_error_from_io_error() {
        let io_error = io::Error::new(io::ErrorKind::Other, "IO operation failed");
        let error: WalrustError = io_error.into();
        match error {
            WalrustError::IoError(_) => assert!(true),
            _ => assert!(false, "Expected WalrustError::IoError"),
        }
    }
}
