use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walrust::filesystem::Filesystem;

/// A node in the mock filesystem.
///
/// This enum represents either a directory or a file in the mock filesystem.
/// It is used to simulate a filesystem structure for testing purposes.
#[derive(Debug)]
pub enum MockFsNode {
    /// A directory containing child nodes.
    Directory(HashMap<String, MockFsNode>),
    /// A file node.
    File,
}

/// Creates a mock directory tree for testing.
///
/// The tree structure is as follows:
/// ```text
/// root
/// ├── nested_1
/// │   └── .git
/// ├── not_a_repo
/// │   └── file.txt
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
///
/// # Returns
/// A `MockFsNode` representing the root of the mock directory tree.
///
/// # Example
/// ```rust
/// use walrust::tests::mock_filesystem::{create_mock_directory_tree, MockFsNode};
///
/// let root = create_mock_directory_tree();
/// if let MockFsNode::Directory(children) = root {
///     assert!(children.contains_key("nested_1"));
/// }
/// ```
pub fn create_mock_directory_tree() -> MockFsNode {
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
                MockFsNode::Directory(HashMap::from([("file.txt".to_string(), MockFsNode::File)])),
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

/// A mock implementation of the `Filesystem` trait for testing purposes.
///
/// This struct simulates a filesystem using an in-memory tree structure.
/// It allows you to test functionality that interacts with the filesystem
/// without requiring actual filesystem operations.
pub struct MockFilesystem {
    /// The root node of the mock filesystem.
    root: MockFsNode,
}

impl MockFilesystem {
    /// Creates a new `MockFilesystem` with the given root node.
    ///
    /// # Arguments
    /// - `root`: The root node of the mock filesystem.
    ///
    /// # Returns
    /// A new instance of `MockFilesystem`.
    ///
    /// # Example
    /// ```rust
    /// use walrust::tests::mock_filesystem::{MockFilesystem, create_mock_directory_tree};
    ///
    /// let root = create_mock_directory_tree();
    /// let fs = MockFilesystem::new(root);
    /// ```
    pub fn new(root: MockFsNode) -> Self {
        Self { root }
    }

    /// Finds a node in the mock filesystem based on the given path.
    ///
    /// # Arguments
    /// - `path`: A reference to a `Path` object representing the path to search for.
    ///
    /// # Returns
    /// An `Option` containing a reference to the `MockFsNode` if found, or `None` if not found.
    ///
    /// # Example
    /// ```rust
    /// use walrust::tests::mock_filesystem::{MockFilesystem, create_mock_directory_tree};
    /// use std::path::Path;
    ///
    /// let fs = MockFilesystem::new(create_mock_directory_tree());
    /// let node = fs.find_node(Path::new("/root/nested_1/.git"));
    /// assert!(node.is_some());
    /// ```
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
    /// Creates a new `MockFilesystem` with a default mock directory tree.
    ///
    /// # Returns
    /// A new instance of `MockFilesystem`.
    ///
    /// # Example
    /// ```rust
    /// use walrust::filesystem::Filesystem;
    /// use walrust::tests::mock_filesystem::MockFilesystem;
    ///
    /// let fs = MockFilesystem::new();
    /// assert!(fs.is_dir(std::path::Path::new("/root")));
    /// ```
    fn new() -> Self {
        Self::new(create_mock_directory_tree())
    }

    /// Checks if the given path is a directory in the mock filesystem.
    ///
    /// # Arguments
    /// - `path`: A reference to a `Path` object representing the path to check.
    ///
    /// # Returns
    /// `true` if the path is a directory, `false` otherwise.
    ///
    /// # Example
    /// ```rust
    /// use walrust::filesystem::Filesystem;
    /// use walrust::tests::mock_filesystem::MockFilesystem;
    /// use std::path::Path;
    ///
    /// let fs = MockFilesystem::new();
    /// assert!(fs.is_dir(Path::new("/root/nested_1")));
    /// ```
    fn is_dir(&self, path: &Path) -> bool {
        matches!(self.find_node(path), Some(MockFsNode::Directory(_)))
    }

    /// Reads the contents of a directory in the mock filesystem.
    ///
    /// # Arguments
    /// - `path`: A reference to a `Path` object representing the directory to read.
    ///
    /// # Returns
    /// A `Result` containing a vector of `PathBuf` objects representing the contents
    /// of the directory, or an error if the directory does not exist.
    ///
    /// # Errors
    /// Returns an error if the directory is not found or is not a directory.
    ///
    /// # Example
    /// ```rust
    /// use walrust::filesystem::Filesystem;
    /// use walrust::tests::mock_filesystem::MockFilesystem;
    /// use std::path::Path;
    ///
    /// let fs = MockFilesystem::new();
    /// let entries = fs.read_dir(Path::new("/root")).unwrap();
    /// assert!(entries.iter().any(|entry| entry.ends_with("nested_1")));
    /// ```
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
