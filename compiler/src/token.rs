// compiler/src/token.rs

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum Token {
    // Single-character tokens.
    Plus,
    Minus,
    Star,
    Slash,
    Percent,
    Bang,
    Equals,
    LessThan,
    GreaterThan,
    Ampersand,
    Pipe,
    Caret,
    Tilde,
    Dot,
    Comma,
    Colon,
    Semicolon,
    Question,
    Hash,
    At,
    Arrow,       // ->
    FatArrow,    // =>
    DoubleColon, // ::
    DoubleEquals,
    NotEquals,
    LessEquals,
    GreaterEquals,
    AndAnd,
    OrOr,
    PlusPlus,
    MinusMinus,

    // Grouping symbols.
    LParen,
    RParen,
    LBrace,
    RBrace,
    LBracket,
    RBracket,

    // Literals.
    Ident(String),
    Int(i64),
    Float(f64),
    String(String),
    Char(char),
    Bool(bool),

    // Keywords.
    Let,
    Const,
    Fn,
    If,
    Else,
    While,
    For,
    In,
    Match,
    Return,
    Break,
    Continue,
    True,
    False,
    Null,
    Import,
    Export,
    Struct,
    Enum,
    Type,
    Interface,
    Impl,
    This,
    Super,
    Self_,
    New,

    // Special.
    Comment(String),
    Whitespace,
    EOF,
    Illegal(char),
}

impl Token {
    pub fn is_literal(&self) -> bool {
        matches!(
            self,
            Token::Int(_) | Token::Float(_) | Token::String(_) | Token::Char(_) | Token::Bool(_)
        )
    }

    pub fn is_operator(&self) -> bool {
        matches!(
            self,
            Token::Plus
                | Token::Minus
                | Token::Star
                | Token::Slash
                | Token::Percent
                | Token::Bang
                | Token::Equals
                | Token::LessThan
                | Token::GreaterThan
                | Token::Ampersand
                | Token::Pipe
                | Token::Caret
                | Token::Tilde
                | Token::DoubleEquals
                | Token::NotEquals
                | Token::LessEquals
                | Token::GreaterEquals
                | Token::AndAnd
                | Token::OrOr
        )
    }

    pub fn is_keyword(s: &str) -> Option<Token> {
        Some(match s {
            "let" => Token::Let,
            "const" => Token::Const,
            "fn" => Token::Fn,
            "if" => Token::If,
            "else" => Token::Else,
            "while" => Token::While,
            "for" => Token::For,
            "in" => Token::In,
            "match" => Token::Match,
            "return" => Token::Return,
            "break" => Token::Break,
            "continue" => Token::Continue,
            "true" => Token::Bool(true),
            "false" => Token::Bool(false),
            "null" => Token::Null,
            "import" => Token::Import,
            "export" => Token::Export,
            "struct" => Token::Struct,
            "enum" => Token::Enum,
            "type" => Token::Type,
            "interface" => Token::Interface,
            "impl" => Token::Impl,
            "this" => Token::This,
            "super" => Token::Super,
            "self" => Token::Self_,
            "new" => Token::New,
            _ => return None,
        })
    }
}