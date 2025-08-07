// compiler/src/stdlib/file.rs

use crate::environment::Environment;
use crate::object::Object;
use std::fs;

/// Load all file-related functions into environment
pub fn load_file_functions(env: &mut Environment) {
    // Move existing functions from environment.rs
    env.add_builtin("readkoro".to_string(), Object::BuiltinNative(read_file));
    env.add_builtin("writekoro".to_string(), Object::BuiltinNative(write_file));
    
    // Add new file functions
    env.add_builtin("file_exists".to_string(), Object::BuiltinNative(file_exists));
    env.add_builtin("delete_file".to_string(), Object::BuiltinNative(delete_file));
    env.add_builtin("copy_file".to_string(), Object::BuiltinNative(copy_file));
}

/// Read file content (moved from environment.rs)
fn read_file(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("readkoro() requires exactly one argument (filename)".to_string());
    }

    match &args[0] {
        Object::String(filename) => match fs::read_to_string(filename) {
            Ok(content) => Object::String(content),
            Err(e) => Object::Error(format!("File read error: {}", e)),
        },
        _ => Object::Error("readkoro() requires a string filename".to_string()),
    }
}

/// Write content to file (moved from environment.rs)
fn write_file(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("writekoro() requires exactly two arguments (filename, content)".to_string());
    }

    match (&args[0], &args[1]) {
        (Object::String(filename), content) => {
            let content_str = format!("{}", content);
            match fs::write(filename, content_str) {
                Ok(_) => Object::Null,
                Err(e) => Object::Error(format!("File write error: {}", e)),
            }
        }
        _ => Object::Error("writekoro() requires a string filename as first argument".to_string()),
    }
}

/// Check if file exists
fn file_exists(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("file_exists() requires exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(filename) => {
            Object::Boolean(std::path::Path::new(filename).exists())
        }
        _ => Object::Error("file_exists() requires a string filename".to_string()),
    }
}

/// Delete a file
fn delete_file(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("delete_file() requires exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(filename) => {
            match fs::remove_file(filename) {
                Ok(_) => Object::Null,
                Err(e) => Object::Error(format!("Delete error: {}", e)),
            }
        }
        _ => Object::Error("delete_file() requires a string filename".to_string()),
    }
}

/// Copy file from source to destination
fn copy_file(args: Vec<Object>) -> Object {
    if args.len() != 2 {
        return Object::Error("copy_file() requires exactly two arguments (source, dest)".to_string());
    }
    
    match (&args[0], &args[1]) {
        (Object::String(source), Object::String(dest)) => {
            match fs::copy(source, dest) {
                Ok(_) => Object::Null,
                Err(e) => Object::Error(format!("Copy error: {}", e)),
            }
        }
        _ => Object::Error("copy_file() requires two string arguments".to_string()),
    }
}