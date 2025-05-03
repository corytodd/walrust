/// `Config` defines the runtime options for walrus.
///
/// ```
/// use walrust::config::Config;
///
/// let config = Config::new();
/// println!("{:?}", config);
/// ```
#[derive(Debug, PartialEq)]
pub struct Config;

impl Config {
    /// Creates a new `Config` instance with default settings.
    ///
    /// # Examples
    ///
    /// ```
    /// use walrust::config::Config;
    ///
    /// let config = Config::new();
    /// assert_eq!(config, Config::new());
    /// ```
    pub fn new() -> Self {
        Config
    }
}
