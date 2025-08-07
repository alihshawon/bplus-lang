// compiler/src/error.rs

use std::fmt;
use std::collections::HashMap;

/// Represents the position (line and column) of an error in the source code.
/// Optionally includes the file name.
#[derive(Debug, Clone, PartialEq)]
pub struct ErrorPosition {
    pub line: usize,
    pub column: usize,
    pub file: Option<String>,
}

impl ErrorPosition {
    /// Create a new error position without file info.
    pub fn new(line: usize, column: usize) -> Self {
        ErrorPosition { line, column, file: None }
    }

    /// Create a new error position including file name.
    pub fn with_file(line: usize, column: usize, file: String) -> Self {
        ErrorPosition { line, column, file: Some(file) }
    }
}

/// Enum representing different types of errors that can occur.
#[derive(Debug, Clone, PartialEq)]
pub enum ErrorType {
    // Lexer errors
    UnexpectedCharacter(char),
    UnterminatedString,
    UnterminatedComment,
    InvalidNumber(String),

    // Parser errors
    UnexpectedToken(String, String), // got, expected
    MissingToken(String),            // expected token
    InvalidExpression(String),
    InvalidStatement(String),

    // Type errors
    TypeMismatch(String, String),   // expected, got
    UndefinedVariable(String),
    UndefinedFunction(String),
    WrongArgumentCount(usize, usize), // expected, got

    // Runtime errors
    DivisionByZero,
    IndexOutOfBounds(i64, usize), // index, length
    FileNotFound(String),
    PermissionDenied(String),
    NetworkError(String),

    // System errors
    OutOfMemory,
    StackOverflow,
    InternalError(String),
}

/// Struct holding complete error information including type, position, and optional custom message.
#[derive(Debug, Clone)]
pub struct BPlusError {
    pub error_type: ErrorType,
    pub position: Option<ErrorPosition>,
    pub message: Option<String>, // Optional custom error message
}

impl BPlusError {
    /// Create a basic error with no position or custom message.
    pub fn new(error_type: ErrorType) -> Self {
        BPlusError {
            error_type,
            position: None,
            message: None,
        }
    }

    /// Create an error with position.
    pub fn with_position(error_type: ErrorType, position: ErrorPosition) -> Self {
        BPlusError {
            error_type,
            position: Some(position),
            message: None,
        }
    }

    /// Create an error with custom message.
    pub fn with_message(error_type: ErrorType, message: String) -> Self {
        BPlusError {
            error_type,
            position: None,
            message: Some(message),
        }
    }
}

/// Error message templates used to generate user-facing error strings.
/// Default language is Banglish (phonetic Bengali).
#[derive(Debug, Clone)]
pub struct ErrorMessages {
    templates: HashMap<String, String>,
    language: String,
}

impl ErrorMessages {
    /// Creates default Banglish (phonetic Bengali) error templates.
    pub fn new_default_banglish() -> Self {
        let mut templates = HashMap::new();

        // Lexer error templates
        templates.insert("unexpected_character".to_string(), "Aporjashito character '{0}' pawa geche".to_string());
        templates.insert("unterminated_string".to_string(), "String shesh hoy nai - quote chinho (') ba (\") onuposthit".to_string());
        templates.insert("unterminated_comment".to_string(), "Comment shesh hoy nai - bondho korar chinho onuposthit".to_string());
        templates.insert("invalid_number".to_string(), "Bhul number '{0}' - thik number likhun".to_string());

        // Parser error templates
        templates.insert("unexpected_token".to_string(), "Protjashito chilo '{1}' kintu pawa gelo '{0}'".to_string());
        templates.insert("missing_token".to_string(), "Onuposthit token '{0}' - doya kore jog korun".to_string());
        templates.insert("invalid_expression".to_string(), "Bhul expression: {0}".to_string());
        templates.insert("invalid_statement".to_string(), "Bhul statement: {0}".to_string());

        // Type error templates
        templates.insert("type_mismatch".to_string(), "Data type mile na - protjashito '{0}' kintu pawa gelo '{1}'".to_string());
        templates.insert("undefined_variable".to_string(), "Ojana variable '{0}' - prothome ghoshona korun".to_string());
        templates.insert("undefined_function".to_string(), "Ojana function '{0}' - thik naam likhun".to_string());
        templates.insert("wrong_argument_count".to_string(), "Bhul argument sonkha - proyojon {0}ti, dewa hoyeche {1}ti".to_string());

        // Runtime error templates
        templates.insert("division_by_zero".to_string(), "Shunno diye bhag kora jay na".to_string());
        templates.insert("index_out_of_bounds".to_string(), "Index {0} shimaar baire (shorbochho {1})".to_string());
        templates.insert("file_not_found".to_string(), "File '{0}' pawa jay ni".to_string());
        templates.insert("permission_denied".to_string(), "'{0}' e probesh er onumoti nei".to_string());
        templates.insert("network_error".to_string(), "Network truti: {0}".to_string());

        // System error templates
        templates.insert("out_of_memory".to_string(), "Memory shesh hoye geche".to_string());
        templates.insert("stack_overflow".to_string(), "Stack overflow - odhik recursive call".to_string());
        templates.insert("internal_error".to_string(), "Antoronio truti: {0}".to_string());

        ErrorMessages {
            templates,
            language: "banglish".to_string(),
        }
    }

