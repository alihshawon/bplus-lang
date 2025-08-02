// compiler/src/token.rs

use std::collections::HashMap;
use once_cell::sync::Lazy;
use std::fmt;

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
    Hoy,
    Tahole,
    Nahoy,
    Noyto,
    Noile,
    Othoba,
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

/// Implement Display trait for TokenType to enable to_string() calls
impl fmt::Display for TokenType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "EOF",

            TokenType::Ident => "Ident",
            TokenType::Int => "Int",
            TokenType::Float => "Float",
            TokenType::List => "List",
            TokenType::Set => "Set",
            TokenType::String => "String",

            TokenType::Assign => "=",
            TokenType::Plus => "+",
            TokenType::Minus => "-",
            TokenType::Bang => "!",
            TokenType::Asterisk => "*",
            TokenType::Slash => "/",
            TokenType::Lt => "<",
            TokenType::Gt => ">",
            TokenType::Eq => "==",
            TokenType::LtEq => "<=",
            TokenType::GtEq => ">=",
            TokenType::NotEq => "!=",

            TokenType::Comma => ",",
            TokenType::Semicolon => ";",
            TokenType::LParen => "(",
            TokenType::RParen => ")",
            TokenType::LBrace => "{",
            TokenType::RBrace => "}",
            TokenType::LBracket => "[",
            TokenType::RBracket => "]",

            TokenType::Function => "function",
            TokenType::Dhoro => "dhoro",
            TokenType::Ha => "ha",
            TokenType::Na => "na",
            TokenType::Jodi => "jodi",
            TokenType::Hoy => "hoy",
            TokenType::Tahole => "tahole",
            TokenType::Nahoy => "nahoy",
            TokenType::Noyto => "noyto",
            TokenType::Noile => "noile",
            TokenType::Othoba => "othoba",
            TokenType::Ferot => "ferot",
            TokenType::Dekhao => "dekhao",
            TokenType::Input => "input",
            TokenType::Shomoy => "shomoy",
        };
        write!(f, "{}", s)
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
    map.insert("thikache", TokenType::Ha);
    map.insert("sotti", TokenType::Ha);
    map.insert("true", TokenType::Ha);

    // Boolean false variants
    map.insert("na", TokenType::Na);
    map.insert("mitthe", TokenType::Na);
    map.insert("mitha", TokenType::Na);
    map.insert("false", TokenType::Na);

    // Conditional 'if' variants
    map.insert("jodi", TokenType::Jodi);

    // Remaining part of conditional 'if' statements
    map.insert("hoy", TokenType::Hoy);

    // Conditional 'then' variants
    map.insert("tahole", TokenType::Tahole);

    // Conditional 'else' variants
    map.insert("nahoy", TokenType::Nahoy);
    map.insert("nahole", TokenType::Nahoy);
    map.insert("noyto", TokenType::Noyto);
    map.insert("noile", TokenType::Noile);
    map.insert("othoba", TokenType::Othoba);


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
