// abap-tokenizer/src/tokenizer/mod.rs
//! Core tokenization module for the ABAP Tokenizer.
//!
//! This module contains the main components responsible for the tokenization process
//! of ABAP code. It includes the flexible tokenizer implementation, token structures,
//! and token type definitions.

/// Contains the implementation of the flexible tokenizer for ABAP code.
pub mod flexible_tokenizer;

/// Defines the Token structure representing individual tokens in ABAP code.
pub mod token;

/// Defines the TokenType structure representing the types of tokens in ABAP code.
pub mod token_type;