// src/error.rs
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ConfigError {
    #[error("IO error: {0}")]
    IoError(String),
    #[error("Parse error: {0}")]
    ParseError(String),
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(String),
    #[error("Missing required field: {0}")]
    MissingField(String),
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

#[derive(Error, Debug)]
pub enum TokenizerError {
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),
    #[error("Invalid token: {0}")]
    InvalidToken(String),
    #[error("Tokenization error: {0}")]
    TokenizationError(String),
}