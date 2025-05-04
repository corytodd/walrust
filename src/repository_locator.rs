use std::fs;
use crate::filesystem::{Filesystem, RealFilesystem};
use std::path::{Path, PathBuf};

/// A struct to locate repositories in a given path
struct RepositoryLocator<F: Filesystem> {
    /// The filesystem to use for locating repositories
    filesystem: F,
    /// The root path to start the search from
    root: PathBuf,
    /// The maximum recursion depth for directory scanning
    depth: usize,
}

impl<F: Filesystem> RepositoryLocator<F> {
    /// Creates a new `RepositoryLocator`
    ///
    /// # Arguments
    /// * `filesystem` - An instance of a type that implements the `Filesystem` trait
    /// * `root` - A `Path` representing the root path to start the search from
    /// * `depth` - The maximum recursion depth for directory scanning
    ///
    /// # Returns
    /// * A new `RepositoryLocator` instance
    ///
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// use my_crate::repository_locator::{RepositoryLocator, RealFilesystem};
    /// let locator = RepositoryLocator::new(RealFilesystem, Path::new("/path/to/dir"), 2);
    /// ```
    pub fn new(filesystem: F, root: &Path, depth: usize) -> Self {
        RepositoryLocator {
            filesystem,
            root: root.to_path_buf(),
            depth,
        }
    }
    /// Discovers repositories in the given path
    ///
    /// # Returns
    /// * A vector of `PathBuf` representing the paths to discovered repositories
    ///
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// use my_crate::repository_locator::RepositoryLocatorLocal;
    /// let locator = RepositoryLocatorLocal::new(Path::new("/path/to/dir"), 2);
    /// let repositories = locator.discover();
    /// ```
    pub fn locate(&self) -> Vec<PathBuf> {
        self.locate_recursive(&self.root, self.depth + 1)
    }
    /// Discovers repositories in the given path with a specified depth
    ///
    ///
    /// # Arguments
    /// * `path` - A reference to a `Path` object
    /// * `depth` - The maximum recursion depth for directory scanning
    ///
    ///
    /// # Returns
    /// * A vector of `PathBuf` representing the paths to discovered repositories
    fn locate_recursive(&self, path: &Path, depth: usize) -> Vec<PathBuf> {
        let mut repositories = Vec::new();
        if depth == 0 {
            return repositories;
        }

        if self.filesystem.is_dir(path) {
            for entry in self.filesystem.read_dir(path).unwrap() {
                let entry_path = entry.as_path();
                if self.filesystem.is_dir(&entry_path) {
                    if entry_path.ends_with(".git") {
                        repositories.push(path.to_path_buf());
                    } else {
                        let sub_repositories = self.locate_recursive(&entry_path, depth - 1);
                        repositories.extend(sub_repositories);
                    }
                }
            }
        }
        repositories
    }
}

/// A struct to locate repositories in a local filesystem
pub struct RepositoryLocatorLocal {
    /// Locator for repositories on a local filesystem
    inner: RepositoryLocator<RealFilesystem>,
}

