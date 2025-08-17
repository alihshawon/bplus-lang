// compiler/src/token.rs

use std::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Categories for tokens, useful for classification and parsing logic.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenCategory {
    Illegal,
    Eof,
    Identifier,
    Literal,
    Operator,
    BitwiseOperator,
    Delimiter,
    Keyword,
    Comment,
    Loop,
    Module,
    ExceptionHandling,
    TypeSystem,
    DataStructure,
    Async,
    Reserved, // Reserved or disabled tokens for future use
}

/// Enum representing all possible token types recognized by the B+ compiler.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenType {
    // Special tokens
    Illegal,      // Unknown or invalid token
    Eof,          // End of file/input

    // Identifiers and literals
    Ident,        // Variable/function names
    Int,          // Integer literals
    Float,        // Floating point literals
    Double,       // Double precision float
    Complex,      // Complex number
    Decimal,      // High precision decimal
    Bool,         // Boolean literal (true/false) -- NOTE: Your language uses Ha/Na keywords for bool, consider usage
    Vector,       // Vector type (optional)
    Matrix,       // Matrix type (optional)
    Char,         // Character literals
    List,         // List literals
    Set,          // Set literals
    String,       // String literals
    Object,       // Object literals (added to match KEYWORDS)

    // Operators
    Assign,       // =
    Plus,         // +
    Minus,        // -
    Bang,         // !
    Asterisk,     // *
    Slash,        // /
    Lt,           // <
    Gt,           // >
    Eq,           // ==
    LtEq,         // <=
    GtEq,         // >=
    NotEq,        // !=

    // Bitwise Operators
    Ampersand,    // &
    Pipe,         // |
    Caret,        // ^
    Tilde,        // ~
    ShiftLeft,    // <<
    ShiftRight,   // >>

    // Delimiters and punctuation
    Comma,        // ,
    Semicolon,    // ;
    LParen,       // (
    RParen,       // )
    LBrace,       // {
    RBrace,       // }
    LBracket,     // [
    RBracket,     // ]
    Fullstop,     // .
    Colon,        // :

    // Language keywords (Banglish)
    Function,     // function keyword
    Dhoro,        // let/variable declaration
    Ha,           // true boolean literal keyword
    Na,           // false boolean literal keyword
    Jodi,         // if conditional start
    Hoy,          // condition connector (like "is")
    Tahole,       // then keyword
    Nahoy,        // else keyword
    Othoba,       // or keyword
    Ebong,        // and keyword
    ReturnKoro,   // return keyword
    Dekhao,       // print keyword
    InputNao,     // input function keyword
    Shomoy,       // time keyword

    // Comment tokens for single and multi-line comments
    EkLineMontobbo,
    BohuLineMontobboShuru,
    BohuLineMontobboShesh,

    // Loop-related keywords
    Jotokhon,
    AgeKoro,
    ErJonno,
    ProtitarJonno,
    Choluk,
    Thamo,
    Jekhane,
    Protibar,

    // Module system
    ImportKoro,
    ExportKoro,
    Module,
    EiHisebe,     // as (aliasing)

    // Exception handling
    CheshtaKoro,
    DhoreFelo,
    Oboseshe,
    ThrowKoro,

    // Type system
    TypeBanao,
    Dhoroner,
    Kisuna,       // null/none

    // Data structures
    Talika,
    Arrow,
    DoubleColon,

    // Async
    OpekkhaKoro,
    ShomoyNiropekho,

    // Reserved / Future use tokens
    // AttributeStart,
    // AttributeEnd,
    // Macro,
}

