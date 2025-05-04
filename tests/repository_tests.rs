mod mock_git_repository;
use mock_git_repository::MockGitRepository;
use std::path::Path;
use walrust::repository::Repository;
use walrust::WalrustError;

type MockRepository = Repository<MockGitRepository>;

#[test]
fn test_local_git_repo_valid_path() {
    let path = Path::new("/path/to/repo");
    let repo = MockRepository::new(&path.to_path_buf());
    assert!(repo.is_ok());

    let repo = repo.unwrap();
    assert_eq!(repo.get_uri(), &path.to_path_buf());
    assert_eq!(repo.get_name(), "repo");
}

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
