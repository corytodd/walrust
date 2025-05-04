use crate::Result;
use crate::{
    filesystem::{Filesystem, LocalFilesystem},
    repository::{GitRepository, LocalGitRepository, Repository},
};
use std::path::{Path, PathBuf};

/// A trait to define the behavior of a repository query
pub trait RepoProbe {
    fn new() -> Self
    where
        Self: Sized;

    /// Check if the given path is a repository
    fn is_repo(&self, path: &Path) -> bool;
}

/// A struct to locate repositories in a given path
pub struct RepositoryLocator<
    F: Filesystem = LocalFilesystem,
    G: GitRepository = LocalGitRepository,
    R: RepoProbe = GitRepoProbe,
> {
    /// The filesystem to use for operations.
    filesystem: F,
    /// The repository query to use for checking if a path is a repository.
    repo_probe: R,
    /// The root path to start searching for repositories.
    search_root: PathBuf,
    /// The maximum depth to search for repositories.
    search_depth: usize,
    phantom: std::marker::PhantomData<G>,
}

impl<F: Filesystem, G: GitRepository, R: RepoProbe> RepositoryLocator<F, G, R> {
    pub fn new(search_root: &Path, search_depth: usize) -> Self {
        Self {
            filesystem: F::new(),
            repo_probe: R::new(),
            search_root: search_root.to_path_buf(),
            search_depth,
            phantom: std::marker::PhantomData,
        }
    }

    pub fn locate(&self) -> Result<Vec<Repository<G>>> {
        // Directory count is zero base so we need to add 1
        self.locate_recursive(&self.search_root, self.search_depth + 1)
    }

    fn locate_recursive(
        &self,
        search_root: &Path,
        search_depth: usize,
    ) -> Result<Vec<Repository<G>>> {
        let mut repositories: Vec<Repository<G>> = Vec::new();
        if search_depth == 0 {
            return Ok(repositories);
        }

        if self.filesystem.is_dir(search_root) {
            for entry in self.filesystem.read_dir(search_root).unwrap() {
                let entry_path = entry.as_path();
                if self.filesystem.is_dir(&entry_path) {
                    if self.repo_probe.is_repo(&entry_path) {
                        let repo: Result<Repository<G>> =
                            Repository::new(&search_root.to_path_buf());
                        match repo {
                            Ok(repo) => repositories.push(repo),
                            Err(err) => {
                                eprintln!("Error creating repository object: {}", err);
                            }
                        }
                    } else {
                        let sub_repositories = self.locate_recursive(&entry_path, search_depth - 1);
                        match sub_repositories {
                            Ok(sub_repositories) => repositories.extend(sub_repositories),
                            Err(err) => {
                                eprintln!("Error locating sub-repositories: {}", err);
                            }
                        }
                    }
                }
            }
        }
        Ok(repositories)
    }
}

/// Probes for Git repositories
///
/// This probe implements a filesystem check to avoid the overhead
/// of creating a git repository object for every potential repository.
pub struct GitRepoProbe;

impl RepoProbe for GitRepoProbe {
    fn new() -> Self {
        Self
    }

    fn is_repo(&self, path: &Path) -> bool {
        path.ends_with(".git")
    }
}

pub type GitRepositoryLocator =
    RepositoryLocator<LocalFilesystem, LocalGitRepository, GitRepoProbe>;
