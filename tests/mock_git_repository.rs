use chrono::{DateTime, Utc};
use std::path::PathBuf;
use walrust::commit::Commit;
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

    fn get_commits(&self, from: DateTime<Utc>, _to: DateTime<Utc>) -> Result<Vec<Commit>> {
        let mut commit_date = from;
        let mut commits = Vec::new();

        for i in 0..2 {
            let commit = Commit::new(
                format!("mock_commit_{}", i + 1),
                "Some Dev".to_string(),
                "some@dev.local".to_string(),
                commit_date,
                format!("Commit message body {}", i + 1),
                format!("{:07}", i + 1),
                format!("{:40}", i + 1),
            );
            commits.push(commit);
            commit_date = commit_date + chrono::Duration::seconds(10 * 60 + 10);
        }

        Ok(commits)
    }
}
