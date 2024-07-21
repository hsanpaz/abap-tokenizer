// abap-tokenizer/src/config/tokenizer_config.rs
//! Tokenizer configuration structures and implementations.
//!
//! This module defines the structures used to represent the tokenizer's configuration,
//! both in its raw form (as loaded from TOML) and in its compiled form (ready for use
//! by the tokenizer).
use crate::error::ConfigError;
use regex::Regex;
use serde::Deserialize;
use std::{any::Any, collections::HashMap};

/// Raw configuration structure as loaded from TOML.
#[derive(Deserialize)]
pub struct RawTokenizerConfig {
    /// General metadata about the tokenizer configuration.
    pub metadata: Metadata,
    /// Definitions of token categories and their properties.
    pub token_categories: HashMap<String, CategoryConfig>,
    /// Raw pattern definitions for each token category.
    pub patterns: HashMap<String, Vec<RawPatternConfig>>,
    /// Rules for handling context-sensitive tokens.
    pub context_rules: HashMap<String, ContextRule>,
    /// Definitions of custom actions to be taken for certain token types.
    pub custom_actions: HashMap<String, CustomAction>,
    /// Optional list of configuration files to import.
    pub imports: Option<Vec<String>>,
    /// Special rules for handling specific token patterns.
    pub special_rules: Vec<SpecialRule>,
}


/// Raw pattern configuration as defined in TOML.
#[derive(Debug, Deserialize)]
pub struct RawPatternConfig {
    /// Regular expression string for matching the token.
    pub regex: String,
    /// Optional subcategory for finer-grained token classification.
    pub subcategory: Option<String>,
}

/// Compiled pattern configuration ready for use by the tokenizer.
#[derive(Debug)]
pub struct CompiledPatternConfig {
    /// Compiled regular expression for efficient token matching.
    pub regex: Regex,
    /// Optional subcategory for finer-grained token classification.
    pub subcategory: Option<String>,
}

/// Main tokenizer configuration structure.
pub struct TokenizerConfig {
    /// General metadata about the tokenizer configuration.
    pub metadata: Metadata,
    /// Definitions of token categories and their properties.
    pub token_categories: HashMap<String, CategoryConfig>,
    /// Compiled pattern definitions for each token category.
    pub patterns: HashMap<String, Vec<CompiledPatternConfig>>,
    /// Rules for handling context-sensitive tokens.
    pub context_rules: HashMap<String, ContextRule>,
    /// Definitions of custom actions to be taken for certain token types.
    pub custom_actions: HashMap<String, CustomAction>,
    /// Optional list of configuration files to import.
    pub imports: Option<Vec<String>>,
    /// Special rules for handling specific token patterns.
    pub special_rules: Vec<SpecialRule>,
}

/// Metadata for the tokenizer configuration.
#[derive(Debug, Deserialize)]
pub struct Metadata {
    /// Version of the ABAP language this configuration targets.
    pub language_version: String,
    /// Whether the tokenizer should be case-sensitive.
    pub case_sensitive: bool,
    /// Whether to allow Unicode characters in identifiers.
    pub allow_unicode_identifiers: bool,
}

/// Configuration for a token category.
#[derive(Debug, Deserialize)]
pub struct CategoryConfig {
    /// Priority of the category for resolving conflicts.
    pub priority: u32,
    /// Color associated with the category (e.g., for syntax highlighting).
    pub color: String,
}

/// Rule for handling context-sensitive tokens.
#[derive(Debug, Deserialize)]
pub struct ContextRule {
    /// String that marks the start of the context.
    pub start: String,
    /// String that marks the end of the context.
    pub end: String,
    /// Optional escape sequence within the context.
    pub escape: Option<String>,
    /// Whether the context can span multiple lines.
    pub multiline: Option<bool>,
}

/// Definition of a custom action for specific token types.
#[derive(Debug, Deserialize)]
pub struct CustomAction {
    /// Name of the action to be performed.
    pub action: String,
    /// Optional arguments for the action.
    pub args: Option<HashMap<String, String>>,
}

/// Raw representation of a special rule.
#[derive(Deserialize, Clone)]
pub struct RawSpecialRule {
    /// Name of the special rule.
    pub name: String,
    /// String that marks the start of the special pattern.
    pub start: String,
    /// Optional string that marks the end of the special pattern.
    pub end: Option<String>,
    /// Optional column where the pattern must start to be valid.
    pub start_column: Option<usize>,
    /// Optional minimum length for the pattern to be valid.
    pub min_length: Option<usize>,
    /// Optional regex for additional pattern matching.
    pub regex: Option<String>,
    /// Type of token to be created when this rule matches.
    pub token_type: String,
}

/// Compiled representation of a special rule.
#[derive(Deserialize, Clone)]
pub struct SpecialRule {
    /// String that marks the start of the special pattern.
    pub start: String,
    /// Optional string that marks the end of the special pattern.
    pub end: Option<String>,
    /// Optional column where the pattern must start to be valid.
    pub start_column: Option<usize>,
    /// Optional minimum length for the pattern to be valid.
    pub min_length: Option<usize>,
    /// Optional regex for additional pattern matching.
    pub regex: Option<String>,
    /// Type of token to be created when this rule matches.
    pub token_type: String,
}

