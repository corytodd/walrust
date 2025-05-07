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
