use std::path::PathBuf;
use walrust::repository::GitRepository;
use walrust::Result;

pub struct MockGitRepository;

impl GitRepository for MockGitRepository {
    fn new(_path: &PathBuf) -> Result<Self> {
        Ok(MockGitRepository)
    }

    fn head(&self) -> String {
        "mock_head".to_string()
    }
}
