use chrono::{DateTime, Utc};

/// Represents the author of a commit.
///
/// This struct encapsulates the author's name and email address.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommitAuthor {
    /// The name of the author.
    pub name: String,
    /// The email address of the author.
    pub email: String,
}

impl CommitAuthor {
    /// Creates a new `CommitAuthor` instance.
    ///
    /// # Arguments
    /// - `name`: The name of the author.
    /// - `email`: The email address of the author.
    ///
    /// # Returns
    /// A new `CommitAuthor` instance.
    pub fn new(name: String, email: String) -> Self {
        Self { name, email }
    }

    /// Returns the author in the format `"Name <email>"`.
    ///
    /// This method checks if both the name and email are empty. If they are, it returns an empty string.
    ///
    /// # Returns
    /// A string representing the author in the format `"Name <email>"`, or an empty string if both fields are empty.
    pub fn to_string(&self) -> String {
        let email = match self.email.is_empty() {
            true => String::new(),
            false => format!("<{}>", self.email),
        };
        let name = match self.name.is_empty() {
            true => String::new(),
            false => format!("{}", self.name),
        };
        // If both name and email are empty, return an empty string
        match (name.is_empty(), email.is_empty()) {
            (true, true) => String::new(),
            _ => format!("{} {}", name, email).trim().to_string(),
        }
    }
}

/// Represents a commit hash.
///
/// This struct encapsulates both the short and full representations of a commit hash.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct CommitHash {
    /// The short version of the hash (e.g., first 7 characters).
    pub short: String,
    /// The full version of the hash.
    pub full: String,
}

impl CommitHash {
    /// Creates a new `CommitHash` instance.
    ///
    /// # Arguments
    /// - `full_hash`: The full version of the commit hash.
    ///
    /// # Returns
    /// A new `CommitHash` instance with both the short and full hash representations.
    pub fn new(full_hash: String) -> Self {
        Self {
            short: full_hash.clone()[..7].to_string(),
            full: full_hash,
        }
    }
}

/// Represents a Git commit.
///
/// This struct encapsulates metadata about a commit, including its title, author,
/// date, message, and hash.
#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Commit {
    /// The title of the commit message.
    pub title: String,
    /// The author of the commit.
    pub author: CommitAuthor,
    /// The date and time of the commit.
    pub commit_date: DateTime<Utc>,
    /// The full commit message body.
    pub message: String,
    /// The hash of the commit.
    pub hash: CommitHash,
}

impl Commit {
    /// Creates a new `Commit` instance.
    ///
    /// # Arguments
    /// - `title`: The title of the commit message.
    /// - `author`: The author of the commit.
    /// - `commit_date`: The date and time when the commit was created.
    /// - `message`: The full commit message body.
    /// - `hash`: The hash of the commit.
    ///
    /// # Returns
    /// A new `Commit` instance with the provided values.
    pub fn new(
        title: String,
        author: CommitAuthor,
        commit_date: DateTime<Utc>,
        message: String,
        hash: CommitHash,
    ) -> Commit {
        Commit {
            title,
            author,
            commit_date,
            message,
            hash,
        }
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
    ///
    /// This test verifies that the default values for a `Commit` instance are as expected.
    fn test_default_commit() {
        let commit = Commit::default();
        assert_eq!(commit.title, "");
        assert_eq!(commit.author.to_string(), "");
        assert_eq!(commit.commit_date, DateTime::<Utc>::default());
        assert_eq!(commit.message, "");
        assert_eq!(commit.hash.full, "");
    }

    #[test]
    /// Tests the `new` method of the `Commit` struct.
    ///
    /// This test verifies that a `Commit` instance is correctly initialized with the provided values.
    fn test_new_commit() {
        let title = "Initial commit".to_string();
        let author = CommitAuthor::new(
            "Luthen Rael".to_string(),
            "luthen.rael@totallynotarebel.com".to_string(),
        );
        let message = "Initial commit message".to_string();
        let hash_full = "abc123def4567890".to_string();
        let hash = CommitHash::new(hash_full.clone());
        let commit = Commit::new(
            title.clone(),
            author.clone(),
            *TEST_COMMIT_DATE,
            message.clone(),
            hash.clone(),
        );

        assert_eq!(commit.title, title);
        assert_eq!(commit.author, author);
        assert_eq!(commit.commit_date, *TEST_COMMIT_DATE);
        assert_eq!(commit.message, message);
        assert_eq!(commit.hash, hash);
    }

    #[test]
    /// Tests the `to_string` method of the `CommitAuthor` struct.
    ///
    /// This test verifies that the `to_string` method correctly formats the author's name and email.
    fn test_commit_author_to_string() {
        let author = CommitAuthor::new(
            "Cassian Andor".to_string(),
            "cassian.andor@rebellion.com".to_string(),
        );
        assert_eq!(
            author.to_string(),
            "Cassian Andor <cassian.andor@rebellion.com>"
        );

        let author_empty_name =
            CommitAuthor::new("".to_string(), "cassian.andor@rebellion.com".to_string());
        assert_eq!(
            author_empty_name.to_string(),
            "<cassian.andor@rebellion.com>"
        );

        let author_empty_email = CommitAuthor::new("Cassian Andor".to_string(), "".to_string());
        assert_eq!(author_empty_email.to_string(), "Cassian Andor");

        let author_empty_both = CommitAuthor::new("".to_string(), "".to_string());
        assert_eq!(author_empty_both.to_string(), "");
    }

    #[test]
    /// Tests the `new` method of the `CommitHash` struct.
    ///
    /// This test verifies that a `CommitHash` instance is correctly initialized with the provided full hash.
    fn test_commit_hash_new() {
        let full_hash = "1234567890abcdef".to_string();
        let hash = CommitHash::new(full_hash.clone());
        assert_eq!(hash.full, full_hash);
        assert_eq!(hash.short, "1234567");
    }

    #[test]
    /// Tests the `new` method of the `CommitAuthor` struct.
    ///
    /// This test verifies that a `CommitAuthor` instance is correctly initialized with the provided name and email.
    fn test_commit_author_new() {
        let name = "Mon Mothma".to_string();
        let email = "mon.mothma@rebellion.com".to_string();
        let author = CommitAuthor::new(name.clone(), email.clone());
        assert_eq!(author.name, name);
        assert_eq!(author.email, email);
    }

    #[test]
    /// Tests the `new` method of the `CommitHash` struct with a short hash.
    ///
    /// This test ensures that the short hash is correctly derived from the full hash.
    fn test_commit_hash_short() {
        let full_hash = "abcdef1234567890".to_string();
        let hash = CommitHash::new(full_hash.clone());
        assert_eq!(hash.short, "abcdef1");
    }

    #[test]
    /// Tests the `new` method of the `Commit` struct with empty fields.
    ///
    /// This test ensures that a `Commit` instance can be created with empty fields.
    fn test_commit_new_empty_fields() {
        let commit = Commit::new(
            "".to_string(),
            CommitAuthor::default(),
            DateTime::<Utc>::default(),
            "".to_string(),
            CommitHash::default(),
        );

        assert_eq!(commit.title, "");
        assert_eq!(commit.author, CommitAuthor::default());
        assert_eq!(commit.commit_date, DateTime::<Utc>::default());
        assert_eq!(commit.message, "");
        assert_eq!(commit.hash, CommitHash::default());
    }
}
