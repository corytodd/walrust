# Walrust

Walrust is a command-line tool designed to query commit data from multiple Git repositories simultaneously. It simplifies the process of gathering and analyzing commit history, making it easier for developers and teams to track changes across their projects.

This is a port of my [Walrus C# project][1], mostly just for fun.

## Features

- Query commit history across multiple Git repositories.
- Filter commits by date range, author, or repository depth.
- Supports custom repository locators and mock implementations for testing.
- Extensible and testable design with abstractions for filesystem and Git operations.

## Installation

To install Walrust, you need to have [Rust](https://www.rust-lang.org/) installed on your system. Once Rust is installed, you can build the project from source:

```bash
git clone https://github.com/corytodd/walrust.git
cd walrust
cargo build --release
```

The compiled binary will be available in the release directory.

## Usage

Walrust provides a simple command-line interface for querying commit data. Below are some examples of how to use it:

**Query Repositories in a Directory**
```
walrust -r /path/to/search -d 3
```

This command searches for Git repositories in /path/to/search up to a depth of 3 and retrieves commit data.



**Filter Commits by Date Range**

```
walrust -r /path/to/search -d 3 --since "2025-01-01T00:00:00Z" --until "2025-12-31T23:59:59Z"
```

This command retrieves commits made between January 1, 2025, and December 31, 2025.


**Filter Commits by Author**
```
walrust -r /path/to/search -d 3 --author "John Doe <john.doe@example.com>"
```

This command retrieves commits authored by "John Doe" in the specified repositories.

## Configuration

Walrust uses the following command-line options:

```
-r, --search-root: The root directory to start searching for repositories.
-d, --search-depth: The maximum recursion depth for directory scanning.
--since: The starting date to filter commits (inclusive).
--until: The ending date to filter commits (inclusive).
--author: The author name to filter commits by (in "Name <email>" format).
Run walrust --help for a full list of options.
```

## Development

### Running Tests

Walrust includes a comprehensive test suite to ensure code quality. To run the tests, use:

```
cargo test
```

## Contributing

Contributions are welcome! If you'd like to contribute to Walrust, please fork the repository and submit a pull request. Make sure to follow the coding standards and include tests for any new features.


## License

This project is licensed under the MIT License. See the LICENSE file for more details.

[1]: https://github.com/corytodd/Walrus