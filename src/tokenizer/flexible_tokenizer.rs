// abap-tokenizer/src/tokenizer/flexible_tokenizer.rs
//! Flexible tokenizer implementation for ABAP code.
//!
//! This module contains the main tokenizer logic, capable of processing ABAP code
//! based on a provided configuration. It handles various token types, special rules,
//! and provides detailed error reporting and debugging information.

use std::collections::HashMap;
use super::token::Token;
use super::token_type::TokenType;
use crate::config::tokenizer_config::SpecialRule;
use crate::config::TokenizerConfig;
use crate::error::TokenizerError;
use log::debug;
use regex::Regex;



/// Type alias for a function that validates special rules.
type RuleValidator = Box<dyn Fn(&str, &SpecialRule, usize) -> Option<usize>>;
pub struct FlexibleTokenizer<'a> {
    /// The input ABAP code to be tokenized.
    input: &'a str,
    /// The configuration for the tokenizer.
    config: TokenizerConfig,
    /// The current position in the input string.
    position: usize,
    /// The current line number being processed.
    line: usize,
    /// The current column number being processed.
    column: usize,
}

/// The main tokenizer struct for processing ABAP code.
impl<'a> FlexibleTokenizer<'a> {
    /// Creates a new FlexibleTokenizer instance.
    ///
    /// # Arguments
    ///
    /// * `input` - A string slice containing the ABAP code to be tokenized
    /// * `config` - The TokenizerConfig to use for tokenization
    ///
    /// # Returns
    ///
    /// A new FlexibleTokenizer instance
    pub fn new(input: &'a str, config: TokenizerConfig) -> Self {
        FlexibleTokenizer {
            input,
            config,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    /// Processes the next token in the input.
    ///
    /// This method is the core of the tokenization process. It performs the following steps:
    /// 1. Skips any whitespace.
    /// 2. Checks for the end of input.
    /// 3. Attempts to match special rules.
    /// 4. Attempts to match regular token patterns.
    /// 5. Handles unknown tokens if no match is found.
    ///
    /// # Returns
    ///
    /// * `Result<Option<Token>, TokenizerError>` - The next token if available, None if end of input,
    ///   or an error if tokenization fails.
    pub fn next_token(&mut self) -> Result<Option<Token>, TokenizerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let remaining_input = &self.input[self.position..];

        // Check special rules first
        if let Some((token, length)) = self.check_special_rules(remaining_input)? {
            for _ in 0..length {
                self.advance();
            }
            return Ok(Some(token));
        }

        // Then check regular patterns
        if let Some((token, length)) = self.find_next_token(remaining_input)? {
            for _ in 0..length {
                self.advance();
            }
            return Ok(Some(token));
        }

        // If no pattern matches, advance one character and return an unknown token
        let ch = self.advance();
        debug!("Unknown token encountered: {}", ch);
        Ok(Some(Token::new(
            TokenType::new("Unknown".to_string(), None),
            ch.to_string(),
            self.line,
            self.column,
        )))
    }
    
    /// Attempts to find the next token based on the configured patterns.
    ///
    /// This method iterates through all token categories in order of priority,
    /// attempting to match the input against each pattern in the category.
    ///
    /// # Arguments
    ///
    /// * `input` - The remaining input string to be tokenized
    ///
    /// # Returns
    ///
    /// * `Result<Option<(Token, usize)>, TokenizerError>` - A tuple containing the matched token
    ///   and its length if found, or None if no match is found.
    fn find_next_token(&self, input: &str) -> Result<Option<(Token, usize)>, TokenizerError> {
        let mut categories: Vec<_> = self.config.token_categories.iter().collect();
        categories.sort_by_key(|&(_, config)| config.priority);

        for (category, _) in categories {
            if let Some(patterns) = self.config.get_pattern(category) {
                for pattern in patterns {
                    if let Some(mat) = pattern.regex.find(input) {
                        if mat.start() == 0 {
                            let value = mat.as_str().to_string();
                            let token_type = TokenType::new(
                                category.to_string(),
                                pattern.subcategory.clone(),
                            );
                            let token = Token::new(token_type, value.clone(), self.line, self.column);
                            debug!("Matched token: {:?}", token);
                            return Ok(Some((token, mat.end())));
                        }
                    }
                }
            }
        }

        Ok(None)
    }

    /// Checks if any special rules apply to the current input.
    ///
    /// This method iterates through all special rules defined in the configuration,
    /// applying each rule's validators to the input.
    ///
    /// # Arguments
    ///
    /// * `input` - The remaining input string to be tokenized
    ///
    /// # Returns
    ///
    /// * `Result<Option<(Token, usize)>, TokenizerError>` - A tuple containing the matched token
    ///   and its length if a special rule applies, or None if no special rule matches.
    fn check_special_rules(&self, input: &str) -> Result<Option<(Token, usize)>, TokenizerError> {
        let validators: HashMap<&str, RuleValidator> = vec![
            ("start", Box::new(Self::validate_start) as RuleValidator),
            ("end", Box::new(Self::validate_end) as RuleValidator),
            ("regex", Box::new(Self::validate_regex) as RuleValidator),
            (
                "min_length",
                Box::new(Self::validate_min_length) as RuleValidator,
            ),
            (
                "start_column",
                Box::new(Self::validate_start_column) as RuleValidator,
            ),
        ]
        .into_iter()
        .collect();
        for rule in &self.config.special_rules {
            let mut end_pos = input.len();
            let mut is_valid = true;
            for (attr, validator) in &validators {
                if let Some(_) = rule.get_attribute(attr) {
                    match validator(input, rule, self.column) {
                        Some(pos) => end_pos = end_pos.min(pos),
                        None => {
                            is_valid = false;
                            break;
                        }
                    }
                }
            }
            if is_valid {
                let token_value = &input[..end_pos];
                return Ok(Some((
                    Token::new(
                        TokenType::new(rule.token_type.clone(), None),
                        token_value.to_string(),
                        self.line,
                        self.column,
                    ),
                    end_pos,
                )));
            }
        }
        Ok(None)
    }

    /// Validates the start condition of a special rule.
    fn validate_start(input: &str, rule: &SpecialRule, _: usize) -> Option<usize> {
        if input.starts_with(&rule.start) {
            Some(input.len())
        } else {
            None
        }
    }

    /// Validates the end condition of a special rule.
    fn validate_end(input: &str, rule: &SpecialRule, _: usize) -> Option<usize> {
        rule.end.as_ref().and_then(|end| {
            input[rule.start.len()..]
                .find(end)
                .map(|pos| rule.start.len() + pos + end.len())
        })
    }

    /// Validates the regex condition of a special rule.
    fn validate_regex(input: &str, rule: &SpecialRule, _: usize) -> Option<usize> {
        rule.regex.as_ref().and_then(|regex| {
            Regex::new(regex).ok().and_then(|re| {
                re.find(input).and_then(|mat| {
                    if mat.start() == 0 {
                        Some(mat.end())
                    } else {
                        None
                    }
                })
            })
        })
    }

    /// Validates the minimum length condition of a special rule.
    fn validate_min_length(input: &str, rule: &SpecialRule, _: usize) -> Option<usize> {
        rule.min_length.map_or(Some(input.len()), |min_len| {
            if input.len() >= min_len {
                Some(input.len())
            } else {
                None
            }
        })
    }

    /// Validates the start column condition of a special rule.
    fn validate_start_column(
        input: &str,
        rule: &SpecialRule,
        current_column: usize,
    ) -> Option<usize> {
        rule.start_column.map_or(Some(input.len()), |start_col| {
            if current_column == start_col {
                Some(input.len())
            } else {
                None
            }
        })
    }

    /// Skips whitespace characters in the input.
    fn skip_whitespace(&mut self) {
        while self.position < self.input.len()
            && self.input[self.position..]
                .chars()
                .next()
                .unwrap()
                .is_whitespace()
        {
            self.advance();
        }
    }

    /// Advances the current position in the input by one character.
    ///
    /// This method also updates the line and column counters.
    ///
    /// # Returns
    ///
    /// The character at the current position before advancing.
    fn advance(&mut self) -> char {
        if let Some(ch) = self.input[self.position..].chars().next() {
            self.position += ch.len_utf8();
            self.column += 1;
            if ch == '\n' {
                self.line += 1;
                self.column = 1;
            }
            ch
        } else {
            '\0' // Return a null character if there are no more characters
        }
    }
}