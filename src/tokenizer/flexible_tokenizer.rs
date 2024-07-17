// abap-tokenizer/src/tokenizer/flexible_tokenizer.rs
use crate::error::TokenizerError;
use crate::config::TokenizerConfig;
use super::token::Token;
use super::token_type::TokenType;
use log::debug;

pub struct FlexibleTokenizer<'a> {
    input: &'a str,
    config: TokenizerConfig,
    position: usize,
    line: usize,
    column: usize,
}

impl<'a> FlexibleTokenizer<'a> {
    pub fn new(input: &'a str, config: TokenizerConfig) -> Self {
        FlexibleTokenizer {
            input,
            config,
            position: 0,
            line: 1,
            column: 1,
        }
    }

    pub fn next_token(&mut self) -> Result<Option<Token>, TokenizerError> {
        self.skip_whitespace();

        if self.position >= self.input.len() {
            return Ok(None);
        }

        let remaining_input = &self.input[self.position..];

        // Try to match patterns
        if let Some((token, length)) = self.find_next_token(remaining_input)? {
            // Advance the position
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

    fn find_next_token(&self, input: &str) -> Result<Option<(Token, usize)>, TokenizerError> {
        for (category, patterns) in &self.config.patterns {
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
        Ok(None)
    }

    fn skip_whitespace(&mut self) {
        while self.position < self.input.len() && self.input[self.position..].chars().next().unwrap().is_whitespace() {
            self.advance();
        }
    }

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
            '\0'  // Return a null character if there are no more characters
        }
    }
}