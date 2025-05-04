mod mock_git_repository;
use mock_git_repository::MockGitRepository;
use std::path::Path;
use walrust::repository::Repository;
use walrust::WalrustError;

/// A type alias for a `Repository` using the mock Git repository.
///
/// This alias simplifies the usage of the `Repository` in tests by
/// specifying `MockGitRepository` as the generic parameter.
type MockRepository = Repository<MockGitRepository>;

/// Tests for the `Repository` struct.
///
/// These tests verify the behavior of the `Repository` struct when interacting
/// with mock Git repositories. The tests cover both valid and invalid paths
/// to ensure proper error handling and functionality.
#[cfg(test)]
mod tests {
    use super::*;

    /// Tests that a `Repository` can be successfully created with a valid path.
    ///
    /// This test verifies that the `Repository` correctly initializes with a valid
    /// path and that its URI and name are set as expected.
    ///
    /// # Panics
    /// This test will panic if:
    /// - The `Repository` fails to initialize with a valid path.
    /// - The URI or name of the `Repository` does not match the expected values.
    #[test]
    fn test_local_git_repo_valid_path() {
        let path = Path::new("/path/to/repo");
        let repo = MockRepository::new(&path.to_path_buf());
        assert!(repo.is_ok());

        let repo = repo.unwrap();
        assert_eq!(repo.get_uri(), &path.to_path_buf());
        assert_eq!(repo.get_name(), "repo");
    }

    /// Tests that a `Repository` returns an error when initialized with an invalid path.
    ///
    /// This test verifies that the `Repository` correctly returns a `PathError`
    /// when initialized with an invalid path.
    ///
    /// # Panics
    /// This test will panic if:
    /// - The `Repository` does not return an error for an invalid path.
    /// - The error returned is not a `PathError`.
    #[test]
    fn test_local_git_repo_invalid_path() {
        let path = Path::new("..");
        let repo = MockRepository::new(&path.to_path_buf());
        assert!(repo.is_err());
        if let Err(WalrustError::PathError(_)) = repo {
            // Expected error
        } else {
            panic!("Expected PathError");
        }
    }
}
