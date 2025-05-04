use crate::commit::Commit;
use crate::{Result, WalrustError};
use chrono::{DateTime, Utc};
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

    /// Get the commits in the repository between two dates.
    ///
    /// # Arguments
    ///
    /// * `since` - Inclusive start date for the commit range.
    /// * `until` - Inclusive end date for the commit range.
    ///
    /// # Returns
    ///
    /// A vector of commits within the specified date range.
    ///
    /// # Errors
    ///
    /// Returns an error if the commit retrieval fails.
    fn get_commits(&self, since: DateTime<Utc>, until: DateTime<Utc>) -> Result<Vec<Commit>>;
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

    /// Get the commits in the repository between two dates.
    ///
    /// # Arguments
    ///
    /// * `since` - Inclusive start date for the commit range.
    /// * `until` - Inclusive end date for the commit range.
    ///
    /// # Returns
    ///
    /// A vector of commits within the specified date range.
    ///
    /// # Errors
    ///
    /// Returns an error if the commit retrieval fails.
    fn get_commits(&self, since: DateTime<Utc>, until: DateTime<Utc>) -> Result<Vec<Commit>> {
        let mut revwalk = self.git.revwalk()?;
        revwalk.push_head()?; // Start from HEAD
        revwalk.set_sorting(git2::Sort::TIME)?; // Sort commits by time (newest to oldest)
        let mut commits = Vec::new();

        for oid in revwalk {
            let oid = oid?;
            let commit = self.git.find_commit(oid)?;

            let commit_date = DateTime::from_timestamp(commit.time().seconds(), 0).ok_or(
                WalrustError::GitError(git2::Error::from_str(
                    "Failed to convert commit time to DateTime",
                )),
            )?;

            // Stop processing if the commit is older than the `since` date
            if commit_date < since {
                break;
            }

            // Only include commits within the date range
            if commit_date <= until {
                let commit_hash = commit.id().to_string();
                commits.push(Commit::new(
                    commit.summary().unwrap_or_default().to_string(),
                    commit.author().name().unwrap_or_default().to_string(),
                    commit.author().email().unwrap_or_default().to_string(),
                    commit_date,
                    commit.message().unwrap_or_default().to_string(),
                    commit_hash.clone()[..7].to_string(),
                    commit_hash.clone(),
                ));
            }
        }

        Ok(commits)
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
