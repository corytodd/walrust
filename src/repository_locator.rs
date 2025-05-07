use crate::Result;
use crate::{
    filesystem::{Filesystem, LocalFilesystem},
    repository::{GitRepository, LocalGitRepository, Repository},
};
use std::path::{Path, PathBuf};

/// The `repository_locator` module provides functionality for locating repositories
/// in a given directory. It uses abstractions for filesystem operations and
/// repository detection to support extensibility and testing.
///
/// # Key Components
/// - `RepositoryLocator`: The main struct for locating repositories.
///
/// # Example
/// ```rust
/// use walrust::repository_locator::GitRepositoryLocator;
/// use std::path::Path;
///
/// let locator = GitRepositoryLocator::new(Path::new("/path/to/search"), 3);
/// let repositories = locator.locate().unwrap();
/// for repo in repositories {
///     println!("Found repository: {}", repo.uri.display());
/// }
/// ```

/// A struct to locate repositories in a given path.
///
/// This struct recursively searches for repositories in a given directory It
/// supports configurable search depth and can be extended with custom
/// filesystem or repository implementations.
///
/// # Type Parameters
/// - `F`: The filesystem abstraction to use (default: `LocalFilesystem`).
/// - `G`: The repository implementation to use (default: `LocalGitRepository`).
///
/// # Example
/// ```rust
/// use walrust::repository_locator::{GitRepositoryLocator};
/// use std::path::Path;
///
/// let locator = GitRepositoryLocator::new(Path::new("/path/to/search"), 3);
/// let repositories = locator.locate().unwrap();
/// for repo in repositories {
///     println!("Found repository: {}", repo.uri.display());
/// }
/// ```
pub struct RepositoryLocator<F: Filesystem = LocalFilesystem, G: GitRepository = LocalGitRepository>
{
    /// The filesystem to use for operations.
    filesystem: F,
    /// The root path to start searching for repositories.
    search_root: PathBuf,
    /// The maximum depth to search for repositories.
    search_depth: usize,
    phantom: std::marker::PhantomData<G>,
}

impl<F: Filesystem, G: GitRepository> RepositoryLocator<F, G> {
    pub fn new(search_root: &Path, search_depth: usize) -> Self {
        Self {
            filesystem: F::new(),
            search_root: search_root.to_path_buf(),
            search_depth,
            phantom: std::marker::PhantomData,
        }
    }

    /// Locates repositories in the configured search root.
    ///
    /// This method starts the recursive search for repositories from the
    /// configured `search_root` and up to the specified `search_depth`.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Repository<G>` objects if successful,
    /// or an error if the search fails.
    ///
    /// # Errors
    /// - Returns an error if the filesystem operations fail (e.g., reading directories).
    /// - Returns an error if a repository object cannot be created.
    ///
    /// # Example
    /// ```rust
    /// use walrust::repository_locator::GitRepositoryLocator;
    /// let locator = GitRepositoryLocator::new(std::path::Path::new("/path/to/search"), 1);
    /// let repositories = locator.locate().unwrap();
    /// for repo in repositories {
    ///     println!("Found repository: {}", repo.uri.display());
    /// }
    /// ```
    pub fn locate(&self) -> Result<Vec<Repository<G>>> {
        self.locate_recursive(&self.search_root, self.search_depth)
    }

    /// Recursively locates repositories in the given path.
    ///
    /// This method is called internally by `locate` to perform a depth-first
    /// search for repositories. It checks each directory to determine if it
    /// is a repository or contains subdirectories to search further.
    ///
    /// # Arguments
    /// - `search_root`: The current directory to search.
    /// - `search_depth`: The remaining depth to search.
    ///
    /// # Returns
    /// A `Result` containing a vector of `Repository<G>` objects if successful,
    /// or an error if the search fails.
    fn locate_recursive(
        &self,
        search_root: &Path,
        search_depth: usize,
    ) -> Result<Vec<Repository<G>>> {
        let mut repositories: Vec<Repository<G>> = Vec::new();

        if !self.filesystem.is_dir(search_root) {
            return Ok(repositories);
        }

        // Happy path - the current search root is a repository
        match self.try_make_repo(search_root) {
            Some(repo) => {
                repositories.push(repo);
                return Ok(repositories);
            }
            None => {
                // If the search depth is 0, stop searching further
                if search_depth == 0 {
                    return Ok(repositories);
                }

                // Otherwise, recursively search for repositories
                for entry in self.filesystem.read_dir(search_root).unwrap() {
                    let entry_path = entry.as_path();
                    repositories.extend(self.locate_recursive(entry_path, search_depth - 1)?);
                }
            }
        }

        Ok(repositories)
    }

    /// Attempts to create a repository object from the given path.
    ///
    /// This method checks if the path is a valid repository and creates
    /// a `Repository<G>` object if it is. If the path is not a valid repository,
    /// it returns `None`.
    ///
    /// # Arguments
    /// - `path`: The path to check and create a repository object from.
    ///
    /// # Returns
    /// An `Option<Repository<G>>` containing the repository object if successful,
    /// or `None` if the path is not a valid repository.
    fn try_make_repo(&self, path: &Path) -> Option<Repository<G>> {
        let expect_git_path = path.join(".git");
        if !self.filesystem.exists(&expect_git_path) {
            return None;
        }

        match Repository::new(&path.to_path_buf()) {
            Ok(repo) => Some(repo),
            Err(_) => {
                eprintln!("Failed to create repository from path: {}", path.display());
                None
            }
        }
    }
}

/// A type alias for a `RepositoryLocator` with default implementations.
///
/// This alias uses the following defaults:
/// - `LocalFilesystem` for filesystem operations.
/// - `LocalGitRepository` for repository objects.
///
/// # Example
/// ```rust
/// use walrust::repository_locator::GitRepositoryLocator;
/// use std::path::Path;
///
/// let locator = GitRepositoryLocator::new(Path::new("/path/to/search"), 3);
/// let repositories = locator.locate().unwrap();
/// for repo in repositories {
///     println!("Found repository: {}", repo.uri.display());
/// }
/// ```
pub type GitRepositoryLocator = RepositoryLocator<LocalFilesystem, LocalGitRepository>;
