use crate::{Result, WalrustError};
use git2::Repository as LibGitRepository;
use std::path::PathBuf;

pub trait GitRepository {
    /// Create a new instance of the GitRepository.
    fn new(path: &PathBuf) -> Result<Self>
    where
        Self: Sized;

    /// Get the current HEAD of the repository.
    fn head(&self) -> String;
}

/// A Git repository on the local filesystem.
pub struct LocalGitRepository {
    git: LibGitRepository,
}

impl GitRepository for LocalGitRepository {
    fn new(path: &PathBuf) -> Result<Self> {
        let git = LibGitRepository::open(path).map_err(|e| WalrustError::GitError(e))?;
        Ok(LocalGitRepository { git })
    }

    fn head(&self) -> String {
        self.git
            .head()
            .and_then(|h| h.peel_to_commit())
            .map(|c| c.id().to_string())
            .unwrap_or_else(|_| "HEAD".to_string())
    }
}

pub struct Repository<G: GitRepository = LocalGitRepository> {
    /// The path to the local repository.
    pub uri: PathBuf,
    /// The name of the repository.
    pub name: String,
    /// Underlying VCS object.
    pub vcs: G,
}

impl<G: GitRepository> Repository<G> {
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

    pub fn get_uri(&self) -> &PathBuf {
        &self.uri
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}
