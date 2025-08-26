// compiler/src/token.rs

use std::fmt;
use once_cell::sync::Lazy;
use std::collections::HashMap;

/// Categories for tokens, useful for classification and parsing logic.
/// Each token type belongs to exactly one category for consistent classification.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum TokenCategory {
    /// Invalid or unrecognized tokens
    Illegal,
    /// End of file marker
    Eof,
    /// Variable and function names
    Identifier,
    /// Numeric and string literals
    Literal,
    /// Arithmetic and comparison operators
    Operator,
    /// Bitwise manipulation operators
    BitwiseOperator,
    /// Punctuation and structural delimiters
    Delimiter,
    /// Language keywords and built-in functions
    Keyword,
    /// Comment markers and content
    Comment,
    /// Loop control keywords
    Loop,
    /// Module system keywords
    Module,
    /// Exception handling keywords
    ExceptionHandling,
    /// Type system keywords
    TypeSystem,
    /// Data structure keywords
    DataStructure,
    /// Asynchronous programming keywords
    Async,
    /// Reserved tokens for future language features
    Reserved,
}

/// Enum representing all possible token types recognized by the B+ compiler.
/// Each variant corresponds to a specific lexical element in the B+ language.
#[derive(Debug, PartialEq, Clone, Copy, Eq, Hash)]
pub enum TokenType {
    // Special tokens
    /// Unknown or invalid token
    Illegal,
    /// End of file/input
    Eof,

    // Identifiers and literals
    /// Variable/function names
    Ident,
    /// Integer literals
    Int,
    /// Floating point literals
    Float,
    /// Double precision float
    Double,
    /// Complex number
    Complex,
    /// High precision decimal
    Decimal,
    /// Boolean literal (true/false) - NOTE: B+ uses Ha/Na keywords for bool
    Bool,
    /// Vector type (optional)
    Vector,
    /// Matrix type (optional)
    Matrix,
    /// Character literals
    Char,
    /// List literals
    List,
    /// Set literals
    Set,
    /// String literals
    String,
    /// Object literals
    Object,

    // Operators
    /// Assignment operator =
    Assign,
    /// Addition operator +
    Plus,
    /// Subtraction operator -
    Minus,
    /// Logical not operator !
    Bang,
    /// Multiplication operator *
    Asterisk,
    /// Division operator /
    Slash,
    /// Less than operator <
    Lt,
    /// Greater than operator >
    Gt,
    /// Equality operator ==
    Eq,
    /// Less than or equal operator <=
    LtEq,
    /// Greater than or equal operator >=
    GtEq,
    /// Not equal operator !=
    NotEq,

    // Bitwise Operators
    /// Bitwise AND &
    Ampersand,
    /// Bitwise OR |
    Pipe,
    /// Bitwise XOR ^
    Caret,
    /// Bitwise NOT ~
    Tilde,
    /// Left shift <<
    ShiftLeft,
    /// Right shift >>
    ShiftRight,

    // Delimiters and punctuation
    /// Comma ,
    Comma,
    /// Semicolon ;
    Semicolon,
    /// Left parenthesis (
    LParen,
    /// Right parenthesis )
    RParen,
    /// Left brace {
    LBrace,
    /// Right brace }
    RBrace,
    /// Left bracket [
    LBracket,
    /// Right bracket ]
    RBracket,
    /// Dot/period .
    Fullstop,
    /// Colon :
    Colon,

    // Language keywords (Banglish)
    /// Function declaration keyword
    Function,
    /// Immutable variable declaration keyword
    Dhoro,
    /// Mutable variable declaration keyword
    Temp,
    /// Boolean true literal
    Ha,
    /// Boolean false literal
    Na,
    /// Conditional if keyword
    Jodi,
    /// Condition connector (like "is")
    Hoy,
    /// Then keyword for conditionals
    Tahole,
    /// Else keyword
    Nahoy,
    /// Logical OR keyword
    Othoba,
    /// Logical AND keyword
    Ebong,
    /// Return statement keyword
    ReturnKoro,
    /// Print/output keyword
    Dekhao,
    /// Input function keyword
    InputNao,
    /// Time keyword
    Shomoy,

    // Comment tokens for single and multi-line comments
    /// Single line comment marker
    EkLineMontobbo,
    /// Multi-line comment start marker
    BohuLineMontobboShuru,
    /// Multi-line comment end marker
    BohuLineMontobboShesh,

