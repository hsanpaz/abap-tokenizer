// abap-tokenizer/src/config/toml_loader.rs
//! TOML configuration loader for the ABAP Tokenizer.
//!
//! This module provides functionality to load and parse TOML configuration files
//! for the ABAP Tokenizer. It handles the initial loading of the raw configuration
//! and its conversion into a usable TokenizerConfig structure.
use crate::error::ConfigError;
use crate::config::tokenizer_config::{RawTokenizerConfig, TokenizerConfig};
use std::fs;


/// Loads and parses a TOML configuration file for the ABAP Tokenizer.
///
/// This function performs the following steps:
/// 1. Reads the TOML file from the given path.
/// 2. Parses the TOML content into a RawTokenizerConfig structure.
/// 3. Converts the RawTokenizerConfig into a fully initialized TokenizerConfig.
///
/// # Arguments
///
/// * `path` - A string slice that holds the path to the TOML configuration file
///
/// # Returns
///
/// * `Result<TokenizerConfig, ConfigError>` - The parsed and initialized TokenizerConfig on success,
///    or a ConfigError if any step of the loading process fails.
///
/// # Errors
///
/// This function will return an error if:
/// * The file cannot be read (e.g., file not found, permissions issues)
/// * The TOML content is invalid or cannot be parsed
/// * The conversion from RawTokenizerConfig to TokenizerConfig fails (e.g., invalid regex patterns)

pub fn load_toml_config(path: &str) -> Result<TokenizerConfig, ConfigError> {
    // Read the content of the TOML file
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::IoError(format!("Failed to read config file: {}", e)))?;
    // Parse the TOML content into a RawTokenizerConfig
    let raw_config: RawTokenizerConfig = toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError(format!("Failed to parse TOML: {}", e)))?;
    // Convert the RawTokenizerConfig into a TokenizerConfig
    TokenizerConfig::from_raw(raw_config)
}