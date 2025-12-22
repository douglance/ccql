use thiserror::Error;

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("JSON parsing error: {0}")]
    JsonParse(#[from] serde_json::Error),

    #[error("Query parse error: {0}")]
    QueryParse(String),

    #[error("Query execution error: {0}")]
    QueryExecution(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Data source error: {0}")]
    DataSource(String),

    #[error("Invalid path: {0}")]
    InvalidPath(String),

    #[error("File not found: {0}")]
    FileNotFound(String),

    #[error("Regex error: {0}")]
    Regex(#[from] regex::Error),
}
