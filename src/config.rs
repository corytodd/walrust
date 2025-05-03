/// `Config` defines the runtime options for walrus.
///
/// ```
/// use walrust::config::Config;
///
/// let config = Config::new();
/// println!("{:?}", config);
/// ```
#[derive(Debug, PartialEq)]
pub struct Config {
    /// The maximum recursion depth for directory scanning relative to the
    /// starting directory.
    pub directory_scan_depth: usize,
}

impl Default for Config {
    /// Creates a `Config` instance with default settings.
    ///
    /// The default configuration sets:
    /// - `directory_scan_depth` to `5`, which is a reasonable default for most use cases.
    ///
    /// # Examples
    ///
    /// ```
    /// use walrust::config::Config;
    ///
    /// let config = Config::default();
    /// assert_eq!(config.directory_scan_depth, 5);
    /// ```
    fn default() -> Self {
        Config {
            directory_scan_depth: 5,
        }
    }
}

impl Config {
    /// Creates a new `Config` instance with default settings.
    ///
    /// This is equivalent to calling `Config::default()`.
    ///
    /// # Examples
    ///
    /// ```
    /// use walrust::config::Config;
    ///
    /// let config = Config::new(None);
    /// assert_eq!(config.directory_scan_depth, 5);
    /// ```
    pub fn new(directory_scan_depth: Option<usize>) -> Self {
        let default_config = Config::default();
        Config {
            directory_scan_depth: directory_scan_depth
                .unwrap_or_else(|| default_config.directory_scan_depth),
        }
    }
}
