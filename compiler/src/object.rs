// compiler/src/object.rs

// Import necessary modules and traits
use crate::ast::{Expression, Statement};
use crate::environment::Environment;
use std::fmt;
use std::io::{self, Write};

// Enum representing built-in functions available in the language
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    Dekhao,
    Input,
    Shomoy,
    Print, // Built-in function for printing output
    // Future builtins can be added here like Tarikh, FileRead, SystemInfo, etc.
}

// Enum representing all possible runtime objects in the language
#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),                  // Integer values
    Boolean(bool),                 // Boolean true or false
    String(String),                // String literals
    Null,                         // Null value
    ReturnValue(Box<Object>),     // Wrapper for return statements' values
    BuiltinFunction(BuiltinFunction),   // Builtin function variant
    BuiltinNative(fn(Vec<Object>) -> Object), // Native builtin function pointer
    Error(String),                // Error object containing error message
    Function {                   // User-defined function object
        parameters: Vec<Expression>, // Function parameters as AST expressions
        body: Vec<Statement>,         // Function body statements
        env: Environment,             // Closure environment capturing variables
    },
}

// Implement Display trait for pretty printing Objects
impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(true) => write!(f, "Ha"),   // True in Bangla
            Object::Boolean(false) => write!(f, "Na"), // False in Bangla
            Object::String(s) => write!(f, "{}", s),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Error(msg) => write!(f, "Error: {}", msg),
            Object::Function { parameters, .. } => {
                // Display function signature with parameter list
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                write!(f, "fn({}) {{ ... }}", params.join(", "))
            }
            Object::BuiltinFunction(name) => write!(f, "[builtin: {:?}]", name),
            Object::BuiltinNative(_) => write!(f, "[native builtin function]"),
        }
    }
}

impl Object {
    // Helper method to check if Object is an error type
    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}

// Builtin native function for input: reads line from stdin and returns String object
pub fn builtin_input(_args: Vec<Object>) -> Object {
    print!(">> ");
    io::stdout().flush().unwrap();

    let mut input = String::new();
    match io::stdin().read_line(&mut input) {
        Ok(_) => {
            let input = input.trim_end().to_string();
            Object::String(input)
        }
        Err(e) => Object::Error(format!("Failed to read input: {}", e)),
    }
}

// Builtin native function for print: prints all arguments separated by space
pub fn builtin_print(args: Vec<Object>) -> Object {
    let output = args.iter()
        .map(|obj| format!("{}", obj))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
    Object::Null
}

impl Object {
    // Retrieve native builtin function object by its name string
    pub fn get_builtin_native(name: &str) -> Option<Object> {
        match name {
            "input" => Some(Object::BuiltinNative(builtin_input)),
            "dekhao" => Some(Object::BuiltinNative(builtin_print)),
            _ => None,
        }
    }
}

impl BuiltinFunction {
    // Convert string name to BuiltinFunction enum variant if known
    pub fn from_name(name: &str) -> Option<BuiltinFunction> {
        match name {
            "dekhao" => Some(BuiltinFunction::Dekhao),
            "input" => Some(BuiltinFunction::Input),
            "shomoy" => Some(BuiltinFunction::Shomoy),
            "print" => Some(BuiltinFunction::Print),
            _ => None,
        }
    }
}