impl TokenType {
    /// Returns the category of the token type.
    pub fn category(&self) -> TokenCategory {
        match self {
            TokenType::Illegal => TokenCategory::Illegal,
            TokenType::Eof => TokenCategory::Eof,

            TokenType::Ident => TokenCategory::Identifier,
            
            TokenType::Int 
            | TokenType::Float 
            | TokenType::Double 
            | TokenType::Complex 
            | TokenType::Decimal 
            | TokenType::Bool 
            | TokenType::Char
            | TokenType::String 
            | TokenType::List 
            | TokenType::Set 
            | TokenType::Object 
            | TokenType::Vector 
            | TokenType::Matrix => TokenCategory::Literal,

            TokenType::Assign 
            | TokenType::Plus 
            | TokenType::Minus 
            | TokenType::Bang 
            | TokenType::Asterisk 
            | TokenType::Slash 
            | TokenType::Lt 
            | TokenType::Gt 
            | TokenType::Eq 
            | TokenType::LtEq 
            | TokenType::GtEq 
            | TokenType::NotEq => TokenCategory::Operator,

            TokenType::Ampersand 
            | TokenType::Pipe 
            | TokenType::Caret 
            | TokenType::Tilde 
            | TokenType::ShiftLeft 
            | TokenType::ShiftRight => TokenCategory::BitwiseOperator,

            TokenType::Comma 
            | TokenType::Semicolon 
            | TokenType::LParen 
            | TokenType::RParen 
            | TokenType::LBrace 
            | TokenType::RBrace 
            | TokenType::LBracket 
            | TokenType::RBracket 
            | TokenType::Fullstop 
            | TokenType::Colon => TokenCategory::Delimiter,

            TokenType::Function 
            | TokenType::Dhoro 
            | TokenType::Ha 
            | TokenType::Na 
            | TokenType::Jodi 
            | TokenType::Hoy 
            | TokenType::Tahole 
            | TokenType::Nahoy 
            | TokenType::Othoba 
            | TokenType::Ebong
            | TokenType::ReturnKoro 
            | TokenType::Dekhao 
            | TokenType::InputNao 
            | TokenType::Shomoy => TokenCategory::Keyword,

            TokenType::EkLineMontobbo 
            | TokenType::BohuLineMontobboShuru 
            | TokenType::BohuLineMontobboShesh => TokenCategory::Comment,

            TokenType::Jotokhon 
            | TokenType::AgeKoro 
            | TokenType::ErJonno 
            | TokenType::ProtitarJonno 
            | TokenType::Choluk 
            | TokenType::Thamo 
            | TokenType::Jekhane 
            | TokenType::Protibar => TokenCategory::Loop,

            TokenType::ImportKoro 
            | TokenType::ExportKoro 
            | TokenType::Module 
            | TokenType::EiHisebe => TokenCategory::Module,

            TokenType::CheshtaKoro 
            | TokenType::DhoreFelo 
            | TokenType::Oboseshe 
            | TokenType::ThrowKoro => TokenCategory::ExceptionHandling,

            TokenType::TypeBanao 
            | TokenType::Dhoroner 
            | TokenType::Kisuna => TokenCategory::TypeSystem,

            TokenType::Talika 
            | TokenType::Arrow 
            | TokenType::DoubleColon => TokenCategory::DataStructure,

            TokenType::OpekkhaKoro 
            | TokenType::ShomoyNiropekho => TokenCategory::Async,

            // Reserved tokens could be added here if enabled later
            // _ => TokenCategory::Reserved,
        }
    }
}

/// Struct representing a token, consisting of type, literal, and position info.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    pub token_type: TokenType,
    pub literal: String,
    pub line: usize,
    pub column: usize,
}

impl Token {
    /// Constructor to create a new token with given type, literal, and position.
    pub fn new(token_type: TokenType, literal: &str, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
            line,
            column,
        }
    }

    /// Creates a string representation of the token, useful for debugging.
    pub fn to_string(&self) -> String {
        format!(
            "{}('{}') at {}:{}",
            self.token_type, self.literal, self.line, self.column
        )
    }
}

impl fmt::Display for TokenType {
    /// Display token type as string representation (mostly for debugging and error messages).
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            TokenType::Illegal => "Illegal",
            TokenType::Eof => "EOF",

            TokenType::Ident => "Ident",
            TokenType::Int => "Int",
            TokenType::Float => "Float",
            TokenType::Double => "Double",
            TokenType::Complex => "Complex",
            TokenType::Decimal => "Decimal",
            TokenType::Bool => "Bool",
            TokenType::Vector => "Vector",
            TokenType::Matrix => "Matrix",
            TokenType::Char => "Char",
            TokenType::List => "List",
            TokenType::Set => "Set",
            TokenType::String => "String",
            TokenType::Object => "Object",

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

            TokenType::Ampersand => "&",
            TokenType::Pipe => "|",
            TokenType::Caret => "^",
            TokenType::Tilde => "~",
            TokenType::ShiftLeft => "<<",
            TokenType::ShiftRight => ">>",

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
            TokenType::Othoba => "othoba",
            TokenType::Ebong => "ebong",
            TokenType::ReturnKoro => "return koro",
            TokenType::Dekhao => "dekhao",
            TokenType::InputNao => "input nao",
            TokenType::Shomoy => "shomoy",

            TokenType::EkLineMontobbo => "EkLineMontobbo",
            TokenType::BohuLineMontobboShuru => "BohuLineMontobboShuru",
            TokenType::BohuLineMontobboShesh => "BohuLineMontobboShesh",

