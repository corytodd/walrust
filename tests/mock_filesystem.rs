use std::collections::HashMap;
use std::path::{Path, PathBuf};
use walrust::filesystem::Filesystem;

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

pub struct MockFilesystem {
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
