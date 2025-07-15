// compiler/src/type_checker.rs

use crate::parser::Expr;

#[derive(Debug, Clone, PartialEq)]
pub enum Type {
    Int,
    Void,
    Unknown,
}

pub struct TypeChecker;

impl TypeChecker {
    pub fn check(expr: &Expr) -> Type {
        match expr {
            Expr::Number(_) => Type::Int,
            Expr::Identifier(_) => {
                // For now, assume all identifiers are int
                Type::Int
            }
            Expr::BinaryOp { op: _, left, right } => {
                let left_type = TypeChecker::check(left);
                let right_type = TypeChecker::check(right);
                if left_type == Type::Int && right_type == Type::Int {
                    Type::Int
                } else {
                    Type::Unknown
                }
            }
        }
    }
}