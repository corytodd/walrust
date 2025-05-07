use chrono::{DateTime, Local, NaiveDate, NaiveDateTime, ParseError, TimeZone};
use clap::Parser;
use log;
use std::path::PathBuf;
use std::process;
use walrust::commit::Commit;
use walrust::repository::GitRepository;
use walrust::repository_locator::GitRepositoryLocator;

/// The configuration for the `walrust` CLI tool.
///
/// This struct defines the command-line arguments and options for the tool,
/// including the search root, search depth, date filters, and author filters.
///
/// # Fields
/// - `search_root`: The root directory to start searching for repositories.
/// - `search_depth`: The maximum recursion depth for directory scanning.
/// - `since`: The starting date to filter commits (inclusive).
/// - `until`: The ending date to filter commits (inclusive).
/// - `author`: The author name to filter commits by.
///
/// # Example
/// ```bash
/// walrust -r /path/to/search -d 3 -a "John Doe <john.doe@example.com>"
/// ```
#[derive(Debug, Parser, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The root directory to start searching for repositories.
    #[arg(
        short = 'r',
        long,
        default_value = ".",
        value_hint = clap::ValueHint::DirPath,
        help = "Sets the root directory to search",
        required = true
    )]
    pub search_root: PathBuf,

    /// The maximum recursion depth for directory scanning relative to the
    /// starting directory.
    #[arg(
        short = 'd',
        long,
        default_value_t = 5,
        value_name = "DEPTH",
        value_hint = clap::ValueHint::Other,
        help = "Sets the depth of the search",
        required = true
    )]
    pub search_depth: usize,

    /// The starting date to filter commits (inclusive).
    #[arg(
        short = 'a',
        long,
        value_name = "SINCE",
        value_hint = clap::ValueHint::Other,
        help = "Filters commits since this date, inclusive. Defaults to yesterday's date."
    )]
    #[arg(
        short = 's',
        long,
        value_name = "SINCE",
        value_hint = clap::ValueHint::Other,
        value_parser = parse_datetime,
        help = "Filters commits since this date, inclusive. Defaults to yesterday's date."
    )]
    pub since: Option<DateTime<Local>>,

    /// The ending date to filter commits (inclusive).
    #[arg(
        short = 'u',
        long,
        value_name = "UNTIL",
        value_hint = clap::ValueHint::Other,
        value_parser = parse_datetime,
        help = "Filters commits until this date, inclusive."
    )]
    pub until: Option<DateTime<Local>>,

    /// The author name to filter commits by.
    #[arg(
        short = 'a',
        long,
        value_name = "AUTHOR",
        value_hint = clap::ValueHint::Other,
        help = "Filters commits by author in 'Name <email>' format"
    )]
    pub author: Option<String>,
}

/// Parses a string into a `chrono::DateTime<Local>` object.
///
/// This function attempts to parse the input string into a `DateTime` object
/// using common date-time formats. If the parsing fails, it returns an error.
///
/// # Arguments
/// - `s`: The input string to parse.
///
/// # Returns
/// A `Result` containing the parsed `DateTime<Local>` or a `ParseError`.
///
/// # Example
/// ```rust
/// let datetime = parse_datetime("2025-05-06").unwrap();
/// println!("{}", datetime); // Outputs: "2025-05-06 00:00:00 +00:00"
/// ```
fn parse_datetime(s: &str) -> Result<DateTime<Local>, ParseError> {
    DateTime::parse_from_rfc3339(s)
        .map(|dt| dt.with_timezone(&Local))
        .or_else(|_| {
            // Fallback to a custom format with both date and time (e.g., "YYYY-MM-DD HH:MM:SS").
            NaiveDateTime::parse_from_str(s, "%Y-%m-%d %H:%M:%S")
                .map(|naive| Local.from_local_datetime(&naive).unwrap())
        })
        .or_else(|_| {
            // Fallback to a date-only format (e.g., "YYYY-MM-DD").
            NaiveDate::parse_from_str(s, "%Y-%m-%d").map(|naive| {
                naive
                    .and_hms_opt(0, 0, 0)
                    .map(|naive| Local.from_local_datetime(&naive).unwrap())
                    .expect("Invalid date format. Try YYYY-MM-DD.")
            })
        })
}

/// Retrieves the default author name and email from the local Git configuration.
///
/// This function reads the `user.name` and `user.email` values from the local
/// Git configuration and combines them into a single string in the format
/// `"Name <email>"`.
///
/// # Returns
/// An optional string containing the author name and email. If the configuration
/// is not found or an error occurs, returns `None`.
///
/// # Example
/// ```rust
/// use walrust::bin::walrust::get_local_git_default_author;
///
/// if let Some(author) = get_local_git_default_author() {
///     println!("Default author: {}", author);
/// } else {
///     println!("No default author found.");
/// }
/// ```
fn get_local_git_default_author() -> Option<String> {
    let git_config = git2::Config::open_default().ok()?;
    let author_name = git_config.get_string("user.name").ok()?;
    let author_email = git_config.get_string("user.email").ok()?;
    Some(format!("{} <{}>", author_name, author_email))
}

