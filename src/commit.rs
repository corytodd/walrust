use chrono::{DateTime, Utc};

/// A repository commit.
///
/// This struct represents a commit in a Git repository, including metadata
/// such as the commit message, author information, commit date, and hashes.
///
/// # Fields
/// - `title`: The title of the commit message.
/// - `author_name`: The name of the commit author.
/// - `author_email`: The email address of the commit author.
/// - `commit_date`: The date and time when the commit was created.
/// - `message`: The full commit message body.
/// - `hash_short`: The short version of the commit hash (e.g., first 7 characters).
/// - `hash_full`: The full commit hash.
///
/// # Example
/// ```rust
/// use walrust::commit::Commit;
/// use chrono::{Utc, TimeZone};
///
/// let commit = Commit {
///     title: "Initial commit".to_string(),
///     author_name: "Luthen Rael".to_string(),
///     author_email: "luthen.rael@totallynotarebel.com".to_string(),
///     commit_date: Utc.with_ymd_and_hms(2025, 5, 4, 11, 0, 0).unwrap(),
///     message: "Initial commit message".to_string(),
///     hash_short: "abc123".to_string(),
///     hash_full: "abc123def4567890".to_string(),
/// };
///
/// println!("Commit: {} by {} <{}>", commit.title, commit.author_name, commit.author_email);
/// ```
#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Commit {
    /// Commit message title.
    pub title: String,
    /// Commit message author name.
    pub author_name: String,
    /// Commit message author email.
    pub author_email: String,
    /// Date of the commit.
    pub commit_date: DateTime<Utc>,
    /// Commit message body.
    pub message: String,
    /// Short commit hash (e.g., first 7 characters of the full hash).
    pub hash_short: String,
    /// Full commit hash.
    pub hash_full: String,
}

impl Commit {
    /// Creates a new `Commit` instance.
    ///
    /// # Arguments
    /// - `title`: The title of the commit message.
    /// - `author_name`: The name of the commit author.
    /// - `author_email`: The email address of the commit author.
    /// - `commit_date`: The date and time when the commit was created.
    /// - `message`: The full commit message body.
    /// - `hash_short`: The short version of the commit hash.
    /// - `hash_full`: The full commit hash.
    ///
    /// # Returns
    /// A new `Commit` instance with the provided values.
    ///
    /// # Example
    /// ```rust
    /// use walrust::commit::Commit;
    /// use chrono::{Utc, TimeZone};
    ///
    /// let commit = Commit::new(
    ///     "Initial commit".to_string(),
    ///     "Luthen Rael".to_string(),
    ///     "luthen.rael@totallynotarebel.com".to_string(),
    ///     Utc.with_ymd_and_hms(2025, 5, 4, 11, 0, 0).unwrap(),
    ///     "Initial commit message".to_string(),
    ///     "abc123".to_string(),
    ///     "abc123def4567890".to_string(),
    /// );
    ///
    /// println!("Commit: {} by {} <{}>", commit.title, commit.author_name, commit.author_email);
    /// ```
    pub fn new(
        title: String,
        author_name: String,
        author_email: String,
        commit_date: DateTime<Utc>,
        message: String,
        hash_short: String,
        hash_full: String,
    ) -> Commit {
        Commit {
            title,
            author_name,
            author_email,
            commit_date,
            message,
            hash_short,
            hash_full,
        }
    }

    /// Returns the commit author in the format `"Name <email>"`.
    ///
    /// # Returns
    /// A string containing the author's name and email in the format `"Name <email>"`.
    ///
    /// # Example
    /// ```rust
    /// use walrust::commit::Commit;
    /// use chrono::{Utc, TimeZone};
    ///
    /// let commit = Commit::new(
    ///     "Initial commit".to_string(),
    ///     "Luthen Rael".to_string(),
    ///     "luthen.rael@totallynotarebel.com".to_string(),
    ///     Utc.with_ymd_and_hms(2025, 5, 4, 11, 0, 0).unwrap(),
    ///     "Initial commit message".to_string(),
    ///     "abc123".to_string(),
    ///     "abc123def4567890".to_string(),
    /// );
    ///
    /// assert_eq!(commit.author(), "Luthen Rael <luthen.rael@totallynotarebel.com>");
    /// ```
    pub fn author(&self) -> String {
        format!("{} <{}>", self.author_name, self.author_email)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::{DateTime, TimeZone, Utc};
    use lazy_static::lazy_static;

    lazy_static! {
        /// A static test commit date for use in tests.
        static ref TEST_COMMIT_DATE: DateTime<Utc> =
            Utc.with_ymd_and_hms(2025, 5, 4, 11, 0, 0).unwrap();
    }

    #[test]
    /// Tests the default implementation of the `Commit` struct.
    fn test_default_commit() {
        let commit = Commit::default();
        assert_eq!(commit.title, "");
        assert_eq!(commit.author_name, "");
        assert_eq!(commit.author_email, "");
        assert_eq!(commit.commit_date, DateTime::<Utc>::default());
        assert_eq!(commit.message, "");
        assert_eq!(commit.hash_short, "");
        assert_eq!(commit.hash_full, "");
    }

    #[test]
    /// Tests the `new` method of the `Commit` struct.
    fn test_new_commit() {
        let title = "Initial commit".to_string();
        let author_name = "Luthen Rael".to_string();
        let author_email = "luthen.rael@totallynotarebel.com".to_string();
        let message = "Initial commit message".to_string();
        let hash_short = "abc123".to_string();
        let hash_full = "abc123def4567890".to_string();
        let commit = Commit::new(
            title.clone(),
            author_name.clone(),
            author_email.clone(),
            *TEST_COMMIT_DATE,
            message.clone(),
            hash_short.clone(),
            hash_full.clone(),
        );

        assert_eq!(commit.title, title);
        assert_eq!(commit.author_name, author_name);
        assert_eq!(commit.author_email, author_email);
        assert_eq!(commit.commit_date, *TEST_COMMIT_DATE);
        assert_eq!(commit.message, message);
        assert_eq!(commit.hash_short, hash_short);
        assert_eq!(commit.hash_full, hash_full);
    }
}
