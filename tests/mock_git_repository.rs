use chrono::{DateTime, Utc};
use std::path::PathBuf;
use walrust::commit::Commit;
use walrust::repository::GitRepository;
use walrust::Result;

/// A mock implementation of a Git repository for testing purposes.
///
/// This struct stores a list of commits and provides methods to add commits
/// and retrieve them based on date ranges.
pub struct MockGitRepository {
    /// A vector of commits stored in the mock repository.
    commits: Vec<Commit>,
}

impl GitRepository for MockGitRepository {
    fn new(_path: &PathBuf) -> Result<Self> {
        Ok(MockGitRepository {
            commits: Vec::new(),
        })
    }

    fn head(&self) -> String {
        "mock_head".to_string()
    }

    fn get_commits(&self, since: DateTime<Utc>, until: DateTime<Utc>) -> Result<Vec<Commit>> {
        let filtered_commits: Vec<Commit> = self
            .commits
            .iter()
            .filter(|commit| commit.commit_date >= since)
            .filter(|commit| commit.commit_date <= until)
            .cloned()
            .collect();
        Ok(filtered_commits)
    }
}
