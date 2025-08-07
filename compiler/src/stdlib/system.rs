// compiler/src/stdlib/system.rs

use crate::environment::Environment;
use crate::object::Object;

/// Load all system-related functions into environment
pub fn load_system_functions(env: &mut Environment) {
    // Move existing functions from environment.rs
    env.add_builtin("exitkoro".to_string(), Object::BuiltinNative(exit_program));
    env.add_builtin("shuru_koro".to_string(), Object::BuiltinNative(restart_message));
    env.add_builtin("bondho_koro".to_string(), Object::BuiltinNative(shutdown_message));
    
    // Add new system functions
    env.add_builtin("platform".to_string(), Object::BuiltinNative(get_platform));
    env.add_builtin("env_var".to_string(), Object::BuiltinNative(get_env_var));
}

/// Exit program with code (moved from environment.rs)
fn exit_program(args: Vec<Object>) -> Object {
    let exit_code = if !args.is_empty() {
        match &args[0] {
            Object::Integer(code) => *code as i32,
            _ => 0,
        }
    } else {
        0
    };

    println!("Program theke exit kora hosse!");
    std::process::exit(exit_code);
}

/// Print restart message (moved from environment.rs)
fn restart_message(_args: Vec<Object>) -> Object {
    println!("প্রোগ্রাম পুনরায় শুরু হচ্ছে...");
    Object::Null
}

/// Print shutdown message (moved from environment.rs)
fn shutdown_message(_args: Vec<Object>) -> Object {
    println!("Program bondho kora holo. Dhonnobad!");
    Object::Null
}

/// Get current platform info
fn get_platform(_args: Vec<Object>) -> Object {
    let platform = if cfg!(target_os = "windows") {
        "Windows"
    } else if cfg!(target_os = "macos") {
        "macOS"
    } else if cfg!(target_os = "linux") {
        "Linux"
    } else {
        "Unknown"
    };
    
    Object::String(platform.to_string())
}

/// Get environment variable
fn get_env_var(args: Vec<Object>) -> Object {
    if args.len() != 1 {
        return Object::Error("env_var() requires exactly one argument".to_string());
    }
    
    match &args[0] {
        Object::String(var_name) => {
            match std::env::var(var_name) {
                Ok(value) => Object::String(value),
                Err(_) => Object::Null,
            }
        }
        _ => Object::Error("env_var() requires a string argument".to_string()),
    }
}