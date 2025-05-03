/// `Config` defines the runtime options for walrus.
///
/// ```
/// use walrust::config::Config;
///
/// let config = Config::default();
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = Config::default();
        assert_eq!(config.directory_scan_depth, 5);
    }

    #[test]
    fn test_new_config() {
        let config = Config::new(Some(10));
        assert_eq!(config.directory_scan_depth, 10);
    }

    #[test]
    fn test_new_config_with_default() {
        let config = Config::new(None);
        assert_eq!(config.directory_scan_depth, 5);
    }
}
