// compiler/src/object.rs

use crate::ast::{Expression, Statement};
use crate::environment::Environment;
use std::fmt;
use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    Dekhao,
    Input,
    Shomoy,
    Print, // Added new builtin function variant for printing
    // Future: Tarikh, FileRead, SystemInfo, etc.
}

#[derive(Debug, PartialEq, Clone)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Box<Object>),
    BuiltinFunction(BuiltinFunction),
    BuiltinNative(fn(Vec<Object>) -> Object),
    Error(String),
    Function {
        parameters: Vec<Expression>,
        body: Vec<Statement>,
        env: Environment,
    },
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(true) => write!(f, "Ha"),
            Object::Boolean(false) => write!(f, "Na"),
            Object::String(s) => write!(f, "{}", s),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Error(msg) => write!(f, "Error: {}", msg),
            Object::Function { parameters, .. } => {
                let params: Vec<String> = parameters.iter().map(|p| format!("{}", p)).collect();
                write!(f, "fn({}) {{ ... }}", params.join(", "))
            }
            Object::BuiltinFunction(name) => write!(f, "[builtin: {:?}]", name),
            Object::BuiltinNative(_) => write!(f, "[native builtin function]"),
        }
    }
}

impl Object {
    pub fn is_error(&self) -> bool {
        matches!(self, Object::Error(_))
    }
}

// Existing builtin_input function
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

// New builtin_print function: prints all arguments separated by space
pub fn builtin_print(args: Vec<Object>) -> Object {
    let output = args.iter()
        .map(|obj| format!("{}", obj))
        .collect::<Vec<String>>()
        .join(" ");
    println!("{}", output);
    Object::Null
}

impl Object {
    pub fn get_builtin_native(name: &str) -> Option<Object> {
        match name {
            "input" => Some(Object::BuiltinNative(builtin_input)),
            "dekhao" => Some(Object::BuiltinNative(builtin_print)),
            _ => None,
        }
    }
}

impl BuiltinFunction {
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
