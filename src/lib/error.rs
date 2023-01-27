//! Error and Result structures used all across this crate.

/// Enumeration of all possible error types.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Error from reading and writing to IO (see [`std::io::Error`]).
    #[error(transparent)]
    IO(#[from] std::io::Error),

    /// Error from parsing category code.
    #[error("invalid category code (got '{0}', must be between 0 and 15 included)")]
    InvalidCategoryCode(String),

    /// Error from checking if `directory` exists and is a actually a directory.
    #[error("invalid directory (got '{0}', does not exist or is not a directory)")]
    InvalidDirectory(String),

    /// Error from checking if `filename` exists and is a actualla a file.
    #[error("invalid filename (got '{0}', does not exist or is not a file)")]
    InvalidFilename(String),
}

/// Result type alias with error type defined above (see [`Error`]).
pub type Result<T> = std::result::Result<T, Error>;
