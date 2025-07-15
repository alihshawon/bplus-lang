// compiler/src/object.rs

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
        parameters: Vec<String>,
        body: String, // For simplicity, body as string (later replace with AST node)
        env: Box<crate::environment::Environment>,
    },
}

impl fmt::Display for Object {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Object::Integer(i) => write!(f, "{}", i),
            Object::Boolean(b) => write!(f, "{}", b),
            Object::String(s) => write!(f, "\"{}\"", s),
            Object::Null => write!(f, "null"),
            Object::ReturnValue(obj) => write!(f, "{}", obj),
            Object::Error(msg) => write!(f, "Error: {}", msg),
            Object::Function { parameters, body, .. } => {
                write!(f, "fn({}) {{ {} }}", parameters.join(", "), body)
            }
        }
    }
}