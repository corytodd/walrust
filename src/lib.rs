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
///
/// # Example
/// ```rust
/// use walrust::Result;
///
/// fn example_function() -> Result<()> {
///     // Your code here
///     Ok(())
/// }
/// ```
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
    /// Formats the `WalrustError` for display.
    ///
    /// # Arguments
    /// - `f`: The formatter to write the error message to.
    ///
    /// # Returns
    /// A `fmt::Result` indicating success or failure.
    ///
    /// # Example
    /// ```rust
    /// use walrust::WalrustError;
    /// use std::path::PathBuf;
    ///
    /// let error = WalrustError::PathError(PathBuf::from("/invalid/path"));
    /// println!("{}", error); // Output: "Invalid path: /invalid/path"
    /// ```
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
    /// Converts a `git2::Error` into a `WalrustError`.
    ///
    /// # Arguments
    /// - `err`: The `git2::Error` to convert.
    ///
    /// # Returns
    /// A `WalrustError::GitError` variant containing the original error.
    ///
    /// # Example
    /// ```rust
    /// use walrust::WalrustError;
    /// use git2::Error;
    ///
    /// let git_error = Error::from_str("Git operation failed");
    /// let walrust_error: WalrustError = git_error.into();
    /// println!("{}", walrust_error); // Output: "Git error: Git operation failed"
    /// ```
    fn from(err: git2::Error) -> Self {
        WalrustError::GitError(err)
    }
}

impl From<io::Error> for WalrustError {
    /// Converts an `io::Error` into a `WalrustError`.
    ///
    /// # Arguments
    /// - `err`: The `io::Error` to convert.
    ///
    /// # Returns
    /// A `WalrustError::IoError` variant containing the original error.
    ///
    /// # Example
    /// ```rust
    /// use walrust::WalrustError;
    /// use std::io;
    ///
    /// let io_error = io::Error::new(io::ErrorKind::NotFound, "File not found");
    /// let walrust_error: WalrustError = io_error.into();
    /// println!("{}", walrust_error); // Output: "IO error: File not found"
    /// ```
    fn from(err: io::Error) -> Self {
        WalrustError::IoError(err)
    }
}
