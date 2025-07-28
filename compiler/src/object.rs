// compiler/src/object.rs

use crate::ast::{Expression, Statement};
use crate::environment::Environment;
use std::fmt;

#[derive(Clone, Debug, PartialEq)]
pub enum Object {
    Integer(i64),
    Boolean(bool),
    String(String),
    Null,
    ReturnValue(Box<Object>),
    Error(String),
    Function {
        parameters: Vec<Expression>, // Identifier expressions
        body: Vec<Statement>,
        env: Environment,
    },
    // Added to represent functions implemented in Rust
    Builtin(fn(Vec<Object>) -> Object),
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
            Object::Builtin(_) => write!(f, "[builtin function]"),
        }
    }
}

