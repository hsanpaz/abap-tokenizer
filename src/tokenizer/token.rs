// abap-tokenizer/src/tokenizer/token.rs
//! Token representation for the ABAP Tokenizer.
//!
//! This module defines the `Token` struct, which represents individual tokens
//! identified during the tokenization process of ABAP code.
use super::token_type::TokenType;

/// Represents a single token in the ABAP code.
///
/// A token is a meaningful unit of code, such as a keyword, identifier, 
/// literal, or punctuation mark. It includes information about its type,
/// value, and position in the source code.
#[derive(Debug, Clone)]
pub struct Token {
    /// The type of the token, including its category and subcategory.
    pub token_type: TokenType,
    /// The actual text value of the token as it appears in the source code.
    pub value: String,
    /// The line number where the token appears in the source code.
    pub line: usize,
    /// The column number where the token starts in its line.
    pub column: usize,
}

impl Token {
    /// Creates a new Token instance.
    ///
    /// # Arguments
    ///
    /// * `token_type` - The type of the token
    /// * `value` - The string value of the token
    /// * `line` - The line number where the token appears
    /// * `column` - The column number where the token starts
    ///
    /// # Returns
    ///
    /// A new Token instance with the provided attributes.
    pub fn new(token_type: TokenType, value: String, line: usize, column: usize) -> Self {
        Token {
            token_type,
            value,
            line,
            column,
        }
    }
}