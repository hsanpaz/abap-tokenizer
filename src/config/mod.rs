// abap-tokenizer/src/config/mod.rs
//! Configuration module for the ABAP Tokenizer.
//!
//! This module handles the loading, parsing, and management of tokenizer configurations.
//! It includes components for working with TOML configuration files and
//! for representing the tokenizer's configuration in memory.

/// Defines the structure and methods for the tokenizer's configuration.
pub(crate) mod tokenizer_config;

/// Handles the loading and parsing of TOML configuration files.
mod toml_loader;

/// Re-exports the TokenizerConfig and CompiledPatternConfig structs for use in other modules.
pub use tokenizer_config::{TokenizerConfig, CompiledPatternConfig};

/// Re-exports the load_toml_config function for loading TOML configuration files.
pub use toml_loader::load_toml_config;