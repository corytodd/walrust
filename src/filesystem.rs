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

    /// Checks if the given path exists.
    ///
    /// # Arguments
    /// - `path`: The path to check.
    ///
    /// # Returns
    /// `true` if the path exists, `false` otherwise.
    fn exists(&self, path: &Path) -> bool;
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

    fn exists(&self, path: &Path) -> bool {
        path.exists()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_is_dir() {
        let fs = LocalFilesystem::new();
        let temp_dir = tempdir().unwrap();

        // Check that the temporary directory is recognized as a directory
        assert!(fs.is_dir(temp_dir.path()));

        // Check that a non-existent path is not recognized as a directory
        let non_existent_path = temp_dir.path().join("non_existent");
        assert!(!fs.is_dir(&non_existent_path));
    }

    #[test]
    fn test_read_dir() {
        let fs = LocalFilesystem::new();
        let temp_dir = tempdir().unwrap();

        // Create some files in the temporary directory
        File::create(temp_dir.path().join("file1.txt")).unwrap();
        File::create(temp_dir.path().join("file2.txt")).unwrap();

        // Read the directory contents
        let entries = fs.read_dir(temp_dir.path()).unwrap();
        let entry_names: Vec<_> = entries
            .iter()
            .map(|entry| entry.file_name().unwrap().to_string_lossy().to_string())
            .collect();

        // Check that the directory contains the expected files
        assert!(entry_names.contains(&"file1.txt".to_string()));
        assert!(entry_names.contains(&"file2.txt".to_string()));
    }

    #[test]
    fn test_read_dir_non_existent() {
        let fs = LocalFilesystem::new();
        let temp_dir = tempdir().unwrap();

        // Try to read a non-existent directory
        let non_existent_path = temp_dir.path().join("non_existent");
        let result = fs.read_dir(&non_existent_path);

        // Check that an error is returned
        assert!(result.is_err());
    }

    #[test]
    fn test_exists() {
        let fs = LocalFilesystem::new();
        let temp_dir = tempdir().unwrap();

        // Create a file in the temporary directory
        let file_path = temp_dir.path().join("file.txt");
        File::create(&file_path).unwrap();

        // Check that the file exists
        assert!(fs.exists(&file_path));

        // Check that a non-existent path does not exist
        let non_existent_path = temp_dir.path().join("non_existent");
        assert!(!fs.exists(&non_existent_path));
    }
}