    /// Returns the formatted message string for the given error type.
    pub fn get_message(&self, error_type: &ErrorType) -> String {
        let template_key = match error_type {
            ErrorType::UnexpectedCharacter(ch) => {
                return self.format_message("unexpected_character", &[&ch.to_string()]);
            }
            ErrorType::UnterminatedString => "unterminated_string",
            ErrorType::UnterminatedComment => "unterminated_comment",
            ErrorType::InvalidNumber(num) => {
                return self.format_message("invalid_number", &[num]);
            }
            ErrorType::UnexpectedToken(got, expected) => {
                return self.format_message("unexpected_token", &[got, expected]);
            }
            ErrorType::MissingToken(expected) => {
                return self.format_message("missing_token", &[expected]);
            }
            ErrorType::InvalidExpression(expr) => {
                return self.format_message("invalid_expression", &[expr]);
            }
            ErrorType::InvalidStatement(stmt) => {
                return self.format_message("invalid_statement", &[stmt]);
            }
            ErrorType::TypeMismatch(expected, got) => {
                return self.format_message("type_mismatch", &[expected, got]);
            }
            ErrorType::UndefinedVariable(name) => {
                return self.format_message("undefined_variable", &[name]);
            }
            ErrorType::UndefinedFunction(name) => {
                return self.format_message("undefined_function", &[name]);
            }
            ErrorType::WrongArgumentCount(expected, got) => {
                return self.format_message("wrong_argument_count", &[&expected.to_string(), &got.to_string()]);
            }
            ErrorType::DivisionByZero => "division_by_zero",
            ErrorType::IndexOutOfBounds(index, len) => {
                return self.format_message("index_out_of_bounds", &[&index.to_string(), &len.to_string()]);
            }
            ErrorType::FileNotFound(file) => {
                return self.format_message("file_not_found", &[file]);
            }
            ErrorType::PermissionDenied(resource) => {
                return self.format_message("permission_denied", &[resource]);
            }
            ErrorType::NetworkError(msg) => {
                return self.format_message("network_error", &[msg]);
            }
            ErrorType::OutOfMemory => "out_of_memory",
            ErrorType::StackOverflow => "stack_overflow",
            ErrorType::InternalError(msg) => {
                return self.format_message("internal_error", &[msg]);
            }
        };

        self.templates
            .get(template_key)
            .cloned()
            .unwrap_or_else(|| format!("Ojana error: {:?}", error_type))
    }

    /// Formats the message string using the provided arguments.
    fn format_message(&self, template_key: &str, args: &[&str]) -> String {
        if let Some(template) = self.templates.get(template_key) {
            let mut result = template.clone();
            for (i, arg) in args.iter().enumerate() {
                result = result.replace(&format!("{{{}}}", i), arg);
            }
            result
        } else {
            format!("Template '{}' pawa jay nai", template_key)
        }
    }

    /// Creates an ErrorMessages instance from a given language pack.
    pub fn from_language_pack(language_pack: &LanguagePack) -> Self {
        ErrorMessages {
            templates: language_pack.error_templates.clone(),
            language: language_pack.language.clone(),
        }
    }
}

/// Represents a language pack that overrides default keywords and error messages.
#[derive(Debug, Clone)]
pub struct LanguagePack {
    pub language: String,
    pub version: String,
    pub author: String,
    pub keyword_mappings: HashMap<String, String>,
    pub error_templates: HashMap<String, String>,
}

/// Enables `BPlusError` to be printed using `println!` or `eprintln!`.
impl fmt::Display for BPlusError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Error: {:?}", self.error_type)
    }
}

/// Manages error formatting, printing, and switching between language packs.
pub struct ErrorManager {
    error_messages: ErrorMessages,
    show_position: bool,
    using_language_pack: bool,
}

impl ErrorManager {
    /// Creates a new error manager with default Banglish error messages.
    pub fn new() -> Self {
        ErrorManager {
            error_messages: ErrorMessages::new_default_banglish(),
            show_position: true,
            using_language_pack: false,
        }
    }

    /// Creates an error manager using a language pack.
    pub fn with_language_pack(language_pack: &LanguagePack) -> Self {
        ErrorManager {
            error_messages: ErrorMessages::from_language_pack(language_pack),
            show_position: true,
            using_language_pack: true,
        }
    }

    /// Formats a `BPlusError` into a user-friendly string.
    pub fn format_error(&self, error: &BPlusError) -> String {
        let message = if let Some(ref custom_msg) = error.message {
            custom_msg.clone()
        } else {
            self.error_messages.get_message(&error.error_type)
        };

        if self.show_position && error.position.is_some() {
            let pos = error.position.as_ref().unwrap();
            if let Some(ref file) = pos.file {
                format!("{}:{}:{}: {}", file, pos.line, pos.column, message)
            } else {
                format!("{}:{}: {}", pos.line, pos.column, message)
            }
        } else {
            message
        }
    }

    /// Prints a formatted error to standard error.
    pub fn print_error(&self, error: &BPlusError) {
        eprintln!("{}", self.format_error(error));
    }

    /// Switches to a new language pack.
    pub fn set_language_pack(&mut self, language_pack: &LanguagePack) {
        self.error_messages = ErrorMessages::from_language_pack(language_pack);
        self.using_language_pack = true;
    }

    /// Resets to the default Banglish language pack.
    pub fn reset_to_default(&mut self) {
        self.error_messages = ErrorMessages::new_default_banglish();
        self.using_language_pack = false;
    }

    /// Checks if a custom language pack is currently being used.
    pub fn is_using_language_pack(&self) -> bool {
        self.using_language_pack
    }

    /// Returns the current language identifier (e.g., "banglish", "english").
    pub fn get_current_language(&self) -> &str {
        &self.error_messages.language
    }
}

/// Type alias for results returned by the B+ compiler.
/// This encapsulates both successful results and errors.
pub type BPlusResult<T> = Result<T, BPlusError>;