impl TokenizerConfig {
    /// Merges another TokenizerConfig into this one.
    ///
    /// This method is useful for combining multiple configuration files.
    ///
    /// # Arguments
    ///
    /// * `other` - Another TokenizerConfig to merge into this one
    ///
    /// # Returns
    ///
    /// * `Result<(), ConfigError>` - Ok if the merge was successful, Err otherwise
    pub fn merge(&mut self, other: TokenizerConfig) -> Result<(), ConfigError> {
        // Merge token categories
        for (category, config) in other.token_categories {
            self.token_categories.entry(category).or_insert(config);
        }

        // Merge patterns
        for (category, patterns) in other.patterns {
            self.patterns
                .entry(category)
                .or_insert_with(Vec::new)
                .extend(patterns);
        }

        // Merge context rules
        self.context_rules.extend(other.context_rules);

        // Merge custom actions
        self.custom_actions.extend(other.custom_actions);

        Ok(())
    }
    
    /// Creates a TokenizerConfig from a RawTokenizerConfig.
    ///
    /// This method compiles the raw configuration, including compiling regex patterns.
    ///
    /// # Arguments
    ///
    /// * `raw_config` - A RawTokenizerConfig to convert
    ///
    /// # Returns
    ///
    /// * `Result<Self, ConfigError>` - The compiled TokenizerConfig or an error
    pub fn from_raw(raw_config: RawTokenizerConfig) -> Result<Self, ConfigError> {
        let mut patterns = HashMap::new();

        for (category, raw_patterns) in raw_config.patterns {
            let mut compiled_patterns = Vec::new();
            for raw_pattern in raw_patterns {
                let compiled_regex = Regex::new(&raw_pattern.regex)
                    .map_err(|_| ConfigError::InvalidRegex(raw_pattern.regex.clone()))?;
                compiled_patterns.push(CompiledPatternConfig {
                    regex: compiled_regex,
                    subcategory: raw_pattern.subcategory,
                });
            }
            patterns.insert(category, compiled_patterns);
        }
        
        let special_rules = raw_config.special_rules
            .into_iter()
            .map(|rule| SpecialRule {
                start: rule.start,
                end: rule.end,
                start_column: rule.start_column,
                min_length: rule.min_length,
                regex: rule.regex,
                token_type: rule.token_type,
            })
            .collect();

        // Validar campos requeridos
        if raw_config.metadata.language_version.is_empty() {
            return Err(ConfigError::MissingField("language_version".to_string()));
        }

        Ok(TokenizerConfig {
            metadata: raw_config.metadata,
            token_categories: raw_config.token_categories,
            patterns,
            context_rules: raw_config.context_rules,
            custom_actions: raw_config.custom_actions,
            imports: raw_config.imports,
            special_rules,
        })
    }

    /// Retrieves the compiled patterns for a given category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category to retrieve patterns for
    ///
    /// # Returns
    ///
    /// * `Option<&Vec<CompiledPatternConfig>>` - The patterns if the category exists
    pub fn get_pattern(&self, category: &str) -> Option<&Vec<CompiledPatternConfig>> {
        self.patterns.get(category)
    }

    /// Adds a new compiled pattern to a category.
    ///
    /// # Arguments
    ///
    /// * `category` - The category to add the pattern to
    /// * `pattern` - The compiled pattern to add
    pub fn add_pattern(&mut self, category: String, pattern: CompiledPatternConfig) {
        self.patterns
            .entry(category)
            .or_insert_with(Vec::new)
            .push(pattern);
    }

    /// Retrieves a context rule by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the context rule
    ///
    /// # Returns
    ///
    /// * `Option<&ContextRule>` - The context rule if it exists
    pub fn get_context_rule(&self, name: &str) -> Option<&ContextRule> {
        self.context_rules.get(name)
    }

    /// Adds a new context rule.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the context rule
    /// * `rule` - The context rule to add
    pub fn add_context_rule(&mut self, name: String, rule: ContextRule) {
        self.context_rules.insert(name, rule);
    }

    /// Retrieves a custom action by name.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the custom action
    ///
    /// # Returns
    ///
    /// * `Option<&CustomAction>` - The custom action if it exists
    pub fn get_custom_action(&self, name: &str) -> Option<&CustomAction> {
        self.custom_actions.get(name)
    }

    /// Adds a new custom action.
    ///
    /// # Arguments
    ///
    /// * `name` - The name of the custom action
    /// * `action` - The custom action to add
    pub fn add_custom_action(&mut self, name: String, action: CustomAction) {
        self.custom_actions.insert(name, action);
    }
}


impl SpecialRule {
    /// Retrieves an attribute of the SpecialRule.
    ///
    /// This method allows flexible access to the rule's attributes,
    /// returning them as trait objects that implement Any.
    ///
    /// # Arguments
    ///
    /// * `attr` - The name of the attribute to retrieve
    ///
    /// # Returns
    ///
    /// * `Option<&dyn Any>` - The attribute value if it exists, wrapped in Any
    pub fn get_attribute(&self, attr: &str) -> Option<&dyn Any> {
        match attr {
            "start" => Some(&self.start),
            "end" => self.end.as_ref().map(|v| v as &dyn Any),
            "regex" => self.regex.as_ref().map(|v| v as &dyn Any),
            "min_length" => self.min_length.as_ref().map(|v| v as &dyn Any),
            "start_column" => self.start_column.as_ref().map(|v| v as &dyn Any),
            _ => None,
        }
    }
}