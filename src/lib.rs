// abap-tokenizer/src/lib.rs
//! # ABAP Tokenizer Library
//!
//! This library provides a flexible tokenizer for the ABAP programming language.
//! It uses a configuration-driven approach, allowing users to customize the 
//! tokenization process through TOML configuration files.
//!
//! ## Main components:
//! - `error`: Defines custom error types for configuration and tokenization.
//! - `config`: Handles loading and parsing of tokenizer configurations.
//! - `tokenizer`: Contains the core tokenization logic.
//!
//! ## Usage
//! To use this library, typically you would:
//! 1. Load a configuration using `config::load_toml_config`
//! 2. Create a `FlexibleTokenizer` instance
//! 3. Use the `next_token` method to tokenize your code
//!
//! ### Example:
//! ```
//! use abap_tokenizer::config::load_toml_config;
//! use abap_tokenizer::tokenizer::flexible_tokenizer::FlexibleTokenizer;
//!
//! let config = load_toml_config("path/to/config.toml")?;
//! let mut tokenizer = FlexibleTokenizer::new(input, config);
//! while let Ok(Some(token)) = tokenizer.next_token() {
//!     println!("{:?}", token);
//! }
//! ```

/// Error handling module for the ABAP Tokenizer.
pub mod error;

/// Configuration handling module for the ABAP Tokenizer.
pub mod config;

/// Core tokenization module for the ABAP Tokenizer.
pub mod tokenizer;

/// Re-export of error types for easier access by users of this library.
pub use error::{ConfigError, TokenizerError};