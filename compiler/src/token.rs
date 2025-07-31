// compiler/src/token.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;

#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenType {
    Illegal,
    Eof,

    // Identifiers + literals
    Ident,
    Int,
    Float,
    List,
    Set,
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
    LtEq,
    GtEq,
    NotEq,

    // Delimiters
    Comma,
    Semicolon,
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Keywords
    Function,
    Dhoro,
    Ha,
    Na,
    Jodi,
    Nahoy,
    Ferot,
    Dekhao,
    Input,
    Shomoy,
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

/// Normalize input by converting to lowercase and removing spaces.
fn normalize_keyword(ident: &str) -> String {
    ident.to_lowercase().replace(' ', "")
}

/// Static keyword map for fast lookup.
static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Let variants
    map.insert("dhoro", TokenType::Dhoro);
    map.insert("dhori", TokenType::Dhoro);
    map.insert("monekori", TokenType::Dhoro);
    map.insert("monekoro", TokenType::Dhoro);
    map.insert("monekore", TokenType::Dhoro);

    // Function variants
    map.insert("kaj", TokenType::Function);
    map.insert("fn", TokenType::Function);
    map.insert("function", TokenType::Function);

    // Boolean true variants
    map.insert("ha", TokenType::Ha);
    map.insert("thik", TokenType::Ha);
    map.insert("sotti", TokenType::Ha);
    map.insert("true", TokenType::Ha);

    // Boolean false variants
    map.insert("na", TokenType::Na);
    map.insert("mitthe", TokenType::Na);
    map.insert("false", TokenType::Na);

    // Conditional 'if' variants
    map.insert("jodi", TokenType::Jodi);
    map.insert("yodi", TokenType::Jodi);
    map.insert("if", TokenType::Jodi);

    // Conditional 'else' variants
    map.insert("nahoy", TokenType::Nahoy);
    map.insert("nahole", TokenType::Nahoy);
    map.insert("else", TokenType::Nahoy);

    // Return statement variants
    map.insert("ferot", TokenType::Ferot);
    map.insert("return", TokenType::Ferot);

    map
});

/// Lookup if an identifier matches a keyword or return Ident otherwise.
pub fn lookup_ident(ident: &str) -> TokenType {
    let normalized = normalize_keyword(ident);

    if let Some(&tok_type) = KEYWORDS.get(normalized.as_str()) {
        tok_type
    } else {
        TokenType::Ident
    }
}
