// abap-tokenizer\src\config\tokenizer_config.rs
use regex::Regex;
use serde::Deserialize;
use std::collections::HashMap;
use crate::error::ConfigError;
//use crate::error::ConfigError;



#[derive(Deserialize)]
pub struct RawTokenizerConfig {
    pub metadata: Metadata,
    pub token_categories: HashMap<String, CategoryConfig>,
    pub patterns: HashMap<String, Vec<RawPatternConfig>>,
    pub context_rules: HashMap<String, ContextRule>,
    pub custom_actions: HashMap<String, CustomAction>,
    pub imports: Option<Vec<String>>,
}


#[derive(Debug, Deserialize)]
pub struct RawPatternConfig {
    pub regex: String,
    pub subcategory: Option<String>,
}

#[derive(Debug)]
pub struct CompiledPatternConfig {
    pub regex: Regex,
    pub subcategory: Option<String>,
}


pub struct TokenizerConfig {
    pub metadata: Metadata,
    pub token_categories: HashMap<String, CategoryConfig>,
    pub patterns: HashMap<String, Vec<CompiledPatternConfig>>,
    pub context_rules: HashMap<String, ContextRule>,
    pub custom_actions: HashMap<String, CustomAction>,
    pub imports: Option<Vec<String>>,
}


#[derive(Debug, Deserialize)]
pub struct Metadata {
    pub language_version: String,
    pub case_sensitive: bool,
    pub allow_unicode_identifiers: bool,
}

#[derive(Debug, Deserialize)]
pub struct CategoryConfig {
    pub priority: u32,
    pub color: String,
}

#[derive(Debug, Deserialize)]
pub struct PatternConfig {
    pub regex: String,
    pub subcategory: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct ContextRule {
    pub start: String,
    pub end: String,
    pub escape: Option<String>,
    pub multiline: Option<bool>,
}

#[derive(Debug, Deserialize)]
pub struct CustomAction {
    pub action: String,
    pub args: Option<HashMap<String, String>>,
}

impl TokenizerConfig {
    
    pub fn merge(&mut self, other: TokenizerConfig) -> Result<(), ConfigError> {
        // Merge token categories
        for (category, config) in other.token_categories {
            self.token_categories.entry(category).or_insert(config);
        }

        // Merge patterns
        for (category, patterns) in other.patterns {
            self.patterns.entry(category)
                .or_insert_with(Vec::new)
                .extend(patterns);
        }

        // Merge context rules
        self.context_rules.extend(other.context_rules);

        // Merge custom actions
        self.custom_actions.extend(other.custom_actions);

        Ok(())
    }

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
        })
    }

    pub fn get_pattern(&self, category: &str) -> Option<&Vec<CompiledPatternConfig>> {
        self.patterns.get(category)
    }

    pub fn add_pattern(&mut self, category: String, pattern: CompiledPatternConfig) {
        self.patterns.entry(category).or_insert_with(Vec::new).push(pattern);
    }

    pub fn get_context_rule(&self, name: &str) -> Option<&ContextRule> {
        self.context_rules.get(name)
    }

    pub fn add_context_rule(&mut self, name: String, rule: ContextRule) {
        self.context_rules.insert(name, rule);
    }

    pub fn get_custom_action(&self, name: &str) -> Option<&CustomAction> {
        self.custom_actions.get(name)
    }

    pub fn add_custom_action(&mut self, name: String, action: CustomAction) {
        self.custom_actions.insert(name, action);
    }

}