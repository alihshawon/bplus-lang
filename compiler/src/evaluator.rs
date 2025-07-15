// compiler/src/evaluator.rs

use crate::ast::*;
use crate::environment::Environment;
use crate::object::{Object, ObjectType};

pub fn eval(node: &ASTNode, env: &mut Environment) -> Object {
    match node {
        ASTNode::Program(statements) => {
            let mut result = Object::Null;
            for stmt in statements {
                result = eval(stmt, env);
                if let Object::ReturnValue(val) = result {
                    return *val;
                }
            }
            result
        }

        ASTNode::ExpressionStatement(expr) => eval(expr, env),

        ASTNode::IntegerLiteral(value) => Object::Integer(*value),

        ASTNode::BooleanLiteral(value) => Object::Boolean(*value),

        ASTNode::PrefixExpression { operator, right } => {
            let right = eval(right, env);
            eval_prefix_expression(operator, right)
        }

        ASTNode::InfixExpression { left, operator, right } => {
            let left = eval(left, env);
            let right = eval(right, env);
            eval_infix_expression(operator, left, right)
        }

        ASTNode::LetStatement { name, value } => {
            let val = eval(value, env);
            env.set(name.clone(), val.clone());
            val
        }

        ASTNode::Identifier(name) => {
            env.get(name).unwrap_or_else(|| Object::Error(format!("identifier not found: {}", name)))
        }

        ASTNode::IfExpression { condition, consequence, alternative } => {
            let condition = eval(condition, env);
            if is_truthy(&condition) {
                eval(consequence, env)
            } else if let Some(alt) = alternative {
                eval(alt, env)
            } else {
                Object::Null
            }
        }

        ASTNode::BlockStatement(statements) => {
            let mut result = Object::Null;
            for stmt in statements {
                result = eval(stmt, env);
                if let Object::ReturnValue(_) = result {
                    return result;
                }
            }
            result
        }

        ASTNode::ReturnStatement(expr) => {
            let val = eval(expr, env);
            Object::ReturnValue(Box::new(val))
        }

        _ => Object::Null,
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => match right {
            Object::Boolean(true) => Object::Boolean(false),
            Object::Boolean(false) => Object::Boolean(true),
            Object::Null => Object::Boolean(true),
            _ => Object::Boolean(false),
        },
        "-" => match right {
            Object::Integer(val) => Object::Integer(-val),
            _ => Object::Error("unknown operator: -".to_string()),
        },
        _ => Object::Error(format!("unknown operator: {}", operator)),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (left, right) {
        (Object::Integer(l), Object::Integer(r)) => match operator {
            "+" => Object::Integer(l + r),
            "-" => Object::Integer(l - r),
            "*" => Object::Integer(l * r),
            "/" => Object::Integer(l / r),
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            "<" => Object::Boolean(l < r),
            ">" => Object::Boolean(l > r),
            _ => Object::Error(format!("unknown operator: {}", operator)),
        },
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error(format!("unknown operator: {}", operator)),
        },
        _ => Object::Error("type mismatch in infix expression".to_string()),
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(b) => *b,
        Object::Null => false,
        _ => true,
    }
}