    // Loop-related keywords
    /// While loop keyword
    Jotokhon,
    /// Do-while loop keyword
    AgeKoro,
    /// For loop keyword
    ErJonno,
    /// For-each loop keyword
    ProtitarJonno,
    /// Continue keyword
    Choluk,
    /// Break keyword
    Thamo,
    /// Where clause keyword
    Jekhane,
    /// Iterator keyword
    Protibar,

    // Module system
    /// Import keyword
    ImportKoro,
    /// Export keyword
    ExportKoro,
    /// Module declaration keyword
    Module,
    /// Alias keyword (as)
    EiHisebe,

    // Exception handling
    /// Try keyword
    CheshtaKoro,
    /// Catch keyword
    DhoreFelo,
    /// Finally keyword
    Oboseshe,
    /// Throw keyword
    ThrowKoro,

    // Type system
    /// Type definition keyword
    TypeBanao,
    /// Typeof keyword
    Dhoroner,
    /// Null/none keyword
    Kisuna,

    // Data structures
    /// List/array keyword
    Talika,
    /// Arrow operator ->
    Arrow,
    /// Double colon ::
    DoubleColon,

    // Async programming
    /// Await keyword
    OpekkhaKoro,
    /// Async keyword
    ShomoyNiropekho,
}

impl TokenType {
    /// Returns the category of the token type.
    /// This is used for classification and parsing logic.
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
            | TokenType::Temp
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
        }
    }
}

/// Struct representing a token, consisting of type, literal, and position info.
/// This is the fundamental unit of lexical analysis.
#[derive(Debug, PartialEq, Clone)]
pub struct Token {
    /// The type of token (keyword, operator, literal, etc.)
    pub token_type: TokenType,
    /// The actual text that was tokenized
    pub literal: String,
    /// Line number in source code (1-indexed)
    pub line: usize,
    /// Column number in source code (1-indexed)
    pub column: usize,
}

impl Token {
    /// Constructor to create a new token with given type, literal, and position.
    /// 
    /// # Examples
    /// ```
    /// let token = Token::new(TokenType::Ident, "variable_name", 1, 5);
    /// ```
    pub fn new(token_type: TokenType, literal: &str, line: usize, column: usize) -> Self {
        Token {
            token_type,
            literal: literal.to_string(),
            line,
            column,
        }
    }

    /// Creates a string representation of the token, useful for debugging.
    /// 
    /// # Examples
    /// ```
    /// let token = Token::new(TokenType::Ident, "x", 1, 1);
    /// println!("{}", token.to_string()); // Outputs: Ident('x') at 1:1
    /// ```
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
            TokenType::Temp => "temp",
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
/// This allows flexible keyword recognition regardless of case or spacing variations.
/// 
/// # Examples
/// ```
/// assert_eq!(normalize_keyword("Mone  Koro"), "mone koro");
/// assert_eq!(normalize_keyword("JODI"), "jodi");
/// ```
fn normalize_keyword(ident: &str) -> String {
    ident
        .to_lowercase()
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

/// A map of keyword strings and their synonyms to the canonical TokenType.
/// This allows multiple natural language variants to map to the same token.
/// 
/// The map includes both base keywords and their various synonyms to support
/// flexible natural language programming in Bengali/Banglish.
pub static KEYWORDS: Lazy<HashMap<&'static str, TokenType>> = Lazy::new(|| {
    let mut map = HashMap::new();

    // Variable declaration keywords
    map.insert("dhoro", TokenType::Dhoro);
    map.insert("dhori", TokenType::Dhoro);
    map.insert("monekori", TokenType::Dhoro);
    map.insert("monekoro", TokenType::Dhoro);
    map.insert("mone kori", TokenType::Dhoro);
    map.insert("mone koro", TokenType::Dhoro);

    // Mutable/Changeable Variable Keywords
    map.insert("temp", TokenType::Temp);
    map.insert("temporary", TokenType::Temp);
    map.insert("changable", TokenType::Temp);
    map.insert("osthayi", TokenType::Temp);
    map.insert("mutable", TokenType::Temp);
    map.insert("mut", TokenType::Temp);

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

    // Async keywords
    map.insert("opekkha", TokenType::OpekkhaKoro);
    map.insert("opekkha koro", TokenType::OpekkhaKoro);
    map.insert("await", TokenType::OpekkhaKoro);

    map.insert("shomoy niropekkho", TokenType::ShomoyNiropekho);
    map.insert("asynchronous", TokenType::ShomoyNiropekho);
    map.insert("async", TokenType::ShomoyNiropekho);

    map
});

