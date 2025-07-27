// compiler/src/token.rs

use std::collections::HashMap;

// Added Eq and Hash to derive macro.
// Changed all variants to UpperCamelCase to follow Rust conventions and fix warnings.
#[derive(Debug, PartialEq, Clone, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,
    String,

    // Operators
    Assign,
    Plus,
    Minus,
    Bang,
    Asterisk,
    Slash,
    Lt,
    Gt,
    Eq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,

    // Keywords
    Function,
    Let,
    Ha,
    Na,
    Jodi,
    Nahoy,
    Ferot,
    Dekhao,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
}

impl Token {
    pub fn new(token_type: TokenType, literal: &str) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
        }
    }
}

/// Looks up an identifier string to see if it's a keyword.
pub fn lookup_ident(ident: &str) -> TokenType {
    let mut keywords: HashMap<String, TokenType> = HashMap::new();
    // Using UpperCamelCase variants and removed old English keywords.
    keywords.insert("fn".to_string(), TokenType::Function);
    keywords.insert("let".to_string(), TokenType::Let);
    keywords.insert("ha".to_string(), TokenType::Ha);
    keywords.insert("thik".to_string(), TokenType::Ha);
    keywords.insert("na".to_string(), TokenType::Na);
    keywords.insert("jodi".to_string(), TokenType::Jodi);
    keywords.insert("nahoy".to_string(), TokenType::Nahoy);
    keywords.insert("ferot".to_string(), TokenType::Ferot);
    keywords.insert("dekhao".to_string(), TokenType::Dekhao);

    if let Some(tok_type) = keywords.get(ident) {
        tok_type.clone()
    } else {
        TokenType::Ident
    }
}