            TokenType::Jotokhon => "jotokhon",
            TokenType::AgeKoro => "age koro",
            TokenType::ErJonno => "er jonno",
            TokenType::ProtitarJonno => "protitar jonno",
            TokenType::Choluk => "choluk",
            TokenType::Thamo => "thamo",
            TokenType::Jekhane => "jekhane",
            TokenType::Protibar => "protibar",

            TokenType::ImportKoro => "import koro",
            TokenType::ExportKoro => "export koro",
            TokenType::Module => "module",
            TokenType::EiHisebe => "ei hisebe",

            TokenType::CheshtaKoro => "cheshta koro",
            TokenType::DhoreFelo => "dhore felo",
            TokenType::Oboseshe => "oboseshe",
            TokenType::ThrowKoro => "throw koro",

            TokenType::TypeBanao => "type banao",
            TokenType::Dhoroner => "dhoroner",
            TokenType::Kisuna => "kisuna",

            TokenType::Talika => "talika",
            TokenType::Arrow => "->",
            TokenType::DoubleColon => "::",

            TokenType::OpekkhaKoro => "opekkha koro",
            TokenType::ShomoyNiropekho => "shomoy niropekkho",
        };
        write!(f, "{}", s)
    }
}

/// Normalize an input identifier or keyword by converting to lowercase and collapsing whitespace.
fn normalize_keyword(ident: &str) -> String {
    ident
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// A map of keyword strings and their synonyms to the canonical TokenType.
/// This allows multiple natural language variants to map to the same token.
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Variable declaration keywords
    map.insert("dhoro", TokenType::Dhoro);
    map.insert("dhori", TokenType::Dhoro);
    map.insert("monekori", TokenType::Dhoro);
    map.insert("monekoro", TokenType::Dhoro);

    // Function keywords
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

    // Condition keywords
    map.insert("jodi", TokenType::Jodi);
    map.insert("hoy", TokenType::Hoy);
    map.insert("hoye", TokenType::Hoy);
    map.insert("hoyethake", TokenType::Hoy);
    map.insert("hoye thake", TokenType::Hoy);
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
    map.insert("othoba", TokenType::Othoba);
    map.insert("ba", TokenType::Othoba);
    map.insert("or", TokenType::Othoba);

    // Return statement variants
    map.insert("ferot", TokenType::ReturnKoro);
    map.insert("ferot koro", TokenType::ReturnKoro);
    map.insert("return koro", TokenType::ReturnKoro);
    map.insert("return", TokenType::ReturnKoro);

    // Print statement variants
    map.insert("dekhao", TokenType::Dekhao);
    map.insert("print", TokenType::Dekhao);
    map.insert("show koro", TokenType::Dekhao);

    // Input variants all mapping to InputNao token
    map.insert("input nao", TokenType::InputNao);
    map.insert("input", TokenType::InputNao);
    map.insert("input nao koro", TokenType::InputNao);
    map.insert("nibho", TokenType::InputNao);

    // Time keywords
    map.insert("shomoy", TokenType::Shomoy);
    map.insert("time", TokenType::Shomoy);
    map.insert("somoy", TokenType::Shomoy);

    // Comment tokens with variants
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
    map.insert("jotokhon porjonto", TokenType::Jotokhon);
    map.insert("age koro", TokenType::AgeKoro);
    map.insert("agekoro", TokenType::AgeKoro);
    map.insert("er jonno", TokenType::ErJonno);
    map.insert("erjonno", TokenType::ErJonno);
    map.insert("protitar jonno", TokenType::ProtitarJonno);
    map.insert("protitarjonno", TokenType::ProtitarJonno);
    map.insert("choluk", TokenType::Choluk);
    map.insert("thamo", TokenType::Thamo);
    map.insert("bhango", TokenType::Thamo);
    map.insert("jekhane", TokenType::Jekhane);
    map.insert("protibar", TokenType::Protibar);

    // Module system
    map.insert("amdani koro", TokenType::ImportKoro);
    map.insert("import", TokenType::ImportKoro);
    map.insert("import koro", TokenType::ImportKoro);

    map.insert("roptani koro", TokenType::ExportKoro);
    map.insert("export", TokenType::ExportKoro);
    map.insert("export koro", TokenType::ExportKoro);

    map.insert("module", TokenType::Module);

    map.insert("ei hisebe", TokenType::EiHisebe);
    map.insert("as", TokenType::EiHisebe);

    // Exception handling
    map.insert("try koro", TokenType::CheshtaKoro);
    map.insert("try", TokenType::CheshtaKoro);
    map.insert("cheshta koro", TokenType::CheshtaKoro);

    map.insert("dhore felo", TokenType::DhoreFelo);
    map.insert("catch", TokenType::DhoreFelo);

    map.insert("oboseshe", TokenType::Oboseshe);
    map.insert("finally", TokenType::Oboseshe);

    map.insert("felo", TokenType::ThrowKoro);
    map.insert("throw", TokenType::ThrowKoro);
    map.insert("throw koro", TokenType::ThrowKoro);

    // Type system
    map.insert("type banao", TokenType::TypeBanao);
    map.insert("type gothon koro", TokenType::TypeBanao);
    map.insert("nirdharon koro", TokenType::TypeBanao);
    map.insert("type dao", TokenType::TypeBanao);

    map.insert("dhoron ber koro", TokenType::Dhoroner);
    map.insert("type nirnoy koro", TokenType::Dhoroner);
    map.insert("typeof", TokenType::Dhoroner);

    map.insert("kisuna", TokenType::Kisuna);
    map.insert("nil", TokenType::Kisuna);
    map.insert("null", TokenType::Kisuna);
    map.insert("none", TokenType::Kisuna);

    // Async
    map.insert("opekkha", TokenType::OpekkhaKoro);
    map.insert("opekkha koro", TokenType::OpekkhaKoro);
    map.insert("await", TokenType::OpekkhaKoro);

    map.insert("shomoy niropekkho", TokenType::ShomoyNiropekho);
    map.insert("asynchronous", TokenType::ShomoyNiropekho);
    map.insert("async", TokenType::ShomoyNiropekho);

    map
});

