use std::fs;
use std::path::{Path, PathBuf};

/// A trait to abstract filesystem operations.
///
/// This trait provides an abstraction over common filesystem operations,
/// such as checking if a path is a directory and reading the contents of a directory.
/// It allows for custom implementations, making it easier to test code that interacts
/// with the filesystem.
pub trait Filesystem {
    /// Creates a new instance of the filesystem.
    ///
    /// # Returns
    /// A new instance of the implementing type.
    fn new() -> Self
    where
        Self: Sized;

    /// Checks if the given path is a directory.
    ///
    /// # Arguments
    /// - `path`: The path to check.
    ///
    /// # Returns
    /// `true` if the path is a directory, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use walrust::filesystem::Filesystem;
    /// use std::path::Path;
    ///
    /// let fs = walrust::filesystem::LocalFilesystem::new();
    /// assert!(fs.is_dir(Path::new(".")));
    /// ```
    fn is_dir(&self, path: &Path) -> bool;

    /// Reads the contents of a directory.
    ///
    /// # Arguments
    /// - `path`: The path to the directory to read.
    ///
    /// # Returns
    /// A `Result` containing a vector of `PathBuf` objects representing the contents
    /// of the directory, or an error if the directory cannot be read.
    ///
    /// # Errors
    /// Returns an error if the directory does not exist or cannot be accessed.
    ///
    /// # Example
    /// ```rust
    /// use walrust::filesystem::Filesystem;
    /// use std::path::Path;
    ///
    /// let fs = walrust::filesystem::LocalFilesystem::new();
    /// let entries = fs.read_dir(Path::new(".")).unwrap();
    /// for entry in entries {
    ///     println!("Found: {}", entry.display());
    /// }
    /// ```
    fn read_dir(&self, path: &Path) -> std::io::Result<Vec<PathBuf>>;
}

/// A concrete implementation of the `Filesystem` trait that interacts with the local filesystem.
///
/// This implementation uses the standard library's `std::fs` module to perform filesystem operations.
/// It is suitable for production use but can be replaced with a mock implementation for testing.
///
/// # Example
/// ```rust
/// use walrust::filesystem::{Filesystem, LocalFilesystem};
/// use std::path::Path;
///
/// let fs = LocalFilesystem::new();
/// assert!(fs.is_dir(Path::new(".")));
/// let entries = fs.read_dir(Path::new(".")).unwrap();
/// for entry in entries {
///     println!("Found: {}", entry.display());
/// }
/// ```
#[derive(Debug)]
pub struct LocalFilesystem;

impl Filesystem for LocalFilesystem {
    fn new() -> Self {
        LocalFilesystem
    }

    fn is_dir(&self, path: &Path) -> bool {
        path.is_dir()
    }

    fn read_dir(&self, path: &Path) -> std::io::Result<Vec<PathBuf>> {
        fs::read_dir(path)
            .map(|entries| {
                entries
                    .filter_map(|entry| entry.ok().map(|e| e.path()))
                    .collect()
            })
            .map_err(|e| std::io::Error::new(e.kind(), format!("Failed to read directory: {}", e)))
    }
}
