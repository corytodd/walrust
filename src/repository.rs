use crate::{Result, WalrustError};
use git2::Repository as LibGitRepository;
use std::path::PathBuf;

/// A trait representing a generic Git repository.
///
/// This trait defines the behavior expected from a Git repository,
/// including creating a new instance, retrieving the current HEAD,
/// and fetching commits within a date range.
pub trait GitRepository {
    /// Create a new instance of the GitRepository.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the repository.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new instance of the GitRepository or an error.
    fn new(path: &PathBuf) -> Result<Self>
    where
        Self: Sized;

    /// Get the current HEAD of the repository.
    ///
    /// # Returns
    ///
    /// A string representing the current HEAD commit hash.
    fn head(&self) -> String;
}

/// A Git repository on the local filesystem.
///
/// This struct provides an implementation of the `GitRepository` trait
/// for repositories stored on the local filesystem.
pub struct LocalGitRepository {
    git: LibGitRepository,
}

impl GitRepository for LocalGitRepository {
    /// Create a new instance of a `LocalGitRepository`.
    ///
    /// # Arguments
    ///
    /// * `path` - The path to the local Git repository.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new instance of `LocalGitRepository` or an error.
    fn new(path: &PathBuf) -> Result<Self> {
        let git = LibGitRepository::open(path).map_err(|e| WalrustError::GitError(e))?;
        Ok(LocalGitRepository { git })
    }

    /// Get the current HEAD of the repository.
    ///
    /// # Returns
    ///
    /// A string representing the current HEAD commit hash.
    fn head(&self) -> String {
        self.git
            .head()
            .and_then(|h| h.peel_to_commit())
            .map(|c| c.id().to_string())
            .unwrap_or_else(|_| "HEAD".to_string())
    }
}

/// A generic repository abstraction.
///
/// This struct provides a high-level abstraction for interacting with
/// repositories, allowing for different implementations of the `GitRepository`
/// trait to be used.
pub struct Repository<G: GitRepository = LocalGitRepository> {
    /// The path to the local repository.
    pub uri: PathBuf,
    /// The name of the repository.
    pub name: String,
    /// Underlying VCS object.
    pub vcs: G,
}

impl<G: GitRepository> Repository<G> {
    /// Create a new instance of a `Repository`.
    ///
    /// # Arguments
    ///
    /// * `uri` - The path to the repository.
    ///
    /// # Returns
    ///
    /// A `Result` containing the new instance of `Repository` or an error.
    pub fn new(uri: &PathBuf) -> Result<Self> {
        let name = uri
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| WalrustError::PathError(uri.clone()))?
            .to_string();
        let vcs = G::new(uri)?;
        Ok(Repository {
            uri: uri.clone(),
            name,
            vcs,
        })
    }

    /// Get the URI of the repository.
    ///
    /// # Returns
    ///
    /// A reference to the path of the repository.
    pub fn get_uri(&self) -> &PathBuf {
        &self.uri
    }

    /// Get the name of the repository.
    ///
    /// # Returns
    ///
    /// A reference to the name of the repository.
    pub fn get_name(&self) -> &String {
        &self.name
    }
}
