use clap::Parser;
use std::path::PathBuf;
use std::process;
use walrust::repository::GitRepository;
use walrust::repository_locator::GitRepositoryLocator;

#[derive(Debug, Parser, PartialEq)]
#[command(version, about, long_about = None)]
pub struct Config {
    /// The root directory to start searching for repositories.
    #[arg(short = 'r', long, default_value = ".", value_hint = clap::ValueHint::DirPath,
        help = "Sets the root directory to search", required = true)]
    pub search_root: PathBuf,
    /// The maximum recursion depth for directory scanning relative to the
    /// starting directory.
    #[arg(short = 'd', long, default_value_t = 5, value_name = "DEPTH", value_hint = clap::ValueHint::Other,
        help = "Sets the depth of the search", required = true)]
    pub search_depth: usize,
}

/// Runs the repository locator with the given configuration.
///
/// # Arguments
/// * `config` - The configuration containing the search root and depth.
///
/// # Returns
/// A result containing a vector of paths to the found repositories or an error message.
///
/// # Errors
/// Returns an error if no repositories are found.
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

    println!(
        "Found {} repositories in {:?}",
        repositories.len(),
        elapsed_time
    );

    for git_repo in &repositories {
        println!("Repository: {}", git_repo.get_uri().display());
        println!("Name: {}", git_repo.get_name());
        println!("Head: {}", git_repo.vcs.head());
    }

    Ok(())
}

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
