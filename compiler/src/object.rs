// compiler/src/object.rs

use crate::ast::{Expression, Statement};
use crate::environment::Environment;
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum BuiltinFunction {
    Dekhao,
    Input,
    Shomoy,
    // Future: Tarikh, FileRead, SystemInfo, etc.
}

#[derive(Debug, PartialEq, Clone)]

pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Box<Object>),
    BuiltinFunction(BuiltinFunction),          // Named builtin function
    BuiltinNative(fn(Vec<Object>) -> Object),  // Native Rust function pointer
    Error(String),
    Function {
        parameters: Vec<Expression>, // Identifier expressions
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

impl BuiltinFunction {
    pub fn from_name(name: &str) -> Option<BuiltinFunction> {
        match name {
            "dekhao" => Some(BuiltinFunction::Dekhao),
            "input" => Some(BuiltinFunction::Input),
            "shomoy" => Some(BuiltinFunction::Shomoy),
            _ => None,
        }
    }
}
