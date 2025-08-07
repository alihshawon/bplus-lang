// compiler/src/stdlib/string.rs

use crate::environment::Environment;
use crate::object::Object;

/// Load all string manipulation functions into environment
pub fn load_string_functions(env: &mut Environment) {
    env.add_builtin("str_length".to_string(), Object::BuiltinNative(string_length));
    env.add_builtin("str_len".to_string(), Object::BuiltinNative(string_length));
    env.add_builtin("length".to_string(), Object::BuiltinNative(string_length));
    
    env.add_builtin("str_upper".to_string(), Object::BuiltinNative(string_upper));
    env.add_builtin("str_lower".to_string(), Object::BuiltinNative(string_lower));
    env.add_builtin("upper".to_string(), Object::BuiltinNative(string_upper));
    env.add_builtin("lower".to_string(), Object::BuiltinNative(string_lower));
    
    env.add_builtin("str_contains".to_string(), Object::BuiltinNative(string_contains));
    env.add_builtin("contains".to_string(), Object::BuiltinNative(string_contains));
    
    env.add_builtin("str_split".to_string(), Object::BuiltinNative(string_split));
    env.add_builtin("split".to_string(), Object::BuiltinNative(string_split));
    
    env.add_builtin("str_trim".to_string(), Object::BuiltinNative(string_trim));
    env.add_builtin("trim".to_string(), Object::BuiltinNative(string_trim));
    
    env.add_builtin("str_replace".to_string(), Object::BuiltinNative(string_replace));
    env.add_builtin("replace".to_string(), Object::BuiltinNative(string_replace));
    
    // Bangla variants
    env.add_builtin("lambai".to_string(), Object::BuiltinNative(string_length));  // length in Bangla
    env.add_builtin("boro".to_string(), Object::BuiltinNative(string_upper));     // upper in Bangla
    env.add_builtin("choto".to_string(), Object::BuiltinNative(string_lower));    // lower in Bangla
}

/// Get string length
fn string_length(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("str_length() takes exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(s) => Object::Integer(s.len() as i64),
        _ => Object::Error("str_length() requires a string argument".to_string()),
    }
}

/// Convert string to uppercase
fn string_upper(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("str_upper() takes exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(s) => Object::String(s.to_uppercase()),
        _ => Object::Error("str_upper() requires a string argument".to_string()),
    }
}

/// Convert string to lowercase
fn string_lower(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("str_lower() takes exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(s) => Object::String(s.to_lowercase()),
        _ => Object::Error("str_lower() requires a string argument".to_string()),
    }
}

/// Check if string contains substring
fn string_contains(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("str_contains() takes exactly two arguments".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Object::String(haystack), Object::String(needle)) => {
            Object::Boolean(haystack.contains(needle))
        }
        _ => Object::Error("str_contains() requires two string arguments".to_string()),
    }
}

/// Split string by delimiter
fn string_split(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("str_split() takes exactly two arguments".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Object::String(text), Object::String(delimiter)) => {
            let parts: Vec<Object> = text
                .split(delimiter)
                .map(|s| Object::String(s.to_string()))
                .collect();
            Object::Array(parts)
        }
        _ => Object::Error("str_split() requires two string arguments".to_string()),
    }
}

/// Trim whitespace from string
fn string_trim(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("str_trim() takes exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(s) => Object::String(s.trim().to_string()),
        _ => Object::Error("str_trim() requires a string argument".to_string()),
    }
}

/// Replace substring in string
fn string_replace(args: Vec<Object>) -> Object {
    if args.len() != 3 {
        return Object::Error("str_replace() takes exactly three arguments (string, old, new)".to_string());
    }
    
    match (&args[0], &args[1], &args[2]) {
        (Object::String(text), Object::String(old), Object::String(new)) => {
            Object::String(text.replace(old, new))
        }
        _ => Object::Error("str_replace() requires three string arguments".to_string()),
    }
}