// compiler/src/evaluator.rs


// Imports required modules from the project and standard library
use crate::ast::{Expression, Program, Statement};
use crate::environment::Environment;
use crate::object::{BuiltinFunction, Object};
use std::panic;

// Main evaluation function for the program (list of statements)
pub fn eval(node: Program, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    // Evaluate each statement in sequence
    for statement in node {
        result = eval_statement(statement, env);

        // Handle early returns or errors
        match &result {
            Object::ReturnValue(value) => return format_boolean(*value.clone()),
            Object::Error(_) => return result,
            _ => (),
        }
    }

    // Format and return the final result
    format_boolean(result)
}

// Evaluates a single statement
fn eval_statement(statement: Statement, env: &mut Environment) -> Object {
    match statement {
        // Evaluate expression statements
        Statement::ExpressionStatement { expression } => eval_expression(expression, env),

        // Handle variable declaration
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

        // Handle return statements
        Statement::Return { return_value } => {
            let val = eval_expression(return_value, env);
            if is_error(&val) {
                return val;
            }
            Object::ReturnValue(Box::new(val))
        }

        // Ignore comments during evaluation
        Statement::CommentSingleLine { .. } => Object::Null,
        Statement::CommentMultiLine { .. } => Object::Null,

        // Handle while loops
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

        // Handle for loops
        Statement::For { init, condition, update, body } => {
            if let Some(init_stmt) = init {
                let result = eval_statement(*init_stmt, env);
                if is_error(&result) {
                    return result;
                }
            }

            while match &condition {
                Some(cond_expr) => is_truthy(&eval_expression(cond_expr.clone(), env)),
                None => true, // If no condition, treat as infinite loop
            } {
                let result = eval_block_statement(body.clone(), env);
                match result {
                    Object::ReturnValue(_) | Object::Error(_) => return result,
                    _ => {}
                }

                // Evaluate update expression after each iteration
                if let Some(ref upd_expr) = update {
                    let result = eval_expression(upd_expr.clone(), env);
                    if is_error(&result) {
                        return result;
                    }
                }
            }

            Object::Null
        }

        // Placeholders for break/continue support
        Statement::Break => Object::Null,
        Statement::Continue => Object::Null,
    }
}

// Evaluates a block of statements
fn eval_block_statement(statements: Vec<Statement>, env: &mut Environment) -> Object {
    let mut result = Object::Null;

    for statement in statements {
        result = eval_statement(statement, env);

        // Early return on return or error
        match &result {
            Object::ReturnValue(_) | Object::Error(_) => return result,
            _ => (),
        }
    }

    result
}

