// src/compiler/ast.rs

#[derive(Debug, Clone)]
pub enum Expr {
    Integer(i64),
    Float(f64),
    Boolean(bool),
    String(String),
    Identifier(String),
    Binary {
        left: Box<Expr>,
        operator: String,
        right: Box<Expr>,
    },
    Unary {
        operator: String,
        operand: Box<Expr>,
    },
    Call {
        function: Box<Expr>,
        arguments: Vec<Expr>,
    },
    Assignment {
        name: String,
        value: Box<Expr>,
    },
    Block(Vec<Expr>),
}