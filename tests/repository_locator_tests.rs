use std::path::{Path, PathBuf};
mod mock_filesystem;
use mock_filesystem::MockFilesystem;
mod mock_git_repository;
use mock_git_repository::MockGitRepository;
use std::collections::HashSet;
use walrust::repository_locator::RepositoryLocator;

type MockGitRepositoryLocator = RepositoryLocator<MockFilesystem, MockGitRepository>;

/// A helper function to check the URIs of the repositories
/// located by the `RepositoryLocator`.
///
///
/// # Arguments
/// * `expected_uris` - A vector of expected URIs.
/// * `search_root` - The root path to start searching for repositories.
/// * `search_depth` - The maximum depth to search for repositories.
fn run_tests(expected_uris: Vec<PathBuf>, search_root: &Path, search_depth: usize) {
    let locator = MockGitRepositoryLocator::new(search_root, search_depth);

    let repositories = locator.locate();
    assert!(repositories.is_ok());

    let repositories = repositories.unwrap();

    assert_eq!(repositories.len(), expected_uris.len());
    let actual_uris_set: HashSet<_> = repositories.iter().map(|repo| repo.get_uri()).collect();
    let expected_uris_set: HashSet<_> = expected_uris.iter().collect();

    assert_eq!(actual_uris_set, expected_uris_set);
}

#[test]
fn test_discover_repositories_depth_0() {
    let expected_uris: Vec<PathBuf> = Vec::new();
    let search_root = Path::new("root");
    let search_depth = 0;

    run_tests(expected_uris, search_root, search_depth);
}

#[test]
fn test_discover_repositories_depth_1() {
    let expected_uris: Vec<PathBuf> = vec![Path::new("root/nested_1").to_path_buf()];
    let search_root = Path::new("root");
    let search_depth = 1;

    run_tests(expected_uris, search_root, search_depth);
}

#[test]
fn test_discover_repositories_depth_2() {
    let expected_uris: Vec<PathBuf> = vec![
        Path::new("root/nested_1").to_path_buf(),
        Path::new("root/depth_2/nested_2").to_path_buf(),
    ];
    let search_root = Path::new("root");
    let search_depth = 2;

    run_tests(expected_uris, search_root, search_depth);
}

#[test]
fn test_discover_repositories_depth_3() {
    let expected_uris: Vec<PathBuf> = vec![
        Path::new("root/nested_1").to_path_buf(),
        Path::new("root/depth_2/nested_2").to_path_buf(),
        Path::new("root/depth_3/depth_3/nested_3").to_path_buf(),
    ];
    let search_root = Path::new("root");
    let search_depth = 3;

    run_tests(expected_uris, search_root, search_depth);
}
