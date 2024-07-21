// abap-tokenizer/src/error.rs
//! Error types for the ABAP Tokenizer.
//!
//! This module defines custom error types used throughout the ABAP Tokenizer library.
//! It includes errors related to configuration loading and parsing, as well as
//! errors that can occur during the tokenization process.
use thiserror::Error;

/// Errors that can occur during configuration loading and parsing.
#[derive(Error, Debug)]
pub enum ConfigError {
    /// Error occurred while reading the configuration file.
    #[error("IO error: {0}")]
    IoError(String),

    /// Error occurred while parsing the configuration file.
    #[error("Parse error: {0}")]
    ParseError(String),

    /// The configuration contains an invalid regular expression.
    #[error("Invalid regex pattern: {0}")]
    InvalidRegex(String),

    /// A required field is missing in the configuration.
    #[error("Missing required field: {0}")]
    MissingField(String),

    /// A general configuration error occurred.
    #[error("Configuration error: {0}")]
    ConfigurationError(String),
}

/// Errors that can occur during the tokenization process.
#[derive(Error, Debug)]
pub enum TokenizerError {
    /// An error in the tokenizer configuration.
    #[error("Configuration error: {0}")]
    ConfigError(#[from] ConfigError),

    /// An unexpected character was encountered during tokenization.
    #[error("Unexpected character: {0}")]
    UnexpectedCharacter(char),

    /// An invalid token was encountered during tokenization.
    #[error("Invalid token: {0}")]
    InvalidToken(String),

    /// A general tokenization error occurred.
    #[error("Tokenization error: {0}")]
    TokenizationError(String),
}