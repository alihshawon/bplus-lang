use std::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// The core token types used by the B+ compiler.
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
    Fullstop, // .
    Colon,    // :

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
    Ebong,
    Ba,
    Ferot,
    Dekhao,
    InputNao,
    Shomoy,

    // Comment tokens
    EkLineMontobbo,
    BohuLineMontobboShuru,
    BohuLineMontobboShesh,

    // Loop tokens
    Jotokhon,
    AgeKoro,
    Jonno,
    ProtitarJonno,
    Choluk,
    Thamo,
    Jekhane,
    Protibar,
}

/// Token struct with type and literal value
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

            TokenType::Fullstop => ".",
            TokenType::Colon => ":",

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
            TokenType::Ebong => "ebong",
            TokenType::Ba => "ba",
            TokenType::Ferot => "ferot",
            TokenType::Dekhao => "dekhao",
            TokenType::InputNao => "input nao",
            TokenType::Shomoy => "shomoy",

            TokenType::EkLineMontobbo => "EkLineMontobbo",
            TokenType::BohuLineMontobboShuru => "BohuLineMontobboShuru",
            TokenType::BohuLineMontobboShesh => "BohuLineMontobboShesh",

            TokenType::Jotokhon => "jotokhon",
            TokenType::AgeKoro => "age koro",
            TokenType::Jonno => "jonno",
            TokenType::ProtitarJonno => "protitar jonno",
            TokenType::Choluk => "choluk",
            TokenType::Thamo => "thamo",
            TokenType::Jekhane => "jekhane",
            TokenType::Protibar => "protibar",
        };
        write!(f, "{}", s)
    }
}

/// Normalize input by lowercasing and collapsing whitespace to single space
fn normalize_keyword(ident: &str) -> String {
    ident
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// Centralized keyword map with synonyms mapped to their base TokenType
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Let/Variable declaration variants
    map.insert("dhoro", TokenType::Dhoro);
    map.insert("dhori", TokenType::Dhoro);
    map.insert("monekori", TokenType::Dhoro);
    map.insert("monekoro", TokenType::Dhoro);

    // Function variants
    map.insert("kaj", TokenType::Function);
    map.insert("fn", TokenType::Function);
    map.insert("function", TokenType::Function);

    // Boolean true variants
    map.insert("ha", TokenType::Ha);
    map.insert("thik", TokenType::Ha);
    map.insert("sotti", TokenType::Ha);
    map.insert("shotti", TokenType::Ha);
    map.insert("true", TokenType::Ha);
    map.insert("shotto", TokenType::Ha);
    map.insert("sotto", TokenType::Ha);

    // Boolean false variants
    map.insert("na", TokenType::Na);
    map.insert("mitthe", TokenType::Na);
    map.insert("mittha", TokenType::Na);
    map.insert("false", TokenType::Na);
    map.insert("thik noy", TokenType::Na);
    map.insert("thiknoy", TokenType::Na);
    map.insert("vul", TokenType::Na);
    map.insert("bhul", TokenType::Na);

    // If/condition start
    map.insert("jodi", TokenType::Jodi);

    // Condition mid
    map.insert("hoy", TokenType::Hoy);
    map.insert("hoye", TokenType::Hoy);
    map.insert("hoyethake", TokenType::Hoy);
    map.insert("hoye thake", TokenType::Hoy);

    // Then
    map.insert("tahole", TokenType::Tahole);
    map.insert("tobe", TokenType::Tahole);

    // Else variants
    map.insert("nahoy", TokenType::Nahoy);
    map.insert("nahole", TokenType::Nahoy);
    map.insert("noyto", TokenType::Nahoy);
    map.insert("noile", TokenType::Nahoy);

    // Logical operators
    map.insert("ebong", TokenType::Ebong);
    map.insert("and", TokenType::Ebong);
    map.insert("ba", TokenType::Ba);
    map.insert("or", TokenType::Ba);

    // Return statement
    map.insert("ferot", TokenType::Ferot);
    map.insert("ferot koro", TokenType::Ferot);
    map.insert("return koro", TokenType::Ferot);
    map.insert("return", TokenType::Ferot);

    // Print variants
    map.insert("dekhao", TokenType::Dekhao);
    map.insert("print", TokenType::Dekhao);
    map.insert("show koro", TokenType::Dekhao);

    // Input variants (all map to InputNao base token)
    map.insert("input nao", TokenType::InputNao);
    map.insert("input", TokenType::InputNao);
    map.insert("input nao koro", TokenType::InputNao);
    map.insert("nibho", TokenType::InputNao);

    // Time variants
    map.insert("shomoy", TokenType::Shomoy);
    map.insert("time", TokenType::Shomoy);
    map.insert("somoy", TokenType::Shomoy);

    // Comment base keywords and their variants
    map.insert("//", TokenType::EkLineMontobbo);
    map.insert("#", TokenType::EkLineMontobbo);

    map.insert("/*", TokenType::BohuLineMontobboShuru);
    map.insert("//--", TokenType::BohuLineMontobboShuru);
    map.insert("<!--", TokenType::BohuLineMontobboShuru);
    map.insert("<comment>", TokenType::BohuLineMontobboShuru);
    map.insert("<cmnt>", TokenType::BohuLineMontobboShuru);
    map.insert("<montobbo>", TokenType::BohuLineMontobboShuru);

    map.insert("*/", TokenType::BohuLineMontobboShesh);
    map.insert("--//", TokenType::BohuLineMontobboShesh);
    map.insert("-->", TokenType::BohuLineMontobboShesh);
    map.insert("--!>", TokenType::BohuLineMontobboShesh);
    map.insert("</comment>", TokenType::BohuLineMontobboShesh);
    map.insert("</cmnt>", TokenType::BohuLineMontobboShesh);
    map.insert("</montobbo>", TokenType::BohuLineMontobboShesh);

    // Loop keywords
    map.insert("jotokhon", TokenType::Jotokhon);
    map.insert("age koro", TokenType::AgeKoro);
    map.insert("agekoro", TokenType::AgeKoro);
    map.insert("jonno", TokenType::Jonno);
    map.insert("protitar jonno", TokenType::ProtitarJonno);
    map.insert("protitarjonno", TokenType::ProtitarJonno);
    map.insert("choluk", TokenType::Choluk);
    map.insert("thamo", TokenType::Thamo);
    map.insert("bhango", TokenType::Thamo);
    map.insert("jekhane", TokenType::Jekhane);
    map.insert("protibar", TokenType::Protibar);

    map
});

/// Lookup an identifier or keyword string, normalize it and get corresponding TokenType.
/// Returns TokenType::Ident if not a keyword.
pub fn lookup_ident(ident: &str) -> TokenType {
    let normalized = normalize_keyword(ident);
    if let Some(&tok_type) = KEYWORDS.get(normalized.as_str()) {
        tok_type
    } else {
        TokenType::Ident
    }
}
