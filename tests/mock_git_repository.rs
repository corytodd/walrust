/// A mock implementation of the `GitRepository` trait for testing purposes.
///
/// `MockGitRepository` allows you to simulate a Git repository by manually adding
/// commits and retrieving them based on date ranges. This is useful for testing
/// functionality that depends on a Git repository without requiring an actual
/// repository.
///
/// # Fields
/// - `commits`: A vector of `Commit` objects representing the commits in the mock repository.
///
/// # Example
/// ```rust
/// use chrono::Utc;
/// use walrust::commit::Commit;
/// use walrust::repository::GitRepository;
/// use walrust::Result;
/// use walrust::tests::mock_git_repository::MockGitRepository;
///
/// let mut mock_repo = MockGitRepository::new(&"mock_path".into()).unwrap();
///
/// // Add some mock commits
/// let commit1 = Commit {
///     commit_date: Utc::now(),
///     ..Default::default()
/// };
/// let commit2 = Commit {
///     commit_date: Utc::now(),
///     ..Default::default()
/// };
/// mock_repo.add_commits_from(vec![commit1, commit2]).unwrap();
///
/// // Retrieve commits within a date range
/// let since = Utc::now() - chrono::Duration::days(1);
/// let until = Utc::now() + chrono::Duration::days(1);
/// let commits = mock_repo.get_commits(since, until).unwrap();
/// assert_eq!(commits.len(), 2);
/// ```
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
    fn is_repo(path: &std::path::Path) -> bool {
        // This is not a real path so check the leaf directory
        path.ends_with(".git")
    }

    /// Creates a new instance of `MockGitRepository`.
    ///
    /// # Arguments
    ///
    /// * `_path` - The path to the repository (ignored in the mock implementation).
    ///
    /// # Returns
    ///
    /// A `Result` containing the new instance of `MockGitRepository`.
    ///
    /// # Example
    /// ```rust
    /// use walrust::tests::mock_git_repository::MockGitRepository;
    /// let mock_repo = MockGitRepository::new(&"mock_path".into()).unwrap();
    /// ```
    fn new(_path: &PathBuf) -> Result<Self> {
        Ok(MockGitRepository {
            commits: Vec::new(),
        })
    }

    /// Retrieves the current HEAD of the repository.
    ///
    /// # Returns
    ///
    /// A string representing the current HEAD commit hash.
    ///
    /// # Example
    /// ```rust
    /// use walrust::tests::mock_git_repository::MockGitRepository;
    /// let mock_repo = MockGitRepository::new(&"mock_path".into()).unwrap();
    /// assert_eq!(mock_repo.head(), "mock_head");
    /// ```
    fn head(&self) -> String {
        "mock_head".to_string()
    }

    /// Retrieves commits in the repository between two dates.
    ///
    /// # Arguments
    ///
    /// * `since` - Inclusive start date for the commit range.
    /// * `until` - Inclusive end date for the commit range.
    ///
    /// # Returns
    ///
    /// A `Result` containing a vector of commits within the specified date range.
    ///
    /// # Example
    /// ```rust
    /// use chrono::Utc;
    /// use walrust::commit::Commit;
    /// use walrust::tests::mock_git_repository::MockGitRepository;
    ///
    /// let mut mock_repo = MockGitRepository::new(&"mock_path".into()).unwrap();
    /// let commit = Commit {
    ///     commit_date: Utc::now(),
    ///     ..Default::default()
    /// };
    /// mock_repo.add_commits_from(vec![commit]).unwrap();
    ///
    /// let since = Utc::now() - chrono::Duration::days(1);
    /// let until = Utc::now() + chrono::Duration::days(1);
    /// let commits = mock_repo.get_commits(since, until).unwrap();
    /// assert_eq!(commits.len(), 1);
    /// ```
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