/// Lookup a keyword or identifier and return its token type.
/// If no keyword match is found, returns TokenType::Ident.
/// 
/// This function first checks for exact matches, then tries normalized variants
/// with lowercase and collapsed whitespace to support flexible keyword recognition.
/// 
/// # Examples
/// ```
/// assert_eq!(lookup_ident("dhoro"), TokenType::Dhoro);
/// assert_eq!(lookup_ident("Dhoro"), TokenType::Dhoro);
/// assert_eq!(lookup_ident("unknown_var"), TokenType::Ident);
/// ```
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

/// Helper: checks if token is a literal type.
/// Literals are values that can be directly represented in source code.
/// 
/// # Examples
/// ```
/// assert!(is_literal(TokenType::Int));
/// assert!(is_literal(TokenType::String));
/// assert!(!is_literal(TokenType::Plus));
/// ```
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

/// Helper: checks if token is an operator (arithmetic, comparison, or bitwise).
/// 
/// # Examples
/// ```
/// assert!(is_operator(TokenType::Plus));
/// assert!(is_operator(TokenType::Eq));
/// assert!(is_operator(TokenType::Ampersand));
/// assert!(!is_operator(TokenType::Ident));
/// ```
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

/// Helper: checks if token is a language keyword.
/// Keywords are reserved words that have special meaning in the B+ language.
/// 
/// # Examples
/// ```
/// assert!(is_keyword(TokenType::Function));
/// assert!(is_keyword(TokenType::Jodi));
/// assert!(is_keyword(TokenType::Temp));
/// assert!(!is_keyword(TokenType::Ident));
/// ```
pub fn is_keyword(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Function
            | TokenType::Dhoro
            | TokenType::Temp
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

/// Helper: checks if token is a loop control keyword.
/// 
/// # Examples
/// ```
/// assert!(is_loop(TokenType::Jotokhon));
/// assert!(is_loop(TokenType::Choluk));
/// assert!(is_loop(TokenType::Thamo));
/// assert!(!is_loop(TokenType::Function));
/// ```
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

/// Helper: checks if token is a comment token.
/// 
/// # Examples
/// ```
/// assert!(is_comment(TokenType::EkLineMontobbo));
/// assert!(is_comment(TokenType::BohuLineMontobboShuru));
/// assert!(!is_comment(TokenType::String));
/// ```
pub fn is_comment(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::EkLineMontobbo
            | TokenType::BohuLineMontobboShuru
            | TokenType::BohuLineMontobboShesh
    )
}

/// Helper: checks if token is part of the module system.
/// 
/// # Examples
/// ```
/// assert!(is_module(TokenType::ImportKoro));
/// assert!(is_module(TokenType::ExportKoro));
/// assert!(is_module(TokenType::Module));
/// assert!(!is_module(TokenType::Function));
/// ```
pub fn is_module(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::ImportKoro
            | TokenType::ExportKoro
            | TokenType::Module
            | TokenType::EiHisebe
    )
}

/// Helper: checks if token is part of exception handling.
/// 
/// # Examples
/// ```
/// assert!(is_exception_handling(TokenType::CheshtaKoro));
/// assert!(is_exception_handling(TokenType::DhoreFelo));
/// assert!(is_exception_handling(TokenType::ThrowKoro));
/// assert!(!is_exception_handling(TokenType::Function));
/// ```
pub fn is_exception_handling(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::CheshtaKoro
            | TokenType::DhoreFelo
            | TokenType::Oboseshe
            | TokenType::ThrowKoro
    )
}

/// Helper: checks if token is part of the type system.
/// 
/// # Examples
/// ```
/// assert!(is_type_system(TokenType::TypeBanao));
/// assert!(is_type_system(TokenType::Dhoroner));
/// assert!(is_type_system(TokenType::Kisuna));
/// assert!(!is_type_system(TokenType::Function));
/// ```
pub fn is_type_system(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::TypeBanao
            | TokenType::Dhoroner
            | TokenType::Kisuna
    )
}

