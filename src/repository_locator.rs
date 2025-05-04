use crate::filesystem::{Filesystem, RealFilesystem};
use std::path::{Path, PathBuf};

/// A trait to define the behavior of a repository query
pub trait RepoProbe {
    /// Check if the given path is a repository
    fn is_repo(&self, path: &Path) -> bool;
}

/// A struct to locate repositories in a given path
pub struct RepositoryLocator<F: Filesystem, R: RepoProbe> {
    /// The filesystem to use for operations.
    filesystem: F,
    /// The repository query to use for checking if a path is a repository.
    repo_probe: R,
    /// The root path to start searching for repositories.
    search_root: PathBuf,
    /// The maximum depth to search for repositories.
    search_depth: usize,
}

impl<F: Filesystem, R: RepoProbe> RepositoryLocator<F, R> {
    pub fn new(filesystem: F, repo_probe: R, search_root: &Path, search_depth: usize) -> Self {
        Self {
            filesystem,
            repo_probe,
            search_root: search_root.to_path_buf(),
            search_depth,
        }
    }

    pub fn locate(&self) -> Vec<PathBuf> {
        // Directory count is zero base so we need to add 1
        self.locate_recursive(&self.search_root, self.search_depth + 1)
    }

    fn locate_recursive(&self, search_root: &Path, search_depth: usize) -> Vec<PathBuf> {
        let mut repositories = Vec::new();
        if search_depth == 0 {
            return repositories;
        }

        if self.filesystem.is_dir(search_root) {
            for entry in self.filesystem.read_dir(search_root).unwrap() {
                let entry_path = entry.as_path();
                if self.filesystem.is_dir(&entry_path) {
                    if self.repo_probe.is_repo(&entry_path) {
                        repositories.push(search_root.to_path_buf());
                    } else {
                        let sub_repositories = self.locate_recursive(&entry_path, search_depth - 1);
                        repositories.extend(sub_repositories);
                    }
                }
            }
        }
        repositories
    }
}

pub struct GitRepoProbe;

impl RepoProbe for GitRepoProbe {
    fn is_repo(&self, path: &Path) -> bool {
        path.ends_with(".git")
    }
}

pub struct GitRepositoryLocator {
    inner: RepositoryLocator<RealFilesystem, GitRepoProbe>,
}

impl GitRepositoryLocator {
    pub fn new(search_root: &Path, search_depth: usize) -> Self {
        let locator =
            RepositoryLocator::new(RealFilesystem, GitRepoProbe, search_root, search_depth);
        Self { inner: locator }
    }

    pub fn locate(&self) -> Vec<PathBuf> {
        self.inner.locate()
    }
}
