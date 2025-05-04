use std::path::{Path, PathBuf};
mod mock_filesystem;
use mock_filesystem::MockFilesystem;
mod mock_git_repository;
use mock_git_repository::MockGitRepository;
use std::collections::HashSet;
use walrust::repository_locator::RepositoryLocator;

/// A type alias for a `RepositoryLocator` using the mock filesystem and mock Git repository.
///
/// This alias simplifies the usage of the `RepositoryLocator` in tests by
/// specifying `MockFilesystem` and `MockGitRepository` as the generic parameters.
type MockGitRepositoryLocator = RepositoryLocator<MockFilesystem, MockGitRepository>;

/// A helper function to check the URIs of the repositories located by the `RepositoryLocator`.
///
/// This function runs the repository locator with the given search root and depth,
/// and compares the discovered repository URIs with the expected URIs.
///
/// # Arguments
/// * `expected_uris` - A vector of expected URIs.
/// * `search_root` - The root path to start searching for repositories.
/// * `search_depth` - The maximum depth to search for repositories.
///
/// # Panics
/// This function will panic if:
/// - The repository locator fails to locate repositories.
/// - The number of discovered repositories does not match the expected number.
/// - The discovered repository URIs do not match the expected URIs.
///
/// # Example
/// ```rust
/// use std::path::Path;
/// use walrust::tests::repository_locator_tests::run_tests;
///
/// let expected_uris = vec![Path::new("root/nested_1").to_path_buf()];
/// let search_root = Path::new("root");
/// let search_depth = 1;
///
/// run_tests(expected_uris, search_root, search_depth);
/// ```
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

/// Tests that no repositories are discovered when the search depth is 0.
///
/// This test verifies that the repository locator does not find any repositories
/// when the search depth is set to 0, even if repositories exist in the root directory.
#[test]
fn test_discover_repositories_depth_0() {
    let expected_uris: Vec<PathBuf> = Vec::new();
    let search_root = Path::new("root");
    let search_depth = 0;

    run_tests(expected_uris, search_root, search_depth);
}

/// Tests that repositories are discovered at depth 1.
///
/// This test verifies that the repository locator correctly discovers repositories
/// located directly within the root directory when the search depth is set to 1.
#[test]
fn test_discover_repositories_depth_1() {
    let expected_uris: Vec<PathBuf> = vec![Path::new("root/nested_1").to_path_buf()];
    let search_root = Path::new("root");
    let search_depth = 1;

    run_tests(expected_uris, search_root, search_depth);
}

/// Tests that repositories are discovered at depth 2.
///
/// This test verifies that the repository locator correctly discovers repositories
/// located within the root directory and one additional level of subdirectories
/// when the search depth is set to 2.
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

/// Tests that repositories are discovered at depth 3.
///
/// This test verifies that the repository locator correctly discovers repositories
/// located within the root directory and up to two additional levels of subdirectories
/// when the search depth is set to 3.
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