/// Lookup a keyword or identifier and return its token type.
/// If no keyword match, returns TokenType::Ident.
pub fn lookup_ident(ident: &str) -> TokenType {
    if let Some(&tok_type) = KEYWORDS.get(ident) {
        return tok_type;
    }

    // Try normalized variant with lowercase and collapsed whitespace
    let normalized = normalize_keyword(ident);
    if let Some(&tok_type) = KEYWORDS.get(normalized.as_str()) {
        return tok_type;
    }

    TokenType::Ident
}

/// Helper: checks if token is a literal type
pub fn is_literal(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Int
            | TokenType::Float
            | TokenType::Double
            | TokenType::Complex
            | TokenType::Decimal
            | TokenType::Bool
            | TokenType::Char
            | TokenType::String
            | TokenType::List
            | TokenType::Set
            | TokenType::Object
            | TokenType::Vector
            | TokenType::Matrix
    )
}

/// Helper: checks if token is an operator
pub fn is_operator(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Assign
            | TokenType::Plus
            | TokenType::Minus
            | TokenType::Bang
            | TokenType::Asterisk
            | TokenType::Slash
            | TokenType::Lt
            | TokenType::Gt
            | TokenType::Eq
            | TokenType::LtEq
            | TokenType::GtEq
            | TokenType::NotEq
            | TokenType::Ampersand
            | TokenType::Pipe
            | TokenType::Caret
            | TokenType::Tilde
            | TokenType::ShiftLeft
            | TokenType::ShiftRight
    )
}

/// Helper: checks if token is a keyword
pub fn is_keyword(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Function
            | TokenType::Dhoro
            | TokenType::Ha
            | TokenType::Na
            | TokenType::Jodi
            | TokenType::Hoy
            | TokenType::Tahole
            | TokenType::Nahoy
            | TokenType::Othoba
            | TokenType::Ebong
            | TokenType::ReturnKoro
            | TokenType::Dekhao
            | TokenType::InputNao
            | TokenType::Shomoy
    )
}

/// Helper: checks if token is a loop keyword
pub fn is_loop(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Jotokhon
            | TokenType::AgeKoro
            | TokenType::ErJonno
            | TokenType::ProtitarJonno
            | TokenType::Choluk
            | TokenType::Thamo
            | TokenType::Jekhane
            | TokenType::Protibar
    )
}

/// Helper: checks if token is a comment token
pub fn is_comment(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::EkLineMontobbo
            | TokenType::BohuLineMontobboShuru
            | TokenType::BohuLineMontobboShesh
    )
}

/// Helper: checks if token is reserved (currently no reserved tokens enabled)
pub fn is_reserved(_token_type: TokenType) -> bool {
    // Uncomment when Reserved tokens are added
    // matches!(
    //     token_type,
    //     TokenType::AttributeStart
    //         | TokenType::AttributeEnd
    //         | TokenType::Macro
    // )
    false
}

pub static RESERVED_KEYWORDS: &[&str] = &[
    "jodi",
    "nahoy",
    "ha",
    "na",
    "dhoro",
    "returnkoro",
    "dekhao",
    "inputnao",
    "function",
    // ... rest reserved keywords that can't be used as variable names.
];