// Evaluates an expression
fn eval_expression(expr: Expression, env: &mut Environment) -> Object {
    match expr {
        Expression::IntegerLiteral(value) => Object::Integer(value),
        Expression::StringLiteral(value) => Object::String(value),
        Expression::Boolean(value) => Object::Boolean(value),

        // Evaluate prefix expressions like ! or -
        Expression::Prefix { operator, right } => {
            let right = eval_expression(*right, env);
            if is_error(&right) {
                return right;
            }
            eval_prefix_expression(&operator, right)
        }

        // Evaluate infix expressions like +, -, ==, etc.
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

        // Resolve variable from environment
        Expression::Identifier(name) => match env.get(&name) {
            Some(obj) => obj,
            None => Object::Error(format!("identifier not found: {}", name)),
        },

        // If expression (conditional)
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

        // Function literal creation
        Expression::FunctionLiteral { parameters, body } => {
            Object::Function { parameters, body, env: env.clone() }
        }

        // Function call expression
            Expression::Call { function, arguments } => {
                // Evaluate the function itself
                let function_obj = eval_expression(*function.clone(), env);
                if is_error(&function_obj) {
                    return function_obj;
                }

                // Handle "dekhao" function with template literal support
                if let Expression::Identifier(ref name) = *function {
                    if name == "dekhao" {
                        let mut output = String::new();

                        // Check if first argument is a template literal
                        if let Some(Expression::TemplateLiteral { parts }) = arguments.get(0) {
                            for part in parts {
                                let val = match part {
                                    Expression::StringLiteral(s) => Object::String(s.clone()),
                                    expr => eval_expression(expr.clone(), env),
                                };
                                match val {
                                    Object::String(s) => output.push_str(&s),
                                    Object::Integer(i) => output.push_str(&i.to_string()),
                                    Object::Boolean(b) => output.push_str(if b { "Ha" } else { "Na" }),
                                    Object::Null => output.push_str("Null"),
                                    Object::Error(ref e) => return Object::Error(e.clone()),
                                    _ => output.push_str(&format!("{:?}", val)),
                                }
                            }
                            println!("{}", output);
                            return Object::Null;
                        }

                        // Fallback for regular single or multiple arguments
                        for arg in arguments {
                            let val = eval_expression(arg, env);
                            if is_error(&val) {
                                return val;
                            }
                            match val {
                                Object::String(s) => output.push_str(&s),
                                Object::Integer(i) => output.push_str(&i.to_string()),
                                Object::Boolean(b) => output.push_str(if b { "Ha" } else { "Na" }),
                                Object::Null => output.push_str("Null"),
                                Object::Error(ref e) => return Object::Error(e.clone()),
                                _ => output.push_str(&format!("{:?}", val)),
                            }
                        }
                        println!("{}", output);
                        return Object::Null;
                    }
                }

                // Regular function call fallback
                let args = eval_expressions(arguments, env);
                if args.len() == 1 && is_error(&args[0]) {
                    return args[0].clone();
                }

                apply_function(function_obj, args)
            }


    }
}

// Evaluates prefix operations like !value or -value
fn eval_prefix_expression(operator: &str, right: Object) -> Object {
    match operator {
        "!" => eval_bang_operator_expression(right),
        "-" => eval_minus_prefix_operator_expression(right),
        _ => Object::Error(format!("unknown operator: {}{:?}", operator, right)),
    }
}

// Evaluates logical NOT (!)
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

// Evaluates unary minus (-)
fn eval_minus_prefix_operator_expression(right: Object) -> Object {
    match right {
        Object::Integer(val) => Object::Integer(-val),
        _ => Object::Error(format!("unknown operator: -{:?}", right)),
    }
}

// Evaluates binary operations like +, -, ==, etc.
fn eval_infix_expression(operator: &str, left: Object, right: Object) -> Object {
    // Helper to convert strings like "Ha"/"Na" into booleans
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
            // Handle boolean comparisons
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

// Evaluates a list of expressions (arguments to a function)
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

// Applies a function (user-defined or built-in)
fn apply_function(func: Object, args: Vec<Object>) -> Object {
    match func {
        Object::BuiltinNative(builtin_fn) => {
            // Catch panic during built-in function execution
            let result = panic::catch_unwind(|| builtin_fn(args));
            match result {
                Ok(val) => val,
                Err(_) => Object::Error("panic occurred in built-in function".to_string()),
            }
        }
        Object::Function { parameters, body, env } => {
            let mut extended_env = Environment::new_enclosed(env);

            // Bind arguments to parameter names
            for (param, arg) in parameters.iter().zip(args.iter()) {
                if let Expression::Identifier(param_name) = param {
                    extended_env.set(param_name.clone(), arg.clone());
                }
            }

            // Execute the function body
            let evaluated = eval_block_statement(body, &mut extended_env);

            // Unwrap return value if needed
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

// Determines truthiness of an object
fn is_truthy(obj: &Object) -> bool {
    match obj {
        Object::Boolean(b) => *b,
        Object::Null => false,
        Object::String(ref s) if s == "Ha" => true,
        Object::String(ref s) if s == "Na" => false,
        _ => true,
    }
}

// Determines if an object is an error
fn is_error(obj: &Object) -> bool {
    matches!(obj, Object::Error(_))
}

// Converts booleans to Bangla-style "Ha"/"Na" strings
fn format_boolean(obj: Object) -> Object {
    match obj {
        Object::Boolean(true) => Object::String("Ha".to_string()),
        Object::Boolean(false) => Object::String("Na".to_string()),
        _ => obj,
    }
}
