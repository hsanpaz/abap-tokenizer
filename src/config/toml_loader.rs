// abap-tokenizer\src\config\toml_loader.rs
use crate::error::ConfigError;
use crate::config::tokenizer_config::{RawTokenizerConfig, TokenizerConfig};
use std::fs;

pub fn load_toml_config(path: &str) -> Result<TokenizerConfig, ConfigError> {
    let content = fs::read_to_string(path)
        .map_err(|e| ConfigError::IoError(format!("Failed to read config file: {}", e)))?;
    
    let raw_config: RawTokenizerConfig = toml::from_str(&content)
        .map_err(|e| ConfigError::ParseError(format!("Failed to parse TOML: {}", e)))?;
    
    TokenizerConfig::from_raw(raw_config)
}