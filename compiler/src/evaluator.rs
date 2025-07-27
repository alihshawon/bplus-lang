// compiler/src/evaluator.rs

use crate::ast::{Expression, Program, Statement};
use crate::environment::Environment;
use crate::object::Object;

pub fn eval(node: Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in node {
        result = eval_statement(statement, env);

        match result {
            Object::ReturnValue(value) => return *value,
            Object::Error(_) => return result,
            _ => (),
        }
    }

    result
}

fn eval_statement(statement: Statement, env: &mut Environment) -> Object {
    match statement {
        Statement::ExpressionStatement { expression } => eval_expression(expression, env),
        Statement::Let { name, value } => {
            let val = eval_expression(value, env);
            if is_error(&val) {
                return val;
            }
            if let Expression::Identifier(ident_name) = name {
                env.set(ident_name, val);
            }
            Object::Null
        }
        Statement::Return { return_value } => {
            let val = eval_expression(return_value, env);
            if is_error(&val) {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }
    }
}

fn eval_block_statement(statements: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement, env);

        // If we get a return value or an error, we need to stop execution
        // and bubble it up immediately.
        match result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => (),
        }
    }

    result
}

fn eval_expression(expr: Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(value) => Object::Integer(value),
        Expression::StringLiteral(value) => Object::String(value),
        Expression::Boolean(value) => Object::Boolean(value),
        Expression::Prefix { operator, right } => {
            let right = eval_expression(*right, env);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&operator, right)
        }
        Expression::Infix { left, operator, right } => {
            let left = eval_expression(*left, env);
            if is_error(&left) {
                return left;
            }
            let right = eval_expression(*right, env);
            if is_error(&right) {
                return right;
            }
            eval_infix_expression(&operator, left, right)
        }
        Expression::Identifier(name) => match env.get(&name) {
            Some(obj) => obj,
            None => Object::Error(format!("identifier not found: {}", name)),
        },
        Expression::If { condition, consequence, alternative } => {
            let condition_obj = eval_expression(*condition, env);
            if is_error(&condition_obj) {
                return condition_obj;
            }
            if is_truthy(&condition_obj) {
                eval_block_statement(consequence, env)
            } else if let Some(alt) = alternative {
                eval_block_statement(alt, env)
            } else {
                Object::Null
            }
        }
        Expression::FunctionLiteral { parameters, body } => {
            Object::Function { parameters, body, env: env.clone() }
        }
        Expression::Call { function, arguments } => {
            let function_obj = eval_expression(*function, env);
            if is_error(&function_obj) {
                return function_obj;
            }

            let args = eval_expressions(arguments, env);
            if args.len() == 1 && is_error(&args[0]) {
                return args[0].clone();
            }

            apply_function(function_obj, args)
        }
    }
}

fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Object::Error(format!("unknown operator: {}{:?}", operator, right)),
    }
}

fn eval_bang_operator_expression(right: Object) -> Object {
    match right {
        Object::Boolean(true) => Object::Boolean(false),
        Object::Boolean(false) => Object::Boolean(true),
        Object::Null => Object::Boolean(true),
        _ => Object::Boolean(false),
    }
}

fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        _ => Object::Error(format!("unknown operator: -{:?}", right)),
    }
}

fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    match (&left, &right) {
        (Object::Integer(l), Object::Integer(r)) => match operator {
            "+" => Object::Integer(l + r),
            "-" => Object::Integer(l - r),
            "*" => Object::Integer(l * r),
            "/" => Object::Integer(l / r),
            "<" => Object::Boolean(l < r),
            ">" => Object::Boolean(l > r),
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error(format!("unknown operator: {:?} {} {:?}", left, operator, right)),
        },
        (Object::Boolean(l), Object::Boolean(r)) => match operator {
            "==" => Object::Boolean(l == r),
            "!=" => Object::Boolean(l != r),
            _ => Object::Error(format!("unknown operator: {:?} {} {:?}", left, operator, right)),
        },
        _ => Object::Error(format!("type mismatch: {:?} {} {:?}", left, operator, right)),
    }
}

fn eval_expressions(exprs: Vec<Expression>, env: &mut Environment) -> Vec<Object> {
    let mut result = Vec::new();
    for e in exprs {
        let evaluated = eval_expression(e, env);
        if is_error(&evaluated) {
            return vec![evaluated];
        }
        result.push(evaluated);
    }
    result
}

fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::Builtin(builtin_fn) => builtin_fn(args),
        Object::Function { parameters, body, env } => {
            let mut extended_env = Environment::new_enclosed(env);

            for (param, arg) in parameters.iter().zip(args.iter()) {
                if let Expression::Identifier(param_name) = param {
                    extended_env.set(param_name.clone(), arg.clone());
                }
            }

            let evaluated = eval_block_statement(body, &mut extended_env);

            // Unwrap the return value if it exists
            if let Object::ReturnValue(value) = evaluated {
                *value
            } else {
                evaluated
            }
        }
        _ => Object::Error(format!("not a function: {:?}", func)),
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(b) => *b,
        Object::Null => false,
        _ => true, // All other types (like numbers and strings) are truthy
    }
}

fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}