impl RepositoryLocatorLocal {
    /// Creates a new `RepositoryLocatorLocal`
    ///
    /// # Arguments
    /// * `path` - A reference to a `Path` object
    /// * `depth` - The maximum recursion depth for directory scanning
    ///
    /// # Returns
    /// * A new `RepositoryLocatorLocal` instance
    ///
    /// # Examples
    /// ```
    /// use std::path::Path;
    /// use walrust::repository_locator::RepositoryLocatorLocal;
    /// let locator = RepositoryLocatorLocal::new(Path::new("/path/to/dir"), 2);
    /// ```
    pub fn new(path: &Path, depth: usize) -> Self {
        let filesystem = RealFilesystem;
        let locator = RepositoryLocator::new(filesystem, path, depth);
        RepositoryLocatorLocal { inner: locator }
    }
    pub fn locate(&self) -> Vec<PathBuf> {
        self.inner.locate()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[derive(Debug)]
    pub enum MockFsNode {
        Directory(HashMap<String, MockFsNode>),
        File,
    }

    /// Creates a mock directory tree for testing.
    ///
    /// The tree structure is as follows:
    /// ```
    /// root
    /// ├── nested_1
    /// │   └── .git
    /// ├── not_a_repo
    /// ├── depth_2
    /// │   └── nested_2
    /// │       └── .git
    /// ├── depth_3
    /// │   └── depth_3
    /// │       └── nested_3
    /// │           └── .git
    /// └── depth_4
    ///     └── depth_4
    ///         └── depth_4
    ///             └── nested_4
    ///                 └── .git
    /// ```
    fn create_mock_directory_tree() -> MockFsNode {
        MockFsNode::Directory(HashMap::from([(
            "root".to_string(),
            MockFsNode::Directory(HashMap::from([
                (
                    "nested_1".to_string(),
                    MockFsNode::Directory(HashMap::from([(
                        ".git".to_string(),
                        MockFsNode::Directory(HashMap::new()),
                    )])),
                ),
                (
                    "not_a_repo".to_string(),
                    MockFsNode::Directory(HashMap::from([(
                        "file.txt".to_string(),
                        MockFsNode::File,
                    )])),
                ),
                (
                    "depth_2".to_string(),
                    MockFsNode::Directory(HashMap::from([(
                        "nested_2".to_string(),
                        MockFsNode::Directory(HashMap::from([(
                            ".git".to_string(),
                            MockFsNode::Directory(HashMap::new()),
                        )])),
                    )])),
                ),
                (
                    "depth_3".to_string(),
                    MockFsNode::Directory(HashMap::from([(
                        "depth_3".to_string(),
                        MockFsNode::Directory(HashMap::from([(
                            "nested_3".to_string(),
                            MockFsNode::Directory(HashMap::from([(
                                ".git".to_string(),
                                MockFsNode::Directory(HashMap::new()),
                            )])),
                        )])),
                    )])),
                ),
                (
                    "depth_4".to_string(),
                    MockFsNode::Directory(HashMap::from([(
                        "depth_4".to_string(),
                        MockFsNode::Directory(HashMap::from([(
                            "depth_4".to_string(),
                            MockFsNode::Directory(HashMap::from([(
                                "nested_4".to_string(),
                                MockFsNode::Directory(HashMap::from([(
                                    ".git".to_string(),
                                    MockFsNode::Directory(HashMap::new()),
                                )])),
                            )])),
                        )])),
                    )])),
                ),
            ])),
        )]))
    }

    struct MockFilesystem {
        root: MockFsNode,
    }

    impl MockFilesystem {
        pub fn new(root: MockFsNode) -> Self {
            Self { root }
        }

        /// Finds a node in the mock filesystem based on the given path.
        ///
        /// # Arguments
        /// * `path` - A reference to a `Path` object
        ///
        /// # Returns
        /// * An `Option` containing a reference to the `MockFsNode` if found, or `None` if not found
        fn find_node<'a>(&'a self, path: &Path) -> Option<&'a MockFsNode> {
            let mut current = &self.root;
            for component in path.iter() {
                if let MockFsNode::Directory(ref children) = current {
                    current = children.get(component.to_str().unwrap())?;
                } else {
                    return None;
                }
            }
            Some(current)
        }
    }

    impl Filesystem for MockFilesystem {
        fn is_dir(&self, path: &Path) -> bool {
            matches!(self.find_node(path), Some(MockFsNode::Directory(_)))
        }

        fn read_dir(&self, path: &Path) -> std::io::Result<Vec<PathBuf>> {
            if let Some(MockFsNode::Directory(children)) = self.find_node(path) {
                Ok(children
                    .keys()
                    .map(|name| {
                        let mut new_path = path.to_path_buf();
                        new_path.push(name);
                        new_path
                    })
                    .collect())
            } else {
                Err(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "Directory not found",
                ))
            }
        }
    }

    #[test]
    fn test_discover_repositories_depth_0() {
        let mock_fs = MockFilesystem::new(create_mock_directory_tree());
        let locator = RepositoryLocator::new(mock_fs, Path::new("root"), 0);

        let repositories = locator.locate();

        assert_eq!(repositories.len(), 0);
    }

    #[test]
    fn test_discover_repositories_depth_1() {
        let mock_fs = MockFilesystem::new(create_mock_directory_tree());
        let locator = RepositoryLocator::new(mock_fs, Path::new("root"), 1);

        let repositories = locator.locate();

        assert_eq!(repositories.len(), 1);
        assert!(repositories.contains(&Path::new("root/nested_1").to_path_buf()));
    }

    #[test]
    fn test_discover_repositories_depth_2() {
        let mock_fs = MockFilesystem::new(create_mock_directory_tree());
        let locator = RepositoryLocator::new(mock_fs, Path::new("root"), 2);

        let repositories = locator.locate();

        assert_eq!(repositories.len(), 2);
        assert!(repositories.contains(&Path::new("root/nested_1").to_path_buf()));
        assert!(repositories.contains(&Path::new("root/depth_2/nested_2").to_path_buf()));
    }

    #[test]
    fn test_discover_repositories_depth_3() {
        let mock_fs = MockFilesystem::new(create_mock_directory_tree());
        let locator = RepositoryLocator::new(mock_fs, Path::new("root"), 3);

        let repositories = locator.locate();

        assert_eq!(repositories.len(), 3);
        assert!(repositories.contains(&Path::new("root/nested_1").to_path_buf()));
        assert!(repositories.contains(&Path::new("root/depth_2/nested_2").to_path_buf()));
        assert!(repositories.contains(&Path::new("root/depth_3/depth_3/nested_3").to_path_buf()));
    }
}
