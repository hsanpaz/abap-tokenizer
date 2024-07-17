// abap-tokenizer\src\tokenizer\token_type.rs
use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct TokenType {
    pub category: String,
    pub subcategory: Option<String>,
}

impl TokenType {
    pub fn new(category: String, subcategory: Option<String>) -> Self {
        TokenType {
            category,
            subcategory,
        }
    }

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
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.subcategory {
            Some(sub) => write!(f, "{}:{}", self.category, sub),
            None => write!(f, "{}", self.category),
        }
    }
}