/// Helper: checks if token is part of data structure syntax.
/// 
/// # Examples
/// ```
/// assert!(is_data_structure(TokenType::Talika));
/// assert!(is_data_structure(TokenType::Arrow));
/// assert!(is_data_structure(TokenType::DoubleColon));
/// assert!(!is_data_structure(TokenType::Function));
/// ```
pub fn is_data_structure(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Talika
            | TokenType::Arrow
            | TokenType::DoubleColon
    )
}

/// Helper: checks if token is part of async programming.
/// 
/// # Examples
/// ```
/// assert!(is_async(TokenType::OpekkhaKoro));
/// assert!(is_async(TokenType::ShomoyNiropekho));
/// assert!(!is_async(TokenType::Function));
/// ```
pub fn is_async(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::OpekkhaKoro
            | TokenType::ShomoyNiropekho
    )
}

/// Helper: checks if token is a delimiter (punctuation).
/// 
/// # Examples
/// ```
/// assert!(is_delimiter(TokenType::LParen));
/// assert!(is_delimiter(TokenType::Comma));
/// assert!(is_delimiter(TokenType::Semicolon));
/// assert!(!is_delimiter(TokenType::Plus));
/// ```
pub fn is_delimiter(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Comma
            | TokenType::Semicolon
            | TokenType::LParen
            | TokenType::RParen
            | TokenType::LBrace
            | TokenType::RBrace
            | TokenType::LBracket
            | TokenType::RBracket
            | TokenType::Fullstop
            | TokenType::Colon
    )
}

/// Helper: checks if token is a bitwise operator.
/// 
/// # Examples
/// ```
/// assert!(is_bitwise_operator(TokenType::Ampersand));
/// assert!(is_bitwise_operator(TokenType::ShiftLeft));
/// assert!(!is_bitwise_operator(TokenType::Plus));
/// ```
pub fn is_bitwise_operator(token_type: TokenType) -> bool {
    matches!(
        token_type,
        TokenType::Ampersand
            | TokenType::Pipe
            | TokenType::Caret
            | TokenType::Tilde
            | TokenType::ShiftLeft
            | TokenType::ShiftRight
    )
}

/// Helper: checks if token is reserved for future use.
/// Currently, no reserved tokens are implemented, but this function
/// is provided for future extensibility.
/// 
/// # Examples
/// ```
/// assert!(!is_reserved(TokenType::Function)); // Currently no reserved tokens
/// ```
pub fn is_reserved(_token_type: TokenType) -> bool {
    // Reserved tokens are not currently implemented
    // This function is provided for future extensibility
    // When reserved tokens are added, they should be matched here
    false
}

/// Helper: checks if a given string is a reserved keyword that cannot be used as an identifier.
/// This is used during parsing to prevent users from using language keywords as variable names.
/// 
/// # Examples
/// ```
/// assert!(is_reserved_keyword("jodi"));
/// assert!(is_reserved_keyword("function"));
/// assert!(!is_reserved_keyword("myVariable"));
/// ```
pub fn is_reserved_keyword(ident: &str) -> bool {
    let normalized = normalize_keyword(ident);
    RESERVED_KEYWORDS.contains(&normalized.as_str())
}

