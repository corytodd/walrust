use std::path::{Path, PathBuf};
mod mock_filesystem;
use mock_filesystem::{create_mock_directory_tree, MockFilesystem};
use walrust::repository_locator::{GitRepoProbe, RepositoryLocator};

fn check_uris(actual_uris: Vec<PathBuf>, expected_uris: Vec<PathBuf>) {
    assert_eq!(actual_uris.len(), expected_uris.len());
    assert_eq!(
        actual_uris.iter().collect::<std::collections::HashSet<_>>(),
        expected_uris
            .iter()
            .collect::<std::collections::HashSet<_>>()
    );
}

#[test]
fn test_discover_repositories_depth_0() {
    let mock_fs = MockFilesystem::new(create_mock_directory_tree());

    let expected_uris: Vec<PathBuf> = Vec::new();
    let search_root = Path::new("root");
    let search_depth = 0;

    let locator = RepositoryLocator::new(mock_fs, GitRepoProbe, search_root, search_depth);

    let repositories = locator.locate();
    check_uris(repositories, expected_uris);
}

#[test]
fn test_discover_repositories_depth_1() {
    let mock_fs = MockFilesystem::new(create_mock_directory_tree());

    let expected_uris: Vec<PathBuf> = vec![Path::new("root/nested_1").to_path_buf()];
    let search_root = Path::new("root");
    let search_depth = 1;

    let locator = RepositoryLocator::new(mock_fs, GitRepoProbe, search_root, search_depth);

    let repositories = locator.locate();
    check_uris(repositories, expected_uris);
}

#[test]
fn test_discover_repositories_depth_2() {
    let mock_fs = MockFilesystem::new(create_mock_directory_tree());

    let expected_uris: Vec<PathBuf> = vec![
        Path::new("root/nested_1").to_path_buf(),
        Path::new("root/depth_2/nested_2").to_path_buf(),
    ];
    let search_root = Path::new("root");
    let search_depth = 2;

    let locator = RepositoryLocator::new(mock_fs, GitRepoProbe, search_root, search_depth);

    let repositories = locator.locate();
    check_uris(repositories, expected_uris);
}

#[test]
fn test_discover_repositories_depth_3() {
    let mock_fs = MockFilesystem::new(create_mock_directory_tree());

    let expected_uris: Vec<PathBuf> = vec![
        Path::new("root/nested_1").to_path_buf(),
        Path::new("root/depth_2/nested_2").to_path_buf(),
        Path::new("root/depth_3/depth_3/nested_3").to_path_buf(),
    ];
    let search_root = Path::new("root");
    let search_depth = 3;

    let locator = RepositoryLocator::new(mock_fs, GitRepoProbe, search_root, search_depth);

    let repositories = locator.locate();
    check_uris(repositories, expected_uris);
}
