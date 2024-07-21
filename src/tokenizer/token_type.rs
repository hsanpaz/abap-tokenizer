// abap-tokenizer/src/tokenizer/token_type.rs
//! Token type representation for the ABAP Tokenizer.
//!
//! This module defines the `TokenType` struct, which represents the type
//! and category of tokens identified during the tokenization process of ABAP code.
use std::fmt;

/// Represents the type of a token in ABAP code.
///
/// Each token is classified into a category, with an optional subcategory
/// for more fine-grained classification. This structure is crucial for
/// understanding the role and meaning of each token in the ABAP code.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenType {
    /// The main category of the token (e.g., "Keyword", "Identifier", "Literal").
    pub category: String,
    /// An optional subcategory for more specific classification 
    /// (e.g., "ControlFlow" for keywords like IF, ELSE, etc.).
    pub subcategory: Option<String>,
}

impl TokenType {
    /// Creates a new TokenType instance.
    ///
    /// # Arguments
    ///
    /// * `category` - The main category of the token
    /// * `subcategory` - An optional subcategory for the token
    ///
    /// # Returns
    ///
    /// A new TokenType instance with the provided category and subcategory.
    pub fn new(category: String, subcategory: Option<String>) -> Self {
        TokenType {
            category,
            subcategory,
        }
    }

    /// Creates a TokenType from a string representation.
    ///
    /// The string should be in the format "category:subcategory" or just "category".
    ///
    /// # Arguments
    ///
    /// * `s` - A string slice representing the token type
    ///
    /// # Returns
    ///
    /// A new TokenType instance parsed from the string.
    ///
    /// # Panics
    ///
    /// Panics if the input string is not in the correct format.
    pub fn from_str(s: &str) -> Self {
        let parts: Vec<&str> = s.split(':').collect();
        match parts.len() {
            1 => TokenType::new(parts[0].to_string(), None),
            2 => TokenType::new(parts[0].to_string(), Some(parts[1].to_string())),
            _ => panic!("Invalid token type string: {}", s),
        }
    }
}

impl fmt::Display for TokenType {
    /// Formats the TokenType for display.
    ///
    /// If a subcategory is present, it will be displayed as "category:subcategory".
    /// Otherwise, only the category will be displayed.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subcategory {
            Some(sub) => write!(f, "{}:{}", self.category, sub),
            None => write!(f, "{}", self.category),
        }
    }
}