/// List of reserved keywords that cannot be used as variable names or identifiers.
/// These are the core language keywords that have special meaning and must be protected
/// from being used as user-defined identifiers.
/// 
/// This list includes both the canonical forms and common variants to ensure
/// comprehensive protection of language keywords.
pub static RESERVED_KEYWORDS: &[&str] = &[
    // Core language keywords
    "jodi",         // if
    "tahole",       // then
    "nahoy",        // else
    "ha",           // true
    "na",           // false
    "dhoro",        // let/variable declaration
    "temp",         // mutable variable
    "function",     // function declaration
    "kaj",          // function (synonym)
    "fn",           // function (synonym)
    "return",       // return
    "returnkoro",   // return (Banglish)
    "ferot",        // return (synonym)
    "dekhao",       // print
    "print",        // print (English)
    "inputnao",     // input
    "input",        // input (English)
    
    // Boolean literals
    "true",
    "false",
    "thik",         // true (synonym)
    "mitthe",       // false (synonym)
    "sotti",        // true (synonym)
    
    // Logical operators
    "ebong",        // and
    "and",          // and (English)
    "othoba",       // or
    "or",           // or (English)
    "ba",           // or (synonym)
    
    // Loop keywords
    "jotokhon",     // while
    "age koro",     // do
    "agekoro",      // do (no space)
    "er jonno",     // for
    "erjonno",      // for (no space)
    "protitar jonno", // for each
    "choluk",       // continue
    "thamo",        // break
    "protibar",     // each iteration
    
    // Module system
    "import",
    "import koro",
    "export",
    "export koro",
    "module",
    "as",
    "ei hisebe",    // as (Banglish)
    
    // Exception handling
    "try",
    "cheshta koro", // try (Banglish)
    "catch",
    "dhore felo",   // catch (Banglish)
    "finally",
    "oboseshe",     // finally (Banglish)
    "throw",
    "throw koro",   // throw (Banglish)
    "felo",         // throw (synonym)
    
    // Type system
    "type banao",   // type definition
    "typeof",
    "dhoroner",     // typeof (Banglish)
    "null",
    "kisuna",       // null (Banglish)
    "nil",          // null (synonym)
    "none",         // null (synonym)
    
    // Async keywords
    "async",
    "await",
    "opekkha koro", // await (Banglish)
    "shomoy niropekkho", // async (Banglish)
    
    // Time and other utilities
    "shomoy",       // time
    "time",         // time (English)
];

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_token_creation() {
        let token = Token::new(TokenType::Ident, "variable", 1, 5);
        assert_eq!(token.token_type, TokenType::Ident);
        assert_eq!(token.literal, "variable");
        assert_eq!(token.line, 1);
        assert_eq!(token.column, 5);
    }

    #[test]
    fn test_keyword_lookup() {
        assert_eq!(lookup_ident("dhoro"), TokenType::Dhoro);
        assert_eq!(lookup_ident("Dhoro"), TokenType::Dhoro);
        assert_eq!(lookup_ident("DHORO"), TokenType::Dhoro);
        assert_eq!(lookup_ident("ha"), TokenType::Ha);
        assert_eq!(lookup_ident("true"), TokenType::Ha);
        assert_eq!(lookup_ident("unknown"), TokenType::Ident);
    }

    #[test]
    fn test_normalize_keyword() {
        assert_eq!(normalize_keyword("Dhoro"), "dhoro");
        assert_eq!(normalize_keyword("JODI"), "jodi");
        assert_eq!(normalize_keyword("  Ha  "), "ha");
    }

    #[test]
    fn test_helper_functions() {
        assert!(is_literal(TokenType::Int));
        assert!(is_literal(TokenType::String));
        assert!(!is_literal(TokenType::Plus));

        assert!(is_operator(TokenType::Plus));
        assert!(is_operator(TokenType::Eq));
        assert!(!is_operator(TokenType::Ident));

        assert!(is_keyword(TokenType::Function));
        assert!(is_keyword(TokenType::Temp));
        assert!(!is_keyword(TokenType::Ident));

        assert!(is_loop(TokenType::Jotokhon));
        assert!(is_loop(TokenType::Choluk));
        assert!(!is_loop(TokenType::Function));

        assert!(is_comment(TokenType::EkLineMontobbo));
        assert!(!is_comment(TokenType::String));
    }

    #[test]
    fn test_token_categories() {
        assert_eq!(TokenType::Dhoro.category(), TokenCategory::Keyword);
        assert_eq!(TokenType::Plus.category(), TokenCategory::Operator);
        assert_eq!(TokenType::Int.category(), TokenCategory::Literal);
        assert_eq!(TokenType::Jotokhon.category(), TokenCategory::Loop);
        assert_eq!(TokenType::ImportKoro.category(), TokenCategory::Module);
    }

    #[test]
    fn test_reserved_keywords() {
        assert!(is_reserved_keyword("jodi"));
        assert!(is_reserved_keyword("function"));
        assert!(is_reserved_keyword("DHORO"));
        assert!(!is_reserved_keyword("myVariable"));
        assert!(!is_reserved_keyword("customName"));
    }

    #[test]
    fn test_display_format() {
        assert_eq!(format!("{}", TokenType::Plus), "+");
        assert_eq!(format!("{}", TokenType::Dhoro), "dhoro");
        assert_eq!(format!("{}", TokenType::Arrow), "->");
    }
}