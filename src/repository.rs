use crate::{Result, WalrustError};
use std::path::PathBuf;

pub struct Repository {
    /// The path to the local repository.
    pub uri: PathBuf,
    /// The name of the repository.
    pub name: String,
}

impl Repository {
    pub fn new(uri: &PathBuf) -> Result<Self> {
        let name = uri
            .file_name()
            .and_then(|n| n.to_str())
            .ok_or_else(|| WalrustError::PathError(uri.clone()))?
            .to_string();
        Ok(Repository {
            uri: uri.clone(),
            name,
        })
    }

    pub fn get_uri(&self) -> &PathBuf {
        &self.uri
    }

    pub fn get_name(&self) -> &String {
        &self.name
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::Path;

    #[test]
    fn test_local_git_repo_valid_path() {
        let path = Path::new("/path/to/repo");
        let repo = Repository::new(&path.to_path_buf()).unwrap();
        assert_eq!(repo.get_uri(), &path.to_path_buf());
        assert_eq!(repo.get_name(), "repo");
    }

    #[test]
    fn test_local_git_repo_invalid_path() {
        let path = Path::new("..");
        let result = Repository::new(&path.to_path_buf());
        assert!(result.is_err());
        if let Err(WalrustError::PathError(_)) = result {
            // Expected error
        } else {
            panic!("Expected PathError");
        }
    }
}
