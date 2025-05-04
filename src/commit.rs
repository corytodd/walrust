use chrono::{DateTime, Utc};

/// A repository commit.
#[derive(Debug, Clone, Default)]
#[cfg_attr(test, derive(PartialEq))]
pub struct Commit {
    /// Commit message title
    pub title: String,
    /// Commit message author name
    pub author_name: String,
    /// Commit message author email
    pub author_email: String,
    /// Date of commit
    pub commit_date: DateTime<Utc>,
    /// Commit message body
    pub message: String,
    /// Short commit hash
    pub hash_short: String,
    /// Full commit hash
    pub hash_full: String,
}

impl Commit {
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

    /// Returns the commit author in the format "Name <email>".
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
        static ref TEST_COMMIT_DATE: DateTime<Utc> =
            Utc.with_ymd_and_hms(2025, 5, 4, 11, 0, 0).unwrap();
    }

    #[test]
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
    fn test_new_commit() {
        let title = "Initial commit".to_string();
        let author_name = "Luthen Rael".to_string();
        let author_email = "<luthen.rael@totallynotarebel.com>".to_string();
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