/// Runs the repository locator with the given configuration.
///
/// This function initializes the repository locator, searches for repositories,
/// and filters commits based on the provided configuration. It prints the results
/// to the console.
///
/// # Arguments
/// - `config`: The configuration containing the search root, depth, date filters, and author filter.
///
/// # Returns
/// A `Result` indicating success or failure. If no repositories are found or an error occurs,
/// an error message is returned.
///
/// # Errors
/// - Returns an error if no repositories are found.
/// - Returns an error if commit retrieval fails.
///
/// # Example
/// ```rust
/// use walrust::bin::walrust::{Config, run};
/// use chrono::Utc;
///
/// let config = Config {
///     search_root: "/path/to/search".into(),
///     search_depth: 3,
///     since: Some(Utc::now() - chrono::Duration::days(1)),
///     until: Some(Utc::now()),
///     author: Some("John Doe <john.doe@example.com>".to_string()),
/// };
///
/// if let Err(err) = run(config) {
///     eprintln!("Error: {}", err);
/// }
/// ```
fn run(config: Config) -> Result<(), String> {
    let start_time = std::time::Instant::now();

    let locator = GitRepositoryLocator::new(&config.search_root, config.search_depth);
    let result = locator.locate();
    let repositories = match result {
        Ok(repositories) => repositories,
        Err(err) => {
            eprintln!("Error locating repositories: {}", err);
            return Err("No repositories found".to_string());
        }
    };

    let elapsed_time = start_time.elapsed();

    if repositories.is_empty() {
        return Err("No repositories found".to_string());
    }

    log::info!(
        "Found {} repositories in {:?}",
        repositories.len(),
        elapsed_time
    );

    let commits_since = config
        .since
        .unwrap_or_else(|| {
            let now = chrono::Utc::now() - chrono::Duration::hours(24);
            now.with_timezone(&Local)
        })
        .to_utc();

    let commits_until = config
        .until
        .unwrap_or_else(|| chrono::Utc::now().with_timezone(&Local))
        .to_utc();

    let author_match = config
        .author
        .unwrap_or(get_local_git_default_author().unwrap_or_default());

    let author_predicate = |commit: &Commit| match author_match {
        ref author if author.is_empty() => true,
        ref author if commit.author.to_string() == *author => true,
        _ => false,
    };

    log::info!(
        "Filtering commits since: {}, until: {}, by author: {}",
        commits_since.to_rfc3339(),
        commits_until.to_rfc3339(),
        author_match,
    );

    for git_repo in &repositories {
        log::info!(
            "Repository: {}, Name: {}, Head: {}",
            git_repo.get_uri().display(),
            git_repo.get_name(),
            git_repo.vcs.head()
        );

        let commits = git_repo.vcs.get_commits(commits_since, commits_until);
        match commits {
            Ok(commits) => {
                let filtered_commits = commits
                    .into_iter()
                    .filter(|commit| author_predicate(&commit))
                    .collect::<Vec<_>>();

                log::debug!("Matching Commit Count: {}", filtered_commits.len());

                for commit in &filtered_commits {
                    println!(
                        "{} {} {}",
                        commit.hash.short,
                        commit.commit_date.to_rfc3339(),
                        commit.title
                    );
                }
            }
            Err(err) => {
                eprintln!("Error getting commits: {}", err);
            }
        }
    }

    Ok(())
}

/// The main entry point for the `walrust` CLI tool.
///
/// This function parses the command-line arguments, runs the repository locator,
/// and handles any errors that occur.
///
/// # Example
/// ```bash
/// walrust -r /path/to/search -d 3 -a "John Doe <john.doe@example.com>"
/// ```
fn main() {
    let config = Config::parse();

    match run(config) {
        Ok(_) => {}
        Err(err) => {
            eprintln!("{}", err);
            process::exit(1);
        }
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;

    #[test]
    fn test_parse_datetime_rfc3339() {
        let input = "1996-12-19T16:39:57-08:00";
        let expected = Utc.with_ymd_and_hms(1996, 12, 20, 0, 39, 57).unwrap();
        let result = parse_datetime(input).unwrap().to_utc();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_datetime_custom_format_with_time() {
        let input = "2025-05-06 12:34:56";
        let expected = Local.with_ymd_and_hms(2025, 5, 6, 12, 34, 56).unwrap();
        let result = parse_datetime(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_datetime_custom_format_date_only() {
        let input = "2025-05-06";
        let expected = Local.with_ymd_and_hms(2025, 5, 6, 0, 0, 0).unwrap();
        let result = parse_datetime(input).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_parse_datetime_invalid_format() {
        let input = "invalid-date";
        let result = parse_datetime(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_datetime_empty_string() {
        let input = "";
        let result = parse_datetime(input);
        assert!(result.is_err());
    }

    #[test]
    fn test_parse_datetime_with_whitespace() {
        let input = " 2025-05-06 ";
        let expected = Local.with_ymd_and_hms(2025, 5, 6, 0, 0, 0).unwrap();
        let result = parse_datetime(input.trim()).unwrap();
        assert_eq!(result, expected);
    }

    #[test]
    fn test_run_no_repositories() {
        let config = Config {
            search_root: "/non/existent/path".into(),
            search_depth: 3,
            since: None,
            until: None,
            author: None,
        };

        let result = run(config);
        assert!(result.is_err());
        assert_eq!(result.unwrap_err(), "No repositories found".to_string());
    }

    #[test]
    fn test_config_invalid_arguments() {
        let args = vec!["walrust", "-r", "", "-d", "invalid-depth"];
        let result = Config::try_parse_from(args);
        assert!(result.is_err());
    }
}
