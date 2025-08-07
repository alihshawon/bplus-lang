// compiler/src/stdlib/math.rs

use crate::environment::Environment;
use crate::object::Object;

/// Load all math functions into environment
pub fn load_math_functions(env: &mut Environment) {
    env.add_builtin("sqrt".to_string(), Object::BuiltinNative(sqrt_function));
    env.add_builtin("abs".to_string(), Object::BuiltinNative(abs_function));
    env.add_builtin("pow".to_string(), Object::BuiltinNative(pow_function));
    env.add_builtin("min".to_string(), Object::BuiltinNative(min_function));
    env.add_builtin("max".to_string(), Object::BuiltinNative(max_function));
    env.add_builtin("random".to_string(), Object::BuiltinNative(random_function));
}

/// Square root function
fn sqrt_function(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("sqrt() takes exactly one argument".to_string());
    }
    match &args[0] {
        Object::Integer(n) => {
            if *n < 0 {
                Object::Error("Cannot take square root of negative number".to_string())
            } else {
                Object::Integer((*n as f64).sqrt() as i64)
            }
        }
        _ => Object::Error("sqrt() requires a number".to_string()),
    }
}

/// Absolute value function
fn abs_function(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("abs() takes exactly one argument".to_string());
    }
    match &args[0] {
        Object::Integer(n) => Object::Integer(n.abs()),
        _ => Object::Error("abs() requires a number".to_string()),
    }
}

/// Power function (base^exponent)
fn pow_function(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("pow() takes exactly two arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Object::Integer(base), Object::Integer(exp)) => {
            if *exp < 0 {
                Object::Error("Negative exponents not supported yet".to_string())
            } else {
                Object::Integer(base.pow(*exp as u32))
            }
        }
        _ => Object::Error("pow() requires two numbers".to_string()),
    }
}

/// Minimum of two numbers
fn min_function(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("min() takes exactly two arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Object::Integer(a), Object::Integer(b)) => Object::Integer(*a.min(b)),
        _ => Object::Error("min() requires two numbers".to_string()),
    }
}

/// Maximum of two numbers  
fn max_function(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("max() takes exactly two arguments".to_string());
    }
    match (&args[0], &args[1]) {
        (Object::Integer(a), Object::Integer(b)) => Object::Integer(*a.max(b)),
        _ => Object::Error("max() requires two numbers".to_string()),
    }
}

/// Random number generator
fn random_function(_args: Vec<Object>) -> Object {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};
    
    // Simple random number generator
    let mut hasher = DefaultHasher::new();
    SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos().hash(&mut hasher);
    let random_value = (hasher.finish() % 100) as i64; // 0-99
    Object::Integer(random_value)
}