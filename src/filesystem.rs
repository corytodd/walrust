use std::fs;
use std::path::{Path, PathBuf};

/// A trait to abstract filesystem operations
pub trait Filesystem {
    fn is_dir(&self, path: &Path) -> bool;
    fn read_dir(&self, path: &Path) -> std::io::Result<Vec<PathBuf>>;
}

/// A concrete implementation of the Filesystem trait that interacts with the local filesystem
#[derive(Debug)]
pub struct LocalFilesystem;

impl Filesystem for LocalFilesystem {
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
