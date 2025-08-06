// compiler/src/evaluator.rs

use crate::ast::{Expression, Program, Statement};
use crate::environment::Environment;
use crate::object::{BuiltinFunction, Object};
use std::panic;

pub fn eval(node: Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in node {
        result = eval_statement(statement, env);

        match &result {
            Object::ReturnValue(value) => return format_boolean(*value.clone()),
            Object::Error(_) => return result,
            _ => (),
        }
    }

    format_boolean(result)
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

        Statement::CommentSingleLine { .. } => Object::Null,

        Statement::CommentMultiLine { .. } => Object::Null,

        // Added arms:

        Statement::While { condition, body } => {
            while is_truthy(&eval_expression(condition.clone(), env)) {
                let result = eval_block_statement(body.clone(), env);
                match result {
                    Object::ReturnValue(_) | Object::Error(_) => return result,
                    _ => {}
                }
            }
            Object::Null
        }

        Statement::For { init, condition, update, body } => {
            if let Some(init_stmt) = init {
                let result = eval_statement(*init_stmt, env);
                if is_error(&result) {
                    return result;
                }
            }

            while match &condition {
                Some(cond_expr) => is_truthy(&eval_expression(cond_expr.clone(), env)),
                None => true,
            } {
                let result = eval_block_statement(body.clone(), env);
                match result {
                    Object::ReturnValue(_) | Object::Error(_) => return result,
                    _ => {}
                }

                if let Some(ref upd_expr) = update {
                    let result = eval_expression(upd_expr.clone(), env);
                    if is_error(&result) {
                        return result;
                    }
                }
            }

            Object::Null
        }


        Statement::Break => {
            // TODO: Implement break logic properly later
            Object::Null
        }

        Statement::Continue => {
            // TODO: Implement continue logic properly later
            Object::Null
        }
    }
}


fn eval_block_statement(statements: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement, env);

        match &result {
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
            } else if let Some(alt_expr) = alternative {
                eval_expression(*alt_expr, env)
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

            // Convert named builtin functions to native implementations
            let function_obj = match &function_obj {
                Object::BuiltinFunction(bf) => {
                    match bf {
                        BuiltinFunction::Input => {
                            Object::BuiltinNative(crate::object::builtin_input)
                        },
                        BuiltinFunction::Print => {
                            Object::BuiltinNative(crate::object::builtin_print)
                        },
                        _ => function_obj.clone(),
                    }
                }
                _ => function_obj.clone(),
            };

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
        Object::String(ref s) if s == "Ha" => Object::Boolean(false),
        Object::String(ref s) if s == "Na" => Object::Boolean(true),
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
    fn to_bool(obj: &Object) -> Option<bool> {
        match obj {
            Object::Boolean(b) => Some(*b),
            Object::String(s) if s == "Ha" => Some(true),
            Object::String(s) if s == "Na" => Some(false),
            _ => None,
        }
    }

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
        (Object::String(l), Object::String(r)) => {
            if operator == "+" {
                Object::String(format!("{}{}", l, r))
            } else {
                Object::Error(format!("unknown operator for strings: {}", operator))
            }
        }
        _ => {
            if let (Some(lb), Some(rb)) = (to_bool(&left), to_bool(&right)) {
                match operator {
                    "==" => Object::Boolean(lb == rb),
                    "!=" => Object::Boolean(lb != rb),
                    _ => Object::Error(format!("unknown operator: {:?} {} {:?}", left, operator, right)),
                }
            } else {
                Object::Error(format!("type mismatch: {:?} {} {:?}", left, operator, right))
            }
        }
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
        Object::BuiltinNative(builtin_fn) => {
            let result = panic::catch_unwind(|| builtin_fn(args));
            match result {
                Ok(val) => val,
                Err(_) => Object::Error("panic occurred in built-in function".to_string()),
            }
        }
        Object::Function { parameters, body, env } => {
            let mut extended_env = Environment::new_enclosed(env);

            for (param, arg) in parameters.iter().zip(args.iter()) {
                if let Expression::Identifier(param_name) = param {
                    extended_env.set(param_name.clone(), arg.clone());
                }
            }

            let evaluated = eval_block_statement(body, &mut extended_env);

            if let Object::ReturnValue(value) = evaluated {
                *value
            } else {
                evaluated
            }
        }
        _ => {
            eprintln!("TypeError: tried to call a non-function object: {:?}", func);
            Object::Error(format!("not a function: {:?}", func))
        }
    }
}

fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(b) => *b,
        Object::Null => false,
        Object::String(ref s) if s == "Ha" => true,
        Object::String(ref s) if s == "Na" => false,
        _ => true,
    }
}

fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}

fn format_boolean(obj: Object) -> Object {
    match obj {
        Object::Boolean(true) => Object::String("Ha".to_string()),
        Object::Boolean(false) => Object::String("Na".to_string()),
        _ => obj,
